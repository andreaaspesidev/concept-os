use crate::i2c::I2C_Channel;
use bthermo_api::ThermoError;
use userlib::{sys_get_timer, sys_log};

/**
 * TMP117
 *
 * Features:
 *  - meet ASTM E1112 and ISO 80601 requirements
 *  - serve as a single chip digital alternative to a Platinum RTD (Class AA RTD)
 *  - 100% tested on a production setup that is NIST traceable
 *  - ±0.1°C (Maximum) From –20°C to +50°C
 *  - ±0.15°C (Maximum) From –40°C to +70°C
 *  - ±0.2°C (Maximum) From –40°C to +100°C
 *  - ±0.25°C (Maximum) From –55°C to +125°C
 *  - ±0.3°C (Maximum) From –55°C to +150°C
 *  -Low Power Consumption 3.5-µA, 1-Hz Conversion Cycle
 *  
 *  Datasheet http://www.ti.com/lit/ds/symlink/tmp117.pdf
 */
const TMP117_ADDR: u8 = 0x49;

const TMP117_REG_TEMPERATURE: u8 = 0x00;
const TMP117_REG_CONFIGURATION: u8 = 0x01;
const TMP117_REG_DEVICE_ID: u8 = 0x0F;

const TMP117_RESOLUTION: f32 = 0.0078125_f32;
const UPDATE_MS: u64 = 1000;

pub struct TMP117 {
    last_update: u64,
    last_temp: f32,
}

impl TMP117 {
    pub fn new() -> Self {
        Self {
            last_update: 0,
            last_temp: 0.0,
        }
    }
    pub fn init_hardware(&mut self, i2c: &mut I2C_Channel) -> Result<(), ThermoError> {
        // Read device ID
        let mut device_id: [u8; 2] = [0; 2];
        i2c.i2c_mem_read(TMP117_ADDR, TMP117_REG_DEVICE_ID, &mut device_id)
            .map_err(|_| ThermoError::TempNotConnected)?;
        if u16::from_be_bytes(device_id) & 0x0fff != 0x117 {
            return Err(ThermoError::TempNotConnected);
        }
        // Set configuration bits
        // .0: not used
        // .1: Soft_Reset
        // .2: DR/Alert
        // .3: POL
        // .4: T/nA
        // .6-.5: AVG[1:0] (01: 8 Averaged conversions)
        // .9-.7: Conversion cycle bit. (100: 1s total with average)
        // .11-.10: MOD[1:0] (00:  Continuous conversion (CC))
        let config_high: u8 = 0b0000_00_10;
        let config_low: u8 = 0b0_01_0_0_0_0_0;
        let config_bytes: [u8; 2] = [config_high, config_low];
        i2c.i2c_mem_write(TMP117_ADDR, TMP117_REG_CONFIGURATION, &config_bytes)
            .map_err(|_| ThermoError::TempNotConnected)
    }

    pub fn read_temperature(&mut self, i2c: &mut I2C_Channel) -> Result<f32, ThermoError> {
        // Avoid reading too often. The temperature updates
        // every 1 second
        let now = sys_get_timer().now;
        if now - self.last_update > UPDATE_MS {
            let mut raw_data: [u8; 2] = [0; 2];
            // Read data from the sensor
            i2c.i2c_mem_read(TMP117_ADDR, TMP117_REG_TEMPERATURE, &mut raw_data)
                .map_err(|_| ThermoError::TempNotConnected)?;
            // Convert it
            let raw_temp = i16::from_be_bytes(raw_data);
            // Store it
            self.last_temp = (raw_temp as f32) * TMP117_RESOLUTION;
            self.last_update = now;
        }
        return Ok(self.last_temp);
    }
}
