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
import os
import math

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
    id = 'performance_analyzer'
    name = 'Performance Analyzer'
    longname = 'Performance Analyzer for ConceptOS components'
    desc = 'Calculate statistics on components.'
    license = 'gplv2+'
    inputs = ['logic']
    outputs = []
    tags = ['Clock/timing', 'Util']
    optional_channels = _channel_decl(MAX_CHANNELS)
    annotations = (
        ('component_id', 'ComponentID'),
        ('process_time', 'ProcessTime'),
        ('process_max', 'ProcessMax'),
        ('process_avg', 'ProcessAvg'),
        ('process_min', 'ProcessMin'),
        ('process_delta', 'ProcessDelta95'),
    )
    annotation_rows = (
        ('component_id', 'ComponentID', (0,)),
        ('process_time', 'ProcessTime', (1,)),
        ('process_max', 'ProcessMax', (2,)),
        ('process_avg', 'ProcessAvg', (3,)),
        ('process_min', 'ProcessMin', (4,)),
        ('process_delta', 'ProcessDelta95', (5,)),
    )
    options = (
        { 'id': 'component_id', 'desc': 'Component ID', 'default': 0 },
        { 'id': 'start_time', 'desc': 'Start Time (ms)', 'default': 0 },
        { 'id': 'end_time', 'desc': 'End Time (ms)', 'default': 10000 },
        { 'id': 'delta', 'desc': 'Delta value to distinguish values (us)', 'default': 5 },
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
        self.value_map = {}

    def metadata(self, key, value):
        if key == srd.SRD_CONF_SAMPLERATE:
            self.samplerate = value

    def start(self):
        self.out_ann = self.register(srd.OUTPUT_ANN)

    def _get_current_id(self, pins) -> int:
        pin_levels = [b if b in (0, 1) else 0 for b in pins]   
        return int(packbits(pin_levels, bitorder='little')[0])
    
    def _is_present(self, value):
        # Under the assumption such values are a few, here we iterate. 
        # More advanced methods (btrees, ...) are advised
        delta = self.options['delta'] / 1000000
        for n_val in self.value_map.keys():
            if value > n_val - delta and value < n_val + delta:
                return n_val
        return None

    # Using Knuth's method for computing the running average and variance
    # http://www.johndcook.com/blog/2008/09/26/comparing-three-methods-of-computing-standard-deviation/
    def _add_stat_for(self, nominal_time, processing_time):
        if nominal_time not in self.value_map:
            stat = RunningStats()
            stat.push(processing_time)
            self.value_map[nominal_time] = stat
        else:
            self.value_map[nominal_time].push(processing_time)

    def _compute_stat(self, processing_time):
        # Check if already present
        n_val = self._is_present(processing_time)
        
        if n_val is None:
            n_val = processing_time
        
        # Compute data
        self._add_stat_for(n_val, processing_time)

        # Retrieve some values
        stats: RunningStats = self.value_map[n_val]
        min = stats.min()
        max = stats.max()
        avg = stats.mean()
        count = stats.num_values()
        delta95 = self._delta_95(stats)

        # Return
        return (max, avg, min, delta95, count)

    def _delta_95(self, stats: RunningStats):
        d = 1.96    # Confidence 0.95
        return d*math.sqrt(stats.variance() / stats.num_values())

    def _get_avg_with_confidence(self, stats: RunningStats):
        lower_bound = stats.mean() - self._delta_95(stats)
        upper_bound = stats.mean() + self._delta_95(stats)
        return lower_bound, upper_bound

    def _log_to_file(self):
        # We put only the most up-to-date values for each syscall
        # Write intestation
        intestation = ";"
        max_times = "MAX;"
        avg_times = "AVG;"
        min_times = "MIN;"
        conf_max = "AVG_CONF_MAX;"
        conf_min = "AVG_CONF_MIN;"
        count_times = "COUNT;"
        for n_val in sorted(self.value_map.keys()):
            # Retrieve some values
            stats: RunningStats = self.value_map[n_val]
            min = stats.min()
            max = stats.max()
            avg = stats.mean()
            count = stats.num_values()
            lower, upper = self._get_avg_with_confidence(stats)
            
            intestation += f"{normalize_total_time(n_val)};"
            max_times += f"{max};"
            avg_times += f"{avg};"
            min_times += f"{min};"
            conf_max += f"{upper};"
            conf_min += f"{lower};"
            count_times += f"{count};"
        # Open pipe truncate and write
        with open(self.options['stream_fifo'], 'a') as f:
            f.write(f"{intestation}\n")
            f.write(f"{max_times}\n")
            f.write(f"{avg_times}\n")
            f.write(f"{min_times}\n")
            f.write(f"{conf_max}\n")
            f.write(f"{conf_min}\n")
            f.write(f"{count_times}\n")
            f.write(f"SAMPLE_RATE;{self.samplerate}\n")

    def decode(self):
        if not self.samplerate:
            raise DecoderError('Cannot decode without samplerate.')
        # Get the configured channels
        channels = [ch for ch in range(0, MAX_CHANNELS) if self.has_channel(ch)]
        # Setup wait condition
        wait_cond = [{ch: 'e'} for ch in channels] # Look for channel change (edge)

        # Setup fifo if requested
        if self.options['stream_fifo_enable'] == 'yes':
            if not os.path.exists(self.options['stream_fifo']):
                os.mkfifo(self.options['stream_fifo'])

        # Initialize
        component_id = self._get_current_id(self.wait())
        last_samplenum = self.samplenum

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

                current_time = self.samplenum / self.samplerate
                t_start = self.options['start_time'] / 1000
                t_end = self.options['end_time'] / 1000
                if current_time > t_start and current_time < t_end:
                    # Identify different types of processing time using a simple method: variation
                    max, avg, min, delta95, count = self._compute_stat(t_processing)
                    # Add to graph
                    self.put(last_samplenum, self.samplenum, self.out_ann, [1, [normalize_total_time(t_processing)]])
                    self.put(last_samplenum, self.samplenum, self.out_ann, [2, [f"{normalize_total_time(max)} ({count} elements)"]])
                    self.put(last_samplenum, self.samplenum, self.out_ann, [3, [f"{normalize_total_time(avg)} ({count} elements)"]])
                    self.put(last_samplenum, self.samplenum, self.out_ann, [4, [f"{normalize_total_time(min)} ({count} elements)"]])
                    self.put(last_samplenum, self.samplenum, self.out_ann, [5, [f"{normalize_total_time(delta95)} ({count} elements)"]])
                    # Log
                    if self.options['stream_fifo_enable'] == 'yes':
                        self._log_to_file()
            
            # Load data for the next round
            last_samplenum = self.samplenum
            component_id = next_component_id