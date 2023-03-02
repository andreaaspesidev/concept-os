use crate::i2c::I2C_Channel;
use bthermo_api::{ThermoError, TimeStructure};
use userlib::sys_log;

/**
 * DS3231
 *
 * The DS3231 is a low-cost, extremely accurate I2C real-time clock (RTC) with an integrated temperature
 * compensated crystal oscillator (TCXO) and crystal.
 * The device incorporates a battery input, and maintains accurate timekeeping when main power to the device
 * is interrupted. The integration of the crystal resonator enhances the long-term accuracy of the device as well
 * as reduces the piece-part count in a manufacturing line.
 * The DS3231 is available in commercial and industrial temperature ranges, and is offered in a 16-pin,
 * 300-mil SO package.
 *
 * The RTC maintains seconds, minutes, hours, day, date, month, and year information.
 * The date at the end of the month is automatically adjusted for months with fewer than 31 days,
 * including corrections for leap year. The clock operates in either the 24-hour or 12-hour
 * format with an AM/PM indicator. Two programmable time-of-day alarms and a programmable
 * square-wave output are provided. Address and data are transferred serially through an I2C
 * bidirectional bus.
 */
const CLOCK_ADDRESS: u8 = 0x68;

pub struct DS3231 {}

impl DS3231 {
    pub fn new() -> Self {
        Self {}
    }
    pub fn init_hardware(&mut self, _: &mut I2C_Channel) -> Result<(), ThermoError> {
        // Nothing to change from the default configuration
        Ok(())
    }

    pub fn read_sensor(&mut self, i2c: &mut I2C_Channel) -> Result<TimeStructure, ThermoError> {
        let mut raw_data: [u8; 7] = [0; 7];
        // Read data from the sensor
        i2c.i2c_mem_read(CLOCK_ADDRESS, 0, &mut raw_data)
            .map_err(|_| ThermoError::RTCNotConnected)?;
        // Convert it
        let ss: u8 = bcd2dec(raw_data[0] & 0x7F);
        let mm = bcd2dec(raw_data[1]);
        let hh = bcd2dec(raw_data[2] & 0b00111111);
        let wd = bcd2dec(raw_data[3]);
        let d = bcd2dec(raw_data[4]);
        let m = bcd2dec(raw_data[5]);
        let ys = bcd2dec(raw_data[6]);
        return Ok(TimeStructure {
            sec: ss,
            min: mm,
            hour: hh,
            week_day: wd,
            day: d,
            month: m,
            year: ys as u16 + 2000_u16,
        });
    }

    pub fn set_date(
        &mut self,
        i2c: &mut I2C_Channel,
        sec: u8,
        min: u8,
        hour: u8,
        week_day: u8,
        day: u8,
        month: u8,
        year: u16,
    ) -> Result<(), ThermoError> {
        assert!(year >= 2000);
        // Prepare arguments
        let mut raw_data: [u8; 7] = [0; 7];
        raw_data[0] = dec2bcd(sec);
        raw_data[1] = dec2bcd(min);
        raw_data[2] = dec2bcd(hour) & 0b10111111;
        raw_data[3] = dec2bcd(week_day);
        raw_data[4] = dec2bcd(day);
        raw_data[5] = dec2bcd(month);
        raw_data[6] = dec2bcd((year - 2000) as u8);
        // Send data
        i2c.i2c_mem_write(CLOCK_ADDRESS, 0x00, &raw_data)
            .map_err(|_| ThermoError::RTCNotConnected)
    }
}

fn dec2bcd(val: u8) -> u8 {
    val / 10 * 16 + (val % 10)
}

fn bcd2dec(val: u8) -> u8 {
    val / 16 * 10 + (val % 16)
}
