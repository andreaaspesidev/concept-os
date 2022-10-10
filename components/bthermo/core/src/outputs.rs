use bthermo_api::{ThermoError, OutputType};
use rcc_api::RCC;
use stm32f303re::device as device;

/**
 * Pinout:
 * OUT1 -> PB5 -> D4
 * OUT2 -> PB4 -> D5
 * OUT3 -> PB10 -> D6
 * OUT4 -> PB6 -> D10
 * 
 * The logic of the pin is inverted, as the outputs turn on when the out is low.
 */

pub struct OutputController<'a> {
    gpiob: &'a device::gpiob::RegisterBlock,
    outs: [bool; 4]
}

impl<'a> OutputController<'a> {
    pub fn new() -> Self {
        Self {
            gpiob: unsafe { &*device::GPIOB::PTR },
            outs: [false; 4]
        }
    }
    pub fn init_hardware(&mut self, rcc: &mut RCC) -> Result<(), ThermoError> {
        // Turn on clock and leave reset
        rcc.enable_clock(rcc_api::Peripheral::GPIOB).unwrap();
        rcc.leave_reset(rcc_api::Peripheral::GPIOB).unwrap();
        // Configure GPIOB
        // Setup pins PB4, PB5, PB6, PB10 as low speed
        self.gpiob.ospeedr.modify(|_, w| {
            w.ospeedr4()
                .low_speed()
                .ospeedr5()
                .low_speed()
                .ospeedr6()
                .low_speed()
                .ospeedr10()
                .low_speed()
        });
        // Setup pins PB4, PB5, PB6, PB10 as output push_pull
        self.gpiob.otyper.modify(|_, w| {
            w.ot4()
                .push_pull()
                .ot5()
                .push_pull()
                .ot6()
                .push_pull()
                .ot10()
                .push_pull()
        });
        // Setup pins PB4, PB5, PB6, PB10 as output
        self.gpiob.moder.modify(|_, w| {
            w.moder4()
                .output()
                .moder5()
                .output()
                .moder6()
                .output()
                .moder10()
                .output()
        });
        // Turn off all outputs
        self.turn_off(OutputType::OUT1);
        self.turn_off(OutputType::OUT2);
        self.turn_off(OutputType::OUT3);
        self.turn_off(OutputType::OUT4);
        Ok(())
    }

    pub fn turn_on(&mut self, out: OutputType) {
        // OUT1 -> PB5
        // OUT2 -> PB4
        // OUT3 -> PB10
        // OUT4 -> PB6
        match out {
            OutputType::OUT1 => self.gpiob.bsrr.write(|w| w.br5().set_bit()),
            OutputType::OUT2 => self.gpiob.bsrr.write(|w| w.br4().set_bit()),
            OutputType::OUT3 => self.gpiob.bsrr.write(|w| w.br10().set_bit()),
            OutputType::OUT4 => self.gpiob.bsrr.write(|w| w.br6().set_bit()),
        }
        self.outs[out as usize -1] = true;
    }

    pub fn turn_off(&mut self, out: OutputType) {
        // OUT1 -> PB5
        // OUT2 -> PB4
        // OUT3 -> PB10
        // OUT4 -> PB6
        match out {
            OutputType::OUT1 => self.gpiob.bsrr.write(|w| w.bs5().set_bit()),
            OutputType::OUT2 => self.gpiob.bsrr.write(|w| w.bs4().set_bit()),
            OutputType::OUT3 => self.gpiob.bsrr.write(|w| w.bs10().set_bit()),
            OutputType::OUT4 => self.gpiob.bsrr.write(|w| w.bs6().set_bit()),
        }
        self.outs[out as usize -1] = false;
    }

    pub fn get_status(&self) -> [bool; 4] {
        self.outs.clone()
    }
}
