##
## This file is part of the libsigrokdecode project.
##
## Copyright (C) 2014 Torsten Duwe <duwe@suse.de>
## Copyright (C) 2014 Sebastien Bourdelin <sebastien.bourdelin@savoirfairelinux.com>
##
## This program is free software; you can redistribute it and/or modify
## it under the terms of the GNU General Public License as published by
## the Free Software Foundation; either version 2 of the License, or
## (at your option) any later version.
##
## This program is distributed in the hope that it will be useful,
## but WITHOUT ANY WARRANTY; without even the implied warranty of
## MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
## GNU General Public License for more details.
##
## You should have received a copy of the GNU General Public License
## along with this program; if not, see <http://www.gnu.org/licenses/>.
##
import numpy as np
from numpy import packbits
import sigrokdecode as srd
from collections import deque

MAX_CHANNELS = 8

class DecoderError(Exception):
    pass

def normalize_time(t):
    if abs(t) >= 1.0:
        return '%.3f s  (%.3f Hz)' % (t, (1/t))
    elif abs(t) >= 0.001:
        if 1/t/1000 < 1:
            return '%.3f ms (%.3f Hz)' % (t * 1000.0, (1/t))
        else:
            return '%.3f ms (%.3f kHz)' % (t * 1000.0, (1/t)/1000)
    elif abs(t) >= 0.000001:
        if 1/t/1000/1000 < 1:
            return '%.3f μs (%.3f kHz)' % (t * 1000.0 * 1000.0, (1/t)/1000)
        else:
            return '%.3f μs (%.3f MHz)' % (t * 1000.0 * 1000.0, (1/t)/1000/1000)
    elif abs(t) >= 0.000000001:
        if 1/t/1000/1000/1000:
            return '%.3f ns (%.3f MHz)' % (t * 1000.0 * 1000.0 * 1000.0, (1/t)/1000/1000)
        else:
            return '%.3f ns (%.3f GHz)' % (t * 1000.0 * 1000.0 * 1000.0, (1/t)/1000/1000/1000)
    else:
        return '%f' % t

def normalize_total_time(t):
    if abs(t) >= 1.0:
        return '%.3f s' % t
    elif abs(t) >= 0.001:
        if 1/t/1000 < 1:
            return '%.3f ms' % (t * 1000.0)
        else:
            return '%.3f ms' % (t * 1000.0)
    elif abs(t) >= 0.000001:
        if 1/t/1000/1000 < 1:
            return '%.3f μs' % (t * 1000.0 * 1000.0)
        else:
            return '%.3f μs' % (t * 1000.0 * 1000.0)
    elif abs(t) >= 0.000000001:
        if 1/t/1000/1000/1000:
            return '%.3f ns' % (t * 1000.0 * 1000.0 * 1000.0)
        else:
            return '%.3f ns' % (t * 1000.0 * 1000.0 * 1000.0)
    else:
        return '%f' % t

def _channel_decl(count):
    return tuple([
        {'id': 'bit{}'.format(i), 'name': 'Bit{}'.format(i), 'desc': 'Bit position {}'.format(i)}
        for i in range(0, count)
    ])

class Decoder(srd.Decoder):
    api_version = 3
    id = 'syscall_analyzer'
    name = 'Syscall Analyzer'
    longname = 'Syscall Analyzer for ConceptOS'
    desc = 'Calculate statistics on syscalls.'
    license = 'gplv2+'
    inputs = ['logic']
    outputs = []
    tags = ['Clock/timing', 'Util']
    channels = (
        {'id': 'syscall_timing', 'name': 'Syscall Timing Line', 'desc': 'Syscall Timing Line'},
    )
    optional_channels = _channel_decl(MAX_CHANNELS)
    annotations = (
        ('syscall_number', 'SyscallNumber'),
        ('syscall_time', 'SyscallTime'),
        ('syscall_max_time', 'SyscallMaxTime'),
        ('syscall_avg_time', 'SyscallAvgTime'),
        ('syscall_min_time', 'SyscallMinTime'),
        ('syscall_avg_intervals', 'SyscallAvgIntervals'),
    )
    annotation_rows = (
        ('syscall_number', 'SyscallNumber', (0,)),
        ('syscall_time', 'SyscallTime', (1,)),
        ('syscall_max_time', 'SyscallMaxTime', (2,)),
        ('syscall_avg_time', 'SyscallAvgTime', (3,)),
        ('syscall_min_time', 'SyscallMinTime', (4,)),
        ('syscall_avg_intervals', 'SyscallAvgIntervals', (5,)),
    )
    options = (
        
    )

    def __init__(self):
        self.reset()

    def reset(self):
        self.samplerate = None
        self.last_samplenum = None
        self.last_n = deque()
        self.chunks = 0
        self.level_changed = False
        self.last_t = None
        self.min_map = dict() # an entry for each syscall number, the min
        self.max_map = dict() # an entry for each syscall number, the max
        self.avg_map = dict()  # two entries for each syscall number. Current avg, total elements
        self.elements_map = dict()  # a list of items for each syscall number, used to compute variance

    def metadata(self, key, value):
        if key == srd.SRD_CONF_SAMPLERATE:
            self.samplerate = value

    def start(self):
        self.out_ann = self.register(srd.OUTPUT_ANN)

    def _get_number(self, pin_levels) -> int:
        return int(packbits(pin_levels, bitorder='little')[0])

    def _visualize_with_confidence(self, syscall_num) -> str:
        # Get elements
        elements = self.elements_map[syscall_num]
        # Compute variance
        avg = np.sum(elements) / len(elements)
        variance = np.sum([pow(x - avg, 2) for x in elements]) / (len(elements) - 1)
        d = 1.96
        lower_bound = avg - d*np.sqrt(variance / len(elements))
        upper_bound = avg + d*np.sqrt(variance / len(elements))
        return f"[{normalize_total_time(lower_bound)},{normalize_total_time(upper_bound)}]"

    def decode(self):
        if not self.samplerate:
            raise DecoderError('Cannot decode without samplerate.')
        # Get the configured channels
        channels = [ch for ch in range(0, MAX_CHANNELS+1) if self.has_channel(ch)]
        # Setup wait condition
        wait_cond = [{ch: 'e'} for ch in channels] # Look for channel change (edge)
        # Initialize
        active_syscall_number = None
        start_syscall_sample = None
        last_syscall_number_change_sample = None
        last_syscall_number = None
        last_active = 0
        
        while True:
            # Convert signals to number
            pins = self.wait(wait_cond)
            pin_levels = [b if b in (0, 1) else 0 for b in pins]   
            
            syscall_active = pin_levels[0]
            syscall_number = self._get_number(pin_levels[1:])
            if active_syscall_number is None:
                active_syscall_number = syscall_number
                last_syscall_number_change_sample = self.samplenum
                last_syscall_number = syscall_number
                continue # Ignore
            
            # Only on logic change compute
            if syscall_active != last_active:
                # Rising edge or falling edge
                if syscall_active > last_active:
                    # Rising edge, syscall started. Mark it
                    active_syscall_number = syscall_number
                    start_syscall_sample = self.samplenum
                else:
                    # Falling edge, syscall ended. Compute
                    if syscall_number != active_syscall_number:
                        #raise DecoderError("Increase resolution, possible missing syscall")
                        # Use the last number as syscall
                        start_syscall_sample = last_syscall_number_change_sample

                    t_syscall = (self.samplenum - start_syscall_sample) / self.samplerate
                    self.put(start_syscall_sample, self.samplenum, self.out_ann, [0, [f"{syscall_number}"]])    
                    self.put(start_syscall_sample, self.samplenum, self.out_ann, [1, [normalize_time(t_syscall)]])  
                    # Update stats
                    # -> Min
                    if syscall_number not in self.min_map:
                        self.min_map[syscall_number] = t_syscall
                    else:
                        if self.min_map[syscall_number] > t_syscall:
                            self.min_map[syscall_number] = t_syscall
                    # -> Max
                    if syscall_number not in self.max_map:
                        self.max_map[syscall_number] = t_syscall
                    else:
                        if self.max_map[syscall_number] < t_syscall:
                            self.max_map[syscall_number] = t_syscall
                    # -> Avg & Var
                    # new_avg = (old_avg * old_num_elems + new_elem) / (old_num_elems + 1)
                    if syscall_number not in self.avg_map:
                        self.avg_map[syscall_number] = [t_syscall, 1]
                        self.elements_map[syscall_number] = [t_syscall]
                    else:
                        old_avg, old_num = self.avg_map[syscall_number]
                        new_num = old_num + 1
                        new_avg = (old_avg * old_num + t_syscall) / new_num
                        self.avg_map[syscall_number] = [new_avg, new_num]
                        self.elements_map[syscall_number].append(t_syscall)
                    
                    self.put(start_syscall_sample, self.samplenum, self.out_ann, [2, [normalize_time(self.max_map[syscall_number])]]) 
                    self.put(start_syscall_sample, self.samplenum, self.out_ann, [3, [normalize_time(self.avg_map[syscall_number][0])]])     
                    self.put(start_syscall_sample, self.samplenum, self.out_ann, [4, [normalize_time(self.min_map[syscall_number])]]) 
                    if len(self.elements_map[syscall_number]) > 1:
                        self.put(start_syscall_sample, self.samplenum, self.out_ann, [5, [self._visualize_with_confidence(syscall_number)]])   
            
            # Prepare for next cycle
            if last_syscall_number != syscall_number:
                last_syscall_number_change_sample = self.samplenum
                last_syscall_number = syscall_number
            last_active = syscall_active