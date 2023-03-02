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
    id = 'scheduler_analyzer'
    name = 'Scheduler Analyzer'
    longname = 'Scheduler Analyzer for ConceptOS components'
    desc = 'Calculate statistics on components.'
    license = 'gplv2+'
    inputs = ['logic']
    outputs = []
    tags = ['Clock/timing', 'Util']
    optional_channels = _channel_decl(MAX_CHANNELS)
    annotations = (
        ('component_id', 'ComponentID'),
        ('process_time', 'ProcessTime'),
        ('wait_time', 'WaitTime'),
        ('total_process_time', 'TotalProcessTime'),
        ('total_waiting_time', 'TotalWaitingTime'),
    )
    annotation_rows = (
        ('component_id', 'ComponentID', (0,)),
        ('process_time', 'ProcessTime', (1,)),
        ('wait_time', 'WaitTime', (2,)),
        ('total_process_time', 'TotalProcessTime', (3,)),
        ('total_waiting_time', 'TotalWaitingTime', (4,)),
    )
    options = (
        { 'id': 'component_id', 'desc': 'Component ID', 'default': 0 },
        { 'id': 'start_time', 'desc': 'Start Time (ms)', 'default': 0 },
        { 'id': 'end_time', 'desc': 'End Time (ms)', 'default': 10000 },
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

    def metadata(self, key, value):
        if key == srd.SRD_CONF_SAMPLERATE:
            self.samplerate = value

    def start(self):
        self.out_ann = self.register(srd.OUTPUT_ANN)

    def _get_current_id(self, pins) -> int:
        pin_levels = [b if b in (0, 1) else 0 for b in pins]   
        return int(packbits(pin_levels, bitorder='little')[0])

    def decode(self):
        if not self.samplerate:
            raise DecoderError('Cannot decode without samplerate.')
        # Get the configured channels
        channels = [ch for ch in range(0, MAX_CHANNELS) if self.has_channel(ch)]
        # Setup wait condition
        wait_cond = [{ch: 'e'} for ch in channels] # Look for channel change (edge)

        # Initialize
        component_id = self._get_current_id(self.wait())
        last_component_ended_sample = self.samplenum
        last_samplenum = self.samplenum
        t_total_processing = 0
        t_total_waiting = 0
        while True:
            # Convert signals to number
            pins = self.wait(wait_cond)
            next_component_id = self._get_current_id(pins)
            if next_component_id == component_id:
                continue  # Ignore
            # Output the component id
            self.put(last_samplenum, self.samplenum, self.out_ann, [0, [f"{component_id}"]])
            # Check wheter to output time
            
            # Compute processing time of that component
            if component_id == self.options['component_id']:
                t_processing = (self.samplenum - last_samplenum) / self.samplerate
                t_waiting = (last_samplenum - last_component_ended_sample) / self.samplerate
                # Visualize
                self.put(last_samplenum, self.samplenum, self.out_ann, [1, [normalize_time(t_processing)]])
                self.put(last_component_ended_sample, last_samplenum, self.out_ann, [2, [normalize_time(t_waiting)]])
                # Update totals
                current_time = self.samplenum / self.samplerate
                t_start = self.options['start_time'] / 1000
                t_end = self.options['end_time'] / 1000
                if current_time > t_start and current_time < t_end:
                    # Processing time
                    t_process_end = self.samplenum / self.samplerate
                    t_process_start = last_samplenum / self.samplerate
                    t_process_start_sample = last_samplenum
                    t_process_end_sample = self.samplenum
                    if t_process_start < t_start:
                        t_process_start_sample = int(t_start * self.samplerate)
                    if t_process_end > t_end:
                        t_process_end_sample = int(t_end * self.samplerate)
                    t_process_corrected = (t_process_end_sample - t_process_start_sample) / self.samplerate
                    if t_process_corrected > 0:
                        t_total_processing += t_process_corrected
                        self.put(t_process_start_sample, t_process_end_sample, self.out_ann, [3, [normalize_total_time(t_total_processing)]])
                    # Waiting time
                    t_wait_end = last_samplenum / self.samplerate
                    t_wait_start = last_component_ended_sample / self.samplerate
                    t_wait_start_sample = last_component_ended_sample
                    t_wait_end_sample = last_samplenum
                    if t_wait_start < t_start:
                        t_wait_start_sample = int(t_start * self.samplerate)
                    if t_wait_end > t_end:
                        t_wait_end_sample = int(t_end * self.samplerate)     
                    t_wait_corrected = (t_wait_end_sample - t_wait_start_sample) / self.samplerate
                    if t_wait_corrected > 0:
                        t_total_waiting += t_wait_corrected
                        self.put(t_wait_start_sample, t_wait_end_sample, self.out_ann, [4, [normalize_total_time(t_total_waiting)]])
            
                # Save for next round
                last_component_ended_sample = self.samplenum           
                 
            # Load data for the next round
            last_samplenum = self.samplenum
            component_id = next_component_id