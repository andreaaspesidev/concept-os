#![allow(unused)]

use fugit::{HertzU32 as Hertz};
use fugit::RateExtU32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MsiFreq {
    #[doc = "range 0 around 100 kHz"]
    RANGE100K = 0,
    #[doc = "range 1 around 200 kHz"]
    RANGE200K = 1,
    #[doc = "range 2 around 400 kHz"]
    RANGE400K = 2,
    #[doc = "range 3 around 800 kHz"]
    RANGE800K = 3,
    #[doc = "range 4 around 1 MHz"]
    RANGE1M = 4,
    #[doc = "range 5 around 2 MHz"]
    RANGE2M = 5,
    #[doc = "range 6 around 4 MHz"]
    RANGE4M = 6,
    #[doc = "range 7 around 8 MHz"]
    RANGE8M = 7,
    #[doc = "range 8 around 16 MHz"]
    RANGE16M = 8,
    #[doc = "range 9 around 24 MHz"]
    RANGE24M = 9,
    #[doc = "range 10 around 32 MHz"]
    RANGE32M = 10,
    #[doc = "range 11 around 48 MHz"]
    RANGE48M = 11,
}

impl MsiFreq {
    fn to_hertz(self) -> Hertz {
        (match self {
            Self::RANGE100K => 100_000,
            Self::RANGE200K => 200_000,
            Self::RANGE400K => 400_000,
            Self::RANGE800K => 800_000,
            Self::RANGE1M => 1_000_000,
            Self::RANGE2M => 2_000_000,
            Self::RANGE4M => 4_000_000,
            Self::RANGE8M => 8_000_000,
            Self::RANGE16M => 16_000_000,
            Self::RANGE24M => 24_000_000,
            Self::RANGE32M => 32_000_000,
            Self::RANGE48M => 48_000_000,
        })
        .Hz()
    }
}

const HSI: u32 = 16_000_000; // Hz

/// Clock configuration
pub struct CFGR {
    hse: Option<HseConfig>,
    lse: Option<LseConfig>,
    msi: Option<MsiFreq>,
    hsi48: bool,
    lsi: bool,
    hclk: Option<u32>,
    pclk1: Option<u32>,
    pclk2: Option<u32>,
    sysclk: Option<u32>,
    pll_source: Option<PllSource>,
    pll_config: Option<PllConfig>,
}

impl CFGR {
    pub fn new() -> Self {
        CFGR {
            hse: None,
            lse: None,
            msi: None,
            hsi48: false,
            lsi: false,
            hclk: None,
            pclk1: None,
            pclk2: None,
            sysclk: None,
            pll_source: None,
            pll_config: None,
        }
    }
    /// Add an HSE to the system
    pub fn hse(mut self, freq: Hertz, bypass: CrystalBypass, css: ClockSecuritySystem) -> Self {
        self.hse = Some(HseConfig {
            speed: freq.raw(),
            bypass,
            css,
        });

        self
    }

    /// Add an 32.768 kHz LSE to the system
    pub fn lse(mut self, bypass: CrystalBypass, css: ClockSecuritySystem) -> Self {
        self.lse = Some(LseConfig { bypass, css });

        self
    }

    /// Sets a frequency for the AHB bus
    pub fn hclk(mut self, freq: Hertz) -> Self {
        self.hclk = Some(freq.raw());
        self
    }

    /// Enable the 48 MHz USB, RNG, SDMMC HSI clock source. Not available on all stm32l4x6 series
    pub fn hsi48(mut self, on: bool) -> Self {
        self.hsi48 = on;
        self
    }

    /// Enables the MSI with the specified speed
    pub fn msi(mut self, range: MsiFreq) -> Self {
        self.msi = Some(range);
        self
    }

    /// Sets LSI clock on (the default) or off
    pub fn lsi(mut self, on: bool) -> Self {
        self.lsi = on;
        self
    }

    /// Sets a frequency for the APB1 bus
    pub fn pclk1(mut self, freq: Hertz) -> Self {
        self.pclk1 = Some(freq.raw());
        self
    }

    /// Sets a frequency for the APB2 bus
    pub fn pclk2(mut self, freq: Hertz) -> Self {
        self.pclk2 = Some(freq.raw());
        self
    }

    /// Sets the system (core) frequency
    pub fn sysclk(mut self, freq: Hertz) -> Self {
        self.sysclk = Some(freq.raw());
        self
    }

    /// Sets the system (core) frequency with some pll configuration
    pub fn sysclk_with_pll(mut self, freq: Hertz, cfg: PllConfig) -> Self {
        self.pll_config = Some(cfg);
        self.sysclk = Some(freq.raw());
        self
    }

    /// Sets the PLL source
    pub fn pll_source(mut self, source: PllSource) -> Self {
        self.pll_source = Some(source);
        self
    }

    /// Freezes the clock configuration, making it effective
    pub fn freeze(&self) -> Clocks {
        let rcc = unsafe { &*stm32l432kc::device::RCC::ptr() };
        let pwr = unsafe {&*stm32l432kc::device::PWR::ptr() };
        let flash = unsafe { &*stm32l432kc::device::FLASH::ptr() };

        // Switch to MSI to prevent problems with PLL configuration.
        if rcc.cr.read().msion().bit_is_clear() {
            // Turn on MSI and configure it to 4MHz.
            rcc.cr.modify(|_, w| {
                w.msirgsel().set_bit(); // MSI Range is provided by MSIRANGE[3:0].
                w.msirange().range4m();
                w.msipllen().clear_bit();
                w.msion().set_bit()
            });

            // Wait until MSI is running
            while rcc.cr.read().msirdy().bit_is_clear() {
                cortex_m::asm::nop();
            }
        }
        if rcc.cfgr.read().sws().bits() != 0 {
            // Set MSI as a clock source, reset prescalers.
            rcc.cfgr.reset();
            // Wait for clock switch status bits to change.
            while rcc.cfgr.read().sws().bits() != 0 {
                cortex_m::asm::nop();
            }
        }

        //
        // 1. Setup clocks
        //

        // Turn on the internal 32 kHz LSI oscillator
        let lsi_used = match (self.lsi, &self.lse) {
            (true, _)
            | (
                _,
                &Some(LseConfig {
                    bypass: _,
                    css: ClockSecuritySystem::Enable,
                }),
            ) => {
                rcc.csr.modify(|_, w| w.lsion().set_bit());

                // Wait until LSI is running
                while rcc.csr.read().lsirdy().bit_is_clear() {
                    cortex_m::asm::nop();
                }

                true
            }
            _ => false,
        };

        if let Some(lse_cfg) = &self.lse {
            // Turn on PWR clock
            rcc.apb1enr1.modify(|_, w| w.pwren().set_bit());

            // 1. Unlock the backup domain
            pwr.cr1.modify(|_, w| w.dbp().set_bit());

            // 2. Setup the LSE
            rcc.bdcr.modify(|_, w| {
                w.lseon().set_bit(); // Enable LSE

                if lse_cfg.bypass == CrystalBypass::Enable {
                    w.lsebyp().set_bit();
                } else {
                    unsafe {
                        w.lsedrv().bits(0b11);
                    } // Max drive strength, TODO: should probably be settable
                }

                w
            });

            // Wait until LSE is running
            while rcc.bdcr.read().lserdy().bit_is_clear() {}

            // Setup CSS
            if lse_cfg.css == ClockSecuritySystem::Enable {
                // Enable CSS and interrupt
                rcc.bdcr.modify(|_, w| w.lsecsson().set_bit());
                rcc.cier.modify(|_, w| w.lsecssie().set_bit());
            }
        }

        // If HSE is available, set it up
        if let Some(hse_cfg) = &self.hse {
            rcc.cr.write(|w| {
                w.hseon().set_bit();

                if hse_cfg.bypass == CrystalBypass::Enable {
                    w.hsebyp().set_bit();
                }

                w
            });

            while rcc.cr.read().hserdy().bit_is_clear() {}

            // Setup CSS
            if hse_cfg.css == ClockSecuritySystem::Enable {
                // Enable CSS
                rcc.cr.modify(|_, w| w.csson().set_bit());
            }
        }

        if let Some(msi) = self.msi {
            unsafe {
                rcc.cr.modify(|_, w| {
                    w.msirange()
                        .bits(msi as u8)
                        .msirgsel()
                        .set_bit()
                        .msion()
                        .set_bit();

                    // If LSE is enabled, enable calibration of MSI
                    if self.lse.is_some() {
                        w.msipllen().set_bit();
                    }

                    w
                })
            };

            // Wait until MSI is running
            while rcc.cr.read().msirdy().bit_is_clear() {
                cortex_m::asm::nop();
            }
        }

        // Turn on USB, RNG Clock using the HSI48 CLK source
        if self.hsi48 {
            // p. 180 in ref-manual
            rcc.crrcr.modify(|_, w| w.hsi48on().set_bit());

            // Wait until HSI48 is running
            while rcc.crrcr.read().hsi48rdy().bit_is_clear() {
                cortex_m::asm::nop();
            }
        }

        // Select MSI as clock source for usb48, rng ...
        if let Some(MsiFreq::RANGE48M) = self.msi {
            unsafe { rcc.ccipr.modify(|_, w| w.clk48sel().bits(0b11)) };
        }

        //
        // 2. Setup PLL
        //

        // Select PLL source
        let (clock_speed, pll_source) = if let Some(source) = self.pll_source {
            match source {
                PllSource::HSE => {
                    if let Some(hse) = &self.hse {
                        (hse.speed, source)
                    } else {
                        panic!("HSE selected as PLL source, but not enabled");
                    }
                }
                PllSource::HSI16 => (HSI, source),
                PllSource::MSI => {
                    if let Some(msi) = self.msi {
                        (msi.to_hertz().raw(), source)
                    } else {
                        panic!("MSI selected as PLL source, but not enabled");
                    }
                }
            }
        } else {
            // No specific PLL source selected, do educated guess

            // 1. HSE
            if let Some(hse) = &self.hse {
                (hse.speed, PllSource::HSE)
            }
            // 2. MSI
            else if let Some(msi) = self.msi {
                (msi.to_hertz().raw(), PllSource::MSI)
            }
            // 3. HSI as fallback
            else {
                (HSI, PllSource::HSI16)
            }
        };

        // Check if HSI should be started
        if pll_source == PllSource::HSI16 || (self.msi.is_none() && self.hse.is_none()) {
            rcc.cr.write(|w| w.hsion().set_bit());
            while rcc.cr.read().hsirdy().bit_is_clear() {}
        }

        let pllconf = if self.pll_config.is_none() {
            if let Some(sysclk) = self.sysclk {
                // Calculate PLL multiplier and create a best effort pll config, just multiply n
                let plln = (2 * sysclk) / clock_speed;

                Some(PllConfig::new(1, plln as u8, PllDivider::Div2))
            } else {
                None
            }
        } else {
            self.pll_config
        };

        let sysclk = match (self.sysclk, self.msi) {
            (Some(sysclk), _) => sysclk,
            (None, Some(msi)) => msi.to_hertz().raw(),
            (None, None) => MsiFreq::RANGE4M.to_hertz().raw(),
        };

        assert!(sysclk <= 80_000_000);

        let (hpre_bits, hpre_div) = self
            .hclk
            .map(|hclk| match sysclk / hclk {
                // From p 194 in RM0394
                0 => unreachable!(),
                1 => (0b0000, 1),
                2 => (0b1000, 2),
                3..=5 => (0b1001, 4),
                6..=11 => (0b1010, 8),
                12..=39 => (0b1011, 16),
                40..=95 => (0b1100, 64),
                96..=191 => (0b1101, 128),
                192..=383 => (0b1110, 256),
                _ => (0b1111, 512),
            })
            .unwrap_or((0b0000, 1));

        let hclk = sysclk / hpre_div;

        assert!(hclk <= sysclk);

        let (ppre1_bits, ppre1) = self
            .pclk1
            .map(|pclk1| match hclk / pclk1 {
                // From p 194 in RM0394
                0 => unreachable!(),
                1 => (0b000, 1),
                2 => (0b100, 2),
                3..=5 => (0b101, 4),
                6..=11 => (0b110, 8),
                _ => (0b111, 16),
            })
            .unwrap_or((0b000, 1));

        let pclk1: u32 = hclk / ppre1 as u32;

        assert!(pclk1 <= sysclk);

        let (ppre2_bits, ppre2) = self
            .pclk2
            .map(|pclk2| match hclk / pclk2 {
                // From p 194 in RM0394
                0 => unreachable!(),
                1 => (0b000, 1),
                2 => (0b100, 2),
                3..=5 => (0b101, 4),
                6..=11 => (0b110, 8),
                _ => (0b111, 16),
            })
            .unwrap_or((0b000, 1));

        let pclk2: u32 = hclk / ppre2 as u32;

        // RM0394 Rev 4, page 188
        // 6.2.14 Timer clock
        //
        // The timer clock frequencies are automatically defined by hardware. There are two cases:
        // 1. If the APB prescaler equals 1, the timer clock frequencies are set to the same
        // frequency as that of the APB domain.
        // 2. Otherwise, they are set to twice (×2) the frequency of the APB domain.
        let timclk1 = if ppre1 == 1 { pclk1 } else { 2 * pclk1 };
        let timclk2 = if ppre2 == 1 { pclk2 } else { 2 * pclk2 };

        assert!(pclk2 <= sysclk);

        // adjust flash wait states
        unsafe {
            flash.acr.write(|w| {
                w.latency().bits(if hclk <= 16_000_000 {
                    0b000
                } else if hclk <= 32_000_000 {
                    0b001
                } else if hclk <= 48_000_000 {
                    0b010
                } else if hclk <= 64_000_000 {
                    0b011
                } else {
                    0b100
                })
            })
        }

        let sysclk_src_bits;
        let mut msi = self.msi;
        if let Some(pllconf) = pllconf {
            // Sanity-checks per RM0394, 6.4.4 PLL configuration register (RCC_PLLCFGR)
            let r = pllconf.r.to_division_factor();
            let clock_speed = clock_speed / (pllconf.m as u32 + 1);
            let vco = clock_speed * pllconf.n as u32;
            let output_clock = vco / r;

            assert!(r <= 8); // Allowed max output divider
            assert!(pllconf.n >= 8); // Allowed min multiplier
            assert!(pllconf.n <= 86); // Allowed max multiplier
            assert!(clock_speed >= 4_000_000); // VCO input clock min
            assert!(clock_speed <= 16_000_000); // VCO input clock max
            assert!(vco >= 64_000_000); // VCO output min
            assert!(vco <= 334_000_000); // VCO output max
            assert!(output_clock <= 80_000_000); // Max output clock

            // use PLL as source
            sysclk_src_bits = 0b11;
            rcc.cr.modify(|_, w| w.pllon().clear_bit());
            while rcc.cr.read().pllrdy().bit_is_set() {
                cortex_m::asm::nop();
            }

            let pllsrc_bits = pll_source.to_pllsrc();

            rcc.pllcfgr.modify(|_, w| unsafe {
                w.pllsrc()
                    .bits(pllsrc_bits)
                    .pllm()
                    .bits(pllconf.m)
                    .pllr()
                    .bits(pllconf.r.to_bits())
                    .plln()
                    .bits(pllconf.n)
            });

            rcc.cr.modify(|_, w| w.pllon().set_bit());
            rcc.pllcfgr.modify(|_, w| w.pllren().set_bit());

            while rcc.cr.read().pllrdy().bit_is_clear() {
                cortex_m::asm::nop();
            }

            // SW: PLL selected as system clock
            rcc.cfgr.modify(|_, w| unsafe {
                w.ppre2()
                    .bits(ppre2_bits)
                    .ppre1()
                    .bits(ppre1_bits)
                    .hpre()
                    .bits(hpre_bits)
                    .sw()
                    .bits(sysclk_src_bits)
            });
        } else {
            // use MSI as fallback source for sysclk
            sysclk_src_bits = 0b00;
            if msi.is_none() {
                msi = Some(MsiFreq::RANGE4M);
            }

            // SW: MSI selected as system clock
            rcc.cfgr.write(|w| unsafe {
                w.ppre2()
                    .bits(ppre2_bits)
                    .ppre1()
                    .bits(ppre1_bits)
                    .hpre()
                    .bits(hpre_bits)
                    .sw()
                    .bits(sysclk_src_bits)
            });
        }

        while rcc.cfgr.read().sws().bits() != sysclk_src_bits {
            cortex_m::asm::nop();
        }

        //
        // 3. Shutdown unused clocks that have auto-started
        //

        // MSI always starts on reset
        if msi.is_none() {
            rcc.cr
                .modify(|_, w| w.msion().clear_bit().msipllen().clear_bit())
        }

        //
        // 4. Clock setup done!
        //

        Clocks {
            hclk: hclk.Hz(),
            lsi: lsi_used,
            lse: self.lse.is_some(),
            msi,
            hsi48: self.hsi48,
            pclk1: pclk1.Hz(),
            pclk2: pclk2.Hz(),
            ppre1,
            ppre2,
            sysclk: sysclk.Hz(),
            timclk1: timclk1.Hz(),
            timclk2: timclk2.Hz(),
            pll_source: pllconf.map(|_| pll_source),
        }
    }
}

#[derive(Debug, PartialEq)]
/// HSE Configuration
struct HseConfig {
    /// Clock speed of HSE
    speed: u32,
    /// If the clock driving circuitry is bypassed i.e. using an oscillator, not a crystal or
    /// resonator
    bypass: CrystalBypass,
    /// Clock Security System enable/disable
    css: ClockSecuritySystem,
}

#[derive(Debug, PartialEq)]
/// LSE Configuration
struct LseConfig {
    /// If the clock driving circuitry is bypassed i.e. using an oscillator, not a crystal or
    /// resonator
    bypass: CrystalBypass,
    /// Clock Security System enable/disable
    css: ClockSecuritySystem,
}

/// Crystal bypass selector
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CrystalBypass {
    /// If the clock driving circuitry is bypassed i.e. using an oscillator
    Enable,
    /// If the clock driving circuitry is not bypassed i.e. using a crystal or resonator
    Disable,
}

/// Clock Security System (CSS) selector
///
/// When this is enabled on HSE it will fire of the NMI interrupt on failure and for the LSE the
/// MCU will be woken if in Standby and then the LSECSS interrupt will fire. See datasheet on how
/// to recover for CSS failures.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ClockSecuritySystem {
    /// Enable the clock security system to detect clock failures
    Enable,
    /// Leave the clock security system disabled
    Disable,
}

#[derive(Clone, Copy, Debug)]
/// PLL output divider options
pub enum PllDivider {
    /// Divider PLL output by 2
    Div2 = 0b00,
    /// Divider PLL output by 4
    Div4 = 0b01,
    /// Divider PLL output by 6
    Div6 = 0b10,
    /// Divider PLL output by 8
    Div8 = 0b11,
}

impl PllDivider {
    #[inline(always)]
    fn to_bits(self) -> u8 {
        self as u8
    }

    #[inline(always)]
    fn to_division_factor(self) -> u32 {
        match self {
            Self::Div2 => 2,
            Self::Div4 => 4,
            Self::Div6 => 6,
            Self::Div8 => 8,
        }
    }
}

#[derive(Clone, Copy, Debug)]
/// PLL Configuration
pub struct PllConfig {
    // Main PLL division factor
    m: u8,
    // Main PLL multiplication factor
    n: u8,
    // Main PLL division factor for PLLCLK (system clock)
    r: PllDivider,
}

impl PllConfig {
    /// Create a new PLL config from manual settings
    ///
    /// PLL output = ((SourceClk / input_divider) * multiplier) / output_divider
    pub fn new(input_divider: u8, multiplier: u8, output_divider: PllDivider) -> Self {
        assert!(input_divider > 0);

        PllConfig {
            m: input_divider - 1,
            n: multiplier,
            r: output_divider,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
/// PLL Source
pub enum PllSource {
    /// Multi-speed internal clock
    MSI,
    /// High-speed internal clock
    HSI16,
    /// High-speed external clock
    HSE,
}

impl PllSource {
    fn to_pllsrc(self) -> u8 {
        match self {
            Self::MSI => 0b01,
            Self::HSI16 => 0b10,
            Self::HSE => 0b11,
        }
    }
}

/// Frozen clock frequencies
///
/// The existence of this value indicates that the clock configuration can no longer be changed
#[derive(Clone, Copy, Debug)]
pub struct Clocks {
    hclk: Hertz,
    hsi48: bool,
    msi: Option<MsiFreq>,
    lsi: bool,
    lse: bool,
    pclk1: Hertz,
    pclk2: Hertz,
    ppre1: u8,
    ppre2: u8,
    sysclk: Hertz,
    timclk1: Hertz,
    timclk2: Hertz,
    pll_source: Option<PllSource>,
}

impl Clocks {
    /// Returns the frequency of the AHB
    pub fn hclk(&self) -> Hertz {
        self.hclk
    }

    /// Returns status of HSI48
    pub fn hsi48(&self) -> bool {
        self.hsi48
    }

    // Returns the status of the MSI
    pub fn msi(&self) -> Option<MsiFreq> {
        self.msi
    }

    /// Returns status of the LSI
    pub fn lsi(&self) -> bool {
        self.lsi
    }

    // Return the status of the LSE
    pub fn lse(&self) -> bool {
        self.lse
    }

    /// Returns the frequency of the APB1
    pub fn pclk1(&self) -> Hertz {
        self.pclk1
    }

    /// Returns the frequency of the APB2
    pub fn pclk2(&self) -> Hertz {
        self.pclk2
    }

    /// Get which source is being used for PLL
    pub fn pll_source(&self) -> Option<PllSource> {
        self.pll_source
    }

    // TODO remove `allow`
    #[allow(dead_code)]
    pub(crate) fn ppre1(&self) -> u8 {
        self.ppre1
    }
    // TODO remove `allow`
    #[allow(dead_code)]
    pub(crate) fn ppre2(&self) -> u8 {
        self.ppre2
    }

    /// Returns the system (core) frequency
    pub fn sysclk(&self) -> Hertz {
        self.sysclk
    }

    /// Returns the frequency for timers on APB1
    pub fn timclk1(&self) -> Hertz {
        self.timclk1
    }

    /// Returns the frequency for timers on APB2
    pub fn timclk2(&self) -> Hertz {
        self.timclk2
    }
}
