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
from pathlib import Path
from tempfile import gettempdir
import numpy as np
from numpy import packbits
import sigrokdecode as srd
from collections import deque
import os

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

class RunningStats:
    def __init__(self) -> None:
        self.m_n = 0
        self.m_oldM = 0.0
        self.m_newM = 0.0
        self.m_oldS = 0.0
        self.m_newS = 0.0
        self.m_min = 0.0
        self.m_max = 0.0

    def clear(self):
        self.m_n = 0

    def push(self, x: float) -> None:
        self.m_n += 1

        # Compute min and max
        if self.m_n == 1:
            self.m_min = x
            self.m_max = x
        elif x < self.m_min:
            self.m_min = x
        elif x > self.m_max:
            self.m_max = x

        # See Knuth TAOCP vol 2, 3rd edition, page 232
        if self.m_n == 1:
            self.m_oldM = x
            self.m_newM = x
            self.m_oldS = 0.0
        else:
            self.m_newM = self.m_oldM + (x - self.m_oldM)/self.m_n
            self.m_newS = self.m_oldS + (x - self.m_oldM)*(x - self.m_newM)
            # set up for next iteration
            self.m_oldM = self.m_newM
            self.m_oldS = self.m_newS

    def num_values(self) -> int:
        return self.m_n

    def min(self) -> float:
        if self.m_n > 0:
            return self.m_min
        else:
            return 0.0
        
    def max(self) -> float:
        if self.m_n > 0:
            return self.m_max
        else:
            return 0.0

    def mean(self) -> float:
        if self.m_n > 0:
            return self.m_newM
        else:
            return 0.0

    def variance(self) -> float:
        if self.m_n > 1:
            return self.m_newS/(self.m_n - 1)
        else:
            return 0.0

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
        { 'id': 'stream_fifo_enable', 'desc': 'Stream on fifo', 'default': 'no', 'values': ('yes', 'no')},
        { 'id': 'stream_fifo', 'desc': 'Fifo path', 'default': ''},
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
        self.value_map = dict() # an entry for each syscall number
        
    def metadata(self, key, value):
        if key == srd.SRD_CONF_SAMPLERATE:
            self.samplerate = value

    def start(self):
        self.out_ann = self.register(srd.OUTPUT_ANN)

    def _get_number(self, pin_levels) -> int:
        return int(packbits(pin_levels, bitorder='little')[0])

    # Using Knuth's method for computing the running average and variance
    # http://www.johndcook.com/blog/2008/09/26/comparing-three-methods-of-computing-standard-deviation/
    def _add_stat_for(self, syscall_number, t_syscall):
        if syscall_number not in self.value_map:
            stat = RunningStats()
            stat.push(t_syscall)
            self.value_map[syscall_number] = stat
        else:
            self.value_map[syscall_number].push(t_syscall)

    def _get_avg_with_confidence(self, syscall_num):
        # Get stats
        stats: RunningStats = self.value_map[syscall_num]
        avg = stats.mean()
        num_data = stats.num_values()
        var = stats.variance()
        # Under the assumtion of having enough data, the mean
        # value is distributed as a gaussian
        d = 1.96    # Confidence 0.95
        lower_bound = avg - d*np.sqrt(var / num_data)
        upper_bound = avg + d*np.sqrt(var / num_data)
        return (lower_bound, upper_bound)

    def _visualize_with_confidence(self, syscall_num) -> str:
        lower_bound, upper_bound = self._get_avg_with_confidence(syscall_num)
        return f"[{normalize_total_time(lower_bound)},{normalize_total_time(upper_bound)}]"

    def _log_to_file(self, _time):
        # We put only the most up-to-date values for each syscall
        # Write intestation
        intestation = ";"
        max_times = "MAX;"
        avg_times = "AVG;"
        min_times = "MIN;"
        conf_max = "AVG_CONF_MAX;"
        conf_min = "AVG_CONF_MIN;"
        for syscall_num in sorted(self.value_map.keys()):
            # Get stats
            stats: RunningStats = self.value_map[syscall_num]
            intestation += f"SYSCALL_{syscall_num};"
            max_times += f"{stats.max()};"
            avg_times += f"{stats.mean()};"
            min_times += f"{stats.min()};"
            lower_bound,upper_bound = self._get_avg_with_confidence(syscall_num)
            conf_max += f"{upper_bound};"
            conf_min += f"{lower_bound};"

        # Open pipe truncate and write
        with open(self.options['stream_fifo'], 'a') as f:
            f.write(f"{intestation}\n")
            f.write(f"{max_times}\n")
            f.write(f"{avg_times}\n")
            f.write(f"{min_times}\n")
            f.write(f"{conf_max}\n")
            f.write(f"{conf_min}\n")
            f.write(f"SAMPLE_RATE;{self.samplerate}\n")

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
        # Setup fifo if requested
        if self.options['stream_fifo_enable'] == 'yes':
            if not os.path.exists(self.options['stream_fifo']):
                os.mkfifo(self.options['stream_fifo'])
        
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
                    self._add_stat_for(syscall_number, t_syscall)
                    stats: RunningStats = self.value_map[syscall_number]
                    self.put(start_syscall_sample, self.samplenum, self.out_ann, [2, [normalize_time(stats.max())]]) 
                    self.put(start_syscall_sample, self.samplenum, self.out_ann, [3, [normalize_time(stats.mean())]])     
                    self.put(start_syscall_sample, self.samplenum, self.out_ann, [4, [normalize_time(stats.min())]]) 
                    self.put(start_syscall_sample, self.samplenum, self.out_ann, [5, [self._visualize_with_confidence(syscall_number)]])   

                    if self.options['stream_fifo_enable'] == 'yes':
                        self._log_to_file(t_syscall)

            # Prepare for next cycle
            if last_syscall_number != syscall_number:
                last_syscall_number_change_sample = self.samplenum
                last_syscall_number = syscall_number
            last_active = syscall_active
