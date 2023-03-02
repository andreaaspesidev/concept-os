use bthermo_api::NUM_TEMPERATURES;
use ringbuffer::{RingBufferExt, RingBufferWrite};

use crate::state::StateManager;

const UPDATE_MS: u64 = 2000;

pub struct History {
    last_update_ticks: u64,
}

impl History {
    pub fn new() -> Self {
        Self {
            last_update_ticks: 0,
        }
    }
    pub fn add_temperature(
        &mut self,
        current_ticks: u64,
        new_temp: f32,
        state: &mut StateManager,
    ) {
        // Only update every tot seconds
        if current_ticks - self.last_update_ticks >= UPDATE_MS - 1 {
            let history_state = state.get_history_state_mut();
            // Add the new temperature
            history_state.circular_buffer.push(new_temp);
            self.last_update_ticks = current_ticks;
        }
    }
    pub fn get_temperatures(
        &self,
        state: &StateManager,
    ) -> [f32; NUM_TEMPERATURES] {
        let mut response_buffer: [f32; NUM_TEMPERATURES] =
            [0.0; NUM_TEMPERATURES];
        let mut pos: usize = 0;
        let history_state = state.get_history_state();
        for i in 0..NUM_TEMPERATURES {
            if let Some(temp) = history_state
                .circular_buffer
                .get(i as isize - NUM_TEMPERATURES as isize)
            {
                response_buffer[pos] = *temp;
                pos += 1;
            }
        }
        response_buffer
    }
    pub fn perform_operation(&self, state: &StateManager) -> f32 {
        cfg_if::cfg_if! {
            if #[cfg(feature = "v1")] {
                let mut max: f32 = f32::MIN;
                let history_state = state.get_history_state();
                for i in 0..NUM_TEMPERATURES {
                    if let Some(temp) = history_state
                        .circular_buffer
                        .get(i as isize - NUM_TEMPERATURES as isize)
                    {
                        if *temp > max {
                            max = *temp;
                        }
                    }
                }
                return max;
            } else if #[cfg(feature = "v2")] {
                let mut avg: f32 = 0.0;
                let mut total: u16 = 0;
                let history_state = state.get_history_state();
                for i in 0..NUM_TEMPERATURES {
                    if let Some(temp) = history_state
                        .circular_buffer
                        .get(i as isize - NUM_TEMPERATURES as isize)
                    {
                        avg += *temp;
                        total += 1;
                    }
                }
                return avg / total as f32;
            } else {
                compile_error!("Must enable at least one feature");
            }
        }
    }
}
