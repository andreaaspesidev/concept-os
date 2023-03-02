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
import sigrokdecode as srd
from collections import deque
import sys

class SamplerateError(Exception):
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

def normalize_time_simple(t):
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

class Decoder(srd.Decoder):
    api_version = 3
    id = 'pulse-timing'
    name = 'Pulse Timing'
    longname = 'Timing calculation with frequency and averaging'
    desc = 'Calculate time of pulses.'
    license = 'gplv2+'
    inputs = ['logic']
    outputs = []
    tags = ['Clock/timing', 'Util']
    channels = (
        {'id': 'data', 'name': 'Data', 'desc': 'Data line'},
    )
    annotations = (
        ('pulse_time', 'Time'),
        ('pulse_average', 'AverageTime'),
        ('pulse_min', 'MinTime'),
        ('pulse_max', 'MaxTime'),
        ('pulse_delta', 'Delta'),
    )
    annotation_rows = (
        ('pulse_time', 'Time', (0,)),
        ('pulse_average', 'AverageTime', (1,)),
        ('pulse_min', 'MinTime', (2,)),
        ('pulse_max', 'MaxTime', (3,)),
        ('pulse_delta', 'Delta', (4,)),
    )
    options = (
        { 'id': 'average', 'desc': 'Compute average', 'default': 'no', 'values': ('yes', 'no') },
        { 'id': 'delta', 'desc': 'Show delta from last', 'default': 'no', 'values': ('yes', 'no') },
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

    def _calculate_average_with_confidence(self):
        # Get elements
        elements = self.last_n
        # Compute variance
        avg = np.sum(elements) / len(elements)
        variance = sum([pow(x - avg, 2) for x in elements]) / (len(elements) - 1)
        d = 1.96
        lower_bound = avg - d*np.sqrt(variance / len(elements))
        upper_bound = avg + d*np.sqrt(variance / len(elements))
        return f"[{normalize_time_simple(lower_bound)},{normalize_time_simple(upper_bound)}]"

    def decode(self):
        if not self.samplerate:
            raise SamplerateError('Cannot decode without samplerate.')

        pulse_min = sys.float_info.max
        pulse_max = sys.float_info.min
        while True:
            # Wait for a rising edge
            self.wait({0: 'r'})
            # Mark the time
            start_sample = self.samplenum
            # Wait for the falling edge
            self.wait({0: 'f'})
            # Calculate time
            samples = self.samplenum - start_sample
            t = samples / self.samplerate

            if t > 0 and self.options['average'] == 'yes':
                self.last_n.append(t)

            if t < pulse_min:
                pulse_min = t
            elif t > pulse_max:
                pulse_max = t    

            self.put(start_sample, self.samplenum, self.out_ann,
                     [0, [normalize_time(t)]])
            
            self.put(start_sample, self.samplenum, self.out_ann,
                         [2, [normalize_time_simple(pulse_min)]])
            self.put(start_sample, self.samplenum, self.out_ann,
                         [3, [normalize_time_simple(pulse_max)]])
        
            if len(self.last_n) > 1 and self.options['average'] == 'yes':
                self.put(start_sample, self.samplenum, self.out_ann,
                         [1, [self._calculate_average_with_confidence()]])

            if self.last_t and self.options['delta'] == 'yes':
                self.put(start_sample, self.samplenum, self.out_ann,
                         [4, [normalize_time(t - self.last_t)]])

            self.last_t = t