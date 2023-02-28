use bthermo_api::ThermoError;
use rcc_api::RCC;
use stm32l476rg::device as device;
use userlib::*;

const RUNS_PER_US: u32 = 72;
const TIMED_LOOP_US: u32 = 10000;

macro_rules! timed_loop {
    ($condition:expr) => {{
        let mut success: bool = false;
        for _ in 0..(RUNS_PER_US * TIMED_LOOP_US) {
            if $condition {
                success = true;
                break;
            }
        }
        if !success {
            return Err(());
        }
    }};
}

/**
 * Pinout:
 *      D15 -> PB8 -> I2C1_SCL
 *      D14 -> PB9 -> I2C1_SDA
 */
#[allow(non_camel_case_types)]
pub struct I2C_Channel<'a> {
    gpiob: &'a device::gpiob::RegisterBlock,
    i2c1: &'a device::i2c1::RegisterBlock,
}

impl<'a> I2C_Channel<'a> {
    pub fn new() -> Self {
        Self {
            gpiob: unsafe { &*device::GPIOB::PTR },
            i2c1: unsafe { &*device::I2C1::PTR },
        }
    }
    pub fn init_hardware(&mut self, rcc: &mut RCC) -> Result<(), ThermoError> {
        self.init_gpio(rcc);
        self.init_i2c(rcc);
        Ok(())
    }
    fn init_gpio(&mut self, rcc: &mut RCC) {
        // Turn on clock and leave reset
        rcc.enable_clock(rcc_api::Peripheral::GPIOB).unwrap_lite();
        rcc.leave_reset(rcc_api::Peripheral::GPIOB).unwrap_lite();
        // Select alternate function for PB8,PB9
        self.gpiob
            .moder
            .modify(|_, w| w.moder8().alternate().moder9().alternate());
        // Setup alternate function (see STM32F303xD STM32F303xE, pg. 53 (DocID026415))
        self.gpiob.afrh.modify(|_, w| w.afrh8().af4().afrh9().af4());
    }
    fn init_i2c(&mut self, rcc: &mut RCC) {
        // Turn on I2C1 and leave reset
        rcc.enable_clock(rcc_api::Peripheral::I2C1).unwrap();
        rcc.leave_reset(rcc_api::Peripheral::I2C1).unwrap();

        // Turn off peripheral
        self.i2c1.cr1.modify(|_, w| w.pe().disabled());
        // Set timing
        self.i2c1.timingr.write(|w| unsafe { w.bits(0x00300209) }); // STM32 HAL obscure constant for FastMode 400Khz
                                                                    // Set own address at 0
        self.i2c1
            .oar1
            .modify(|_, w| w.oa1en().enabled().oa1().bits(0));
        // Enable the AUTOEND by default, and enable NACK
        self.i2c1
            .cr2
            .modify(|_, w| w.autoend().set_bit().nack().set_bit());
        // Generalcall and NoStretch mode
        self.i2c1
            .cr1
            .modify(|_, w| w.gcen().enabled().nostretch().enabled());
        // Enable I2C
        self.i2c1.cr1.modify(|_, w| w.pe().enabled());
    }

    pub fn i2c_mem_read(
        &mut self,
        device_address: u8,
        mem_address: u8,
        data: &mut [u8],
    ) -> Result<(), ()> {
        // Max data length
        if data.len() > u8::MAX as usize {
            panic!("Too much data in a single packet");
        }
        // Request memory
        self._i2c_select_register(device_address, mem_address)?;

        // Configure reception
        self.i2c1.cr2.modify(|_, w| {
            w.sadd()
                .bits((device_address << 1 | 1) as u16)
                .nbytes()
                .bits(data.len() as u8)
                .autoend()
                .automatic()
                .rd_wrn()
                .read()
                .start()
                .start()
        });

        // Wait for data
        let mut curr_pos: usize = 0;
        while curr_pos < data.len() {
            timed_loop!(self.i2c1.isr.read().rxne().bit_is_set());
            let byte = (self.i2c1.rxdr.read().bits() & 0xFF) as u8;
            data[curr_pos] = byte;
            curr_pos += 1;
        }
        Ok(())
    }

    pub fn i2c_mem_write(
        &mut self,
        device_address: u8,
        mem_address: u8,
        data: &[u8],
    ) -> Result<(), ()> {
        // Max data length
        if data.len() > u8::MAX as usize {
            panic!("Too much data in a single packet");
        }
        // Configure reception
        self.i2c1.cr2.modify(|_, w| {
            w.sadd()
                .bits((device_address << 1 | 1) as u16)
                .nbytes()
                .bits((data.len() + 1) as u8)
                .autoend()
                .automatic()
                .rd_wrn()
                .write()
                .start()
                .start()
        });

        // Start by sending the register address
        // Wait to be ready
        timed_loop!(self.i2c1.isr.read().txis().is_empty());
        // Put address on the tx reg
        self.i2c1.txdr.write(|w| w.txdata().bits(mem_address));

        let mut curr_pos: usize = 0;
        while curr_pos < data.len() {
            // Wait to be ready
            timed_loop!(self.i2c1.isr.read().txis().is_empty());
            // Put address on the tx reg
            self.i2c1.txdr.write(|w| w.txdata().bits(data[curr_pos]));
            curr_pos += 1;
        }
        Ok(())
    }

    fn _i2c_select_register(
        &mut self,
        device_address: u8,
        mem_address: u8,
    ) -> Result<(), ()> {
        // Configure CR2
        self.i2c1.cr2.modify(|_, w| {
            w.sadd()
                .bits((device_address << 1 | 0) as u16)
                .nbytes()
                .bits(8) // The memory address
                .autoend()
                .software()
                .rd_wrn()
                .write()
                .start()
                .start()
                .stop()
                .no_stop()
        });
        // Wait to be ready
        timed_loop!(self.i2c1.isr.read().txis().is_empty());
        // Put address on the tx reg
        self.i2c1.txdr.write(|w| w.txdata().bits(mem_address));
        // Wait until transfer completes
        timed_loop!(self.i2c1.isr.read().txis().is_empty());
        // Put a stop
        self.i2c1
            .cr2
            .modify(|_, w| w.start().no_start().stop().stop());
        Ok(())
    }
}
