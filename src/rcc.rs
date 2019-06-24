//! Reset and Clock Control
#![deny(missing_docs)]

use crate::pwr::VoltageScale as Voltage;
use crate::stm32::rcc::cfgr::SWW;
use crate::stm32::rcc::cfgr::TIMPREW;
use crate::stm32::rcc::d1ccipr::CKPERSELW;
use crate::stm32::rcc::d1cfgr::HPREW;
use crate::stm32::rcc::pllckselr::PLLSRCW;
use crate::stm32::{rcc, RCC, SYSCFG};
use crate::time::Hertz;

/// This module configures the RCC unit to provide set frequencies for
/// the input to the SCGU `sys_ck`, the AMBA High-performace Busses
/// and Advanced eXtensible Interface bus `hclk`, the AMBA Peripheral
/// Busses `pclkN` and the periperal clock `per_ck`.
///
/// Check Fig 46 "Core and bus clock generation" in the reference
/// manual for information (p 336).
///
/// HSI is 64 MHz.
///

/// Extension trait that constrains the `RCC` peripheral
pub trait RccExt {
    /// Constrains the `RCC` peripheral so it plays nicely with the
    /// other abstractions
    fn constrain(self) -> Rcc;
}

impl RccExt for RCC {
    fn constrain(self) -> Rcc {
        Rcc {
            config: Config {
                hse: None,
                sys_ck: None,
                per_ck: None,
                rcc_hclk: None,
                rcc_pclk1: None,
                rcc_pclk2: None,
                rcc_pclk3: None,
                rcc_pclk4: None,
            },
            rb: self,
        }
    }
}

/// Constrained RCC peripheral
///
/// Generated by calling `constrain` on the PAC's RCC peripheral.
///
/// ```rust
/// let dp = stm32::Peripherals::take().unwrap();
/// let rcc = dp.RCC.constrain();
/// ```
pub struct Rcc {
    config: Config,
    pub(crate) rb: RCC,
}

/// Core Clock Distribution and Reset (CCDR)
///
/// Generated when the RCC is frozen. The configuration of the Sys_Ck
/// `sys_ck`, CPU Clock `c_ck`, AXI peripheral clock `aclk`, AHB
/// clocks `hclk`, APB clocks `pclkN` and PLL outputs `pllN_X_ck` are
/// frozen. However the distribution of some clocks may still be
/// modified and peripherals enabled / reset by passing this object
/// to other implementations in this stack.
pub struct Ccdr {
    /// A record of the frozen core clock frequencies
    pub clocks: CoreClocks,
    /// AMBA High-performance Bus (AHB1) registers
    pub ahb1: AHB1,
    /// AMBA High-performance Bus (AHB3) registers
    pub ahb3: AHB3,
    /// AMBA High-performance Bus (AHB4) registers
    pub ahb4: AHB4,
    /// Advanced Peripheral Bus 1 (APB1) registers
    pub apb1: APB1,
    /// Advanced Peripheral Bus 2 (APB2) registers
    pub apb2: APB2,
    /// Advanced Peripheral Bus 3 (APB3) registers
    pub apb3: APB3,
    /// Advanced Peripheral Bus 4 (APB4) registers
    pub apb4: APB4,
    /// RCC Domain 3 Kernel Clock Configuration Register
    pub d3ccipr: D3CCIPR,
    // Yes, it lives (locally)! We retain the right to switch most
    // PKSUs on the fly, to fine-tune PLL frequencies, and to enable /
    // reset peripherals.
    //
    // TODO: Remove this once all permitted RCC register accesses
    // after freeze are enumerated in this struct
    pub(crate) rb: RCC,
}

/// AMBA High-performance Bus (AHB) peripheral registers
pub struct AHB1 {
    _0: (),
}

impl AHB1 {
    #[allow(unused)]
    pub(crate) fn enr(&mut self) -> &rcc::AHB1ENR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).ahb1enr }
    }

    #[allow(unused)]
    pub(crate) fn rstr(&mut self) -> &rcc::AHB1RSTR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).ahb1rstr }
    }

}
/// AMBA High-performance Bus (AHB) peripheral registers
pub struct AHB3 {
    _0: (),
}

impl AHB3 {
    #[allow(unused)]
    pub(crate) fn enr(&mut self) -> &rcc::AHB3ENR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).ahb3enr }
    }

    #[allow(unused)]
    pub(crate) fn rstr(&mut self) -> &rcc::AHB3RSTR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).ahb3rstr }
    }
}

/// AMBA High-performance Bus (AHB) peripheral registers
pub struct AHB4 {
    _0: (),
}

impl AHB4 {
    #[allow(unused)]
    pub(crate) fn enr(&mut self) -> &rcc::AHB4ENR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).ahb4enr }
    }

    #[allow(unused)]
    pub(crate) fn rstr(&mut self) -> &rcc::AHB4RSTR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).ahb4rstr }
    }
}

/// Advanced Peripheral Bus 1 (APB1) peripheral registers
pub struct APB1 {
    _0: (),
}

impl APB1 {
    #[allow(unused)]
    pub(crate) fn lenr(&mut self) -> &rcc::APB1LENR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).apb1lenr }
    }

    #[allow(unused)]
    pub(crate) fn lrstr(&mut self) -> &rcc::APB1LRSTR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).apb1lrstr }
    }
}

/// Advanced Peripheral Bus 2 (APB2) peripheral registers
pub struct APB2 {
    _0: (),
}

impl APB2 {
    #[allow(unused)]
    pub(crate) fn enr(&mut self) -> &rcc::APB2ENR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).apb2enr }
    }

    #[allow(unused)]
    pub(crate) fn rstr(&mut self) -> &rcc::APB2RSTR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).apb2rstr }
    }
}

/// Advanced Peripheral Bus 3 (APB3) peripheral registers
pub struct APB3 {
    _0: (),
}

impl APB3 {
    #[allow(unused)]
    pub(crate) fn enr(&mut self) -> &rcc::APB3ENR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).apb3enr }
    }

    #[allow(unused)]
    pub(crate) fn rstr(&mut self) -> &rcc::APB3RSTR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).apb3rstr }
    }
}

/// Advanced Peripheral Bus 4 (APB4) peripheral registers
pub struct APB4 {
    _0: (),
}

impl APB4 {
    #[allow(unused)]
    pub(crate) fn enr(&mut self) -> &rcc::APB4ENR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).apb4enr }
    }

    #[allow(unused)]
    pub(crate) fn rstr(&mut self) -> &rcc::APB4RSTR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).apb4rstr }
    }
}

/// RCC Domain 3 Kernel Clock Configuration Register
pub struct D3CCIPR {
    _0: (),
}

impl D3CCIPR {
    pub(crate) fn kernel_ccip(&mut self) -> &rcc::D3CCIPR {
        unsafe {&(*RCC::ptr()).d3ccipr}
    }
}

const HSI: u32 = 64_000_000; // Hz
const CSI: u32 = 4_000_000; // Hz

/// Configuration of the core clocks
pub struct Config {
    hse: Option<u32>,
    sys_ck: Option<u32>,
    per_ck: Option<u32>,
    rcc_hclk: Option<u32>,
    rcc_pclk1: Option<u32>,
    rcc_pclk2: Option<u32>,
    rcc_pclk3: Option<u32>,
    rcc_pclk4: Option<u32>,
}

/// Setter defintion for pclk 1 - 4
macro_rules! pclk_setter {
    ($($name:ident: $pclk:ident,)+) => {
        $(
            /// Set the peripheral clock frequency for APB
            /// peripherals.
            pub fn $name<F>(mut self, freq: F) -> Self
            where
                F: Into<Hertz>,
            {
                self.config.$pclk = Some(freq.into().0);
                self
            }
        )+
    };
}

impl Rcc {
    /// Uses HSE (external oscillator) instead of HSI (internal RC
    /// oscillator) as the clock source. Will result in a hang if an
    /// external oscillator is not connected or it fails to start.
    pub fn use_hse<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.config.hse = Some(freq.into().0);
        self
    }

    /// Set input frequency to the SCGU
    pub fn sys_ck<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.config.sys_ck = Some(freq.into().0);
        self
    }

    /// Set input frequency to the SCGU - ALIAS
    pub fn sysclk<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.config.sys_ck = Some(freq.into().0);
        self
    }

    /// Set peripheral clock frequency
    pub fn per_ck<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.config.per_ck = Some(freq.into().0);
        self
    }

    /// Set the peripheral clock frequency for AHB and AXI
    /// peripherals. There are several gated versions `rcc_hclk[1-4]`
    /// for different power domains, but they are all the same frequency
    pub fn hclk<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.config.rcc_hclk = Some(freq.into().0);
        self
    }

    pclk_setter! {
        pclk1: rcc_pclk1,
        pclk2: rcc_pclk2,
        pclk3: rcc_pclk3,
        pclk4: rcc_pclk4,
    }
}

/// Divider calculator for pclk 1 - 4
///
/// Also calulate tim[xy]_ker_clk if there are timers on this bus
macro_rules! ppre_calculate {
    ($(($ppre:ident, $bits:ident): ($self: ident, $hclk: ident,
                                    $pclk: ident, $max: ident
                                    $(,$rcc_tim_ker_clk:ident, $timpre:ident)*),)+) => {
        $(
            // Get intended rcc_pclkN frequency
            let $pclk: u32 = $self.config
                .$pclk
                .unwrap_or_else(|| core::cmp::min($max, $hclk / 2));

            // Calculate suitable divider
            let ($bits, $ppre) = match ($hclk + $pclk - 1) / $pclk
            {
                0 => unreachable!(),
                1 => (0b000, 1 as u8),
                2 => (0b100, 2),
                3..=5 => (0b101, 4),
                6..=11 => (0b110, 8),
                _ => (0b111, 16),
            };

            // Calculate real APBn clock
            let $pclk = $hclk / u32::from($ppre);

            // Check in range
            assert!($pclk <= $max);

            $(
                let $rcc_tim_ker_clk = match ($bits, &$timpre)
                {
                    (0b101, TIMPREW::DEFAULTX2) => $hclk / 2,
                    (0b110, TIMPREW::DEFAULTX4) => $hclk / 2,
                    (0b110, TIMPREW::DEFAULTX2) => $hclk / 4,
                    (0b111, TIMPREW::DEFAULTX4) => $hclk / 4,
                    (0b111, TIMPREW::DEFAULTX2) => $hclk / 8,
                    _ => $hclk,
                };
            )*
        )+
    };
}

impl Rcc {
    /// PLL1 Setup
    /// Returns (Option(pll1_p_ck), Option(pll1_q_ck), Option(pll1_r_ck))
    fn pll_setup(
        &self,
        rcc: &RCC,
    ) -> (Option<Hertz>, Option<Hertz>, Option<Hertz>) {
        // Compare available with wanted clocks
        let srcclk = self.config.hse.unwrap_or(HSI);
        let sys_ck = self.config.sys_ck.unwrap_or(srcclk);

        // The requested system clock is not the immediately available
        // HSE/HSI clock. Perhaps there are other ways of obtaining
        // the requested system clock (such as `HSIDIV`) but we will
        // ignore those for now and use PLL1.
        if sys_ck != srcclk {
            let pllsrc = if self.config.hse.is_some() {
                PLLSRCW::HSE
            } else {
                PLLSRCW::HSI
            };

            assert!(srcclk > 0);

            // Currently we use the Medium Range VCO with 1 - 2 MHz
            // input

            // Input divisor, resulting in a reference clock in the
            // range 1 to 2 MHz. Choose the highest reference clock
            let pll1_m = (srcclk + 1_999_999) / 2_000_000;

            assert!(pll1_m < 64);

            // Calculate resulting reference clock
            let ref1_ck = srcclk / pll1_m;
            assert!(ref1_ck >= 1_000_000 && ref1_ck <= 2_000_000);

            // VCO output frequency. Choose the highest VCO frequency
            let pll1_vco_min = 150_000_000;
            let pll1_vco_max = 420_000_000;
            let pll1_p = if sys_ck > pll1_vco_max / 2 {
                1
            } else {
                ((pll1_vco_max / sys_ck) | 1) - 1 // Must be even or unity
            };
            let vco1_ck = sys_ck * pll1_p;

            assert!(pll1_p <= 128);
            assert!(vco1_ck >= pll1_vco_min);
            assert!(vco1_ck <= pll1_vco_max);

            // Feedback divider. Integer only
            let pll1_n = vco1_ck / ref1_ck;

            assert!(pll1_n >= 4);
            assert!(pll1_n <= 512);

            // Calculate PLL P output clock
            let pll1_p_ck = ref1_ck * pll1_n / pll1_p;

            // Calculate PLL Q output clock - same as P
            let pll1_q = pll1_p;
            let pll1_q_ck = ref1_ck * pll1_n / pll1_q;

            // Write dividers
            rcc.pllckselr.modify(|_, w| {
                w.pllsrc()
                    .variant(pllsrc) // hse
                    .divm1()
                    .bits(pll1_m as u8) // ref prescaler
            });
            rcc.pll1divr.modify(|_, w| unsafe {
                w.divq1()
                    .bits((pll1_q - 1) as u8)
                    .divp1()
                    .bits((pll1_p - 1) as u8)
                    .divn1()
                    .bits((pll1_n - 1) as u16)
            });

            // Configure PLL
            rcc.pllcfgr.write(|w| {
                w.pll1fracen()
                    .reset() // No FRACN
                    .pll1vcosel()
                    .medium_vco() // 150 - 420MHz Medium VCO
                    .pll1rge()
                    .range1() // ref1_ck is 1 - 2 MHz
                    .divp1en()
                    .enabled()
                    .divq1en()
                    .enabled()
            });

            (Some(Hertz(pll1_p_ck)), Some(Hertz(pll1_q_ck)), None)
        } else {
            (None, None, None)
        }
    }

    fn flash_setup(rcc_aclk: u32, vos: Voltage) {
        use crate::stm32::FLASH;
        let rcc_aclk_mhz = rcc_aclk / 1_000_000;

        // See RM0433 Table 13. FLASH recommended number of wait
        // states and programming delay
        let (wait_states, progr_delay) = match vos {
            // VOS 1 range VCORE 1.15V - 1.26V
            Voltage::Scale0 | Voltage::Scale1 => match rcc_aclk_mhz {
                0..=69 => (0, 0),
                70..=139 => (1, 1),
                140..=184 => (2, 1),
                185..=209 => (2, 2),
                210..=224 => (3, 2),
                _ => (7, 3),
            },
            // VOS 2 range VCORE 1.05V - 1.15V
            Voltage::Scale2 => match rcc_aclk_mhz {
                0..=54 => (0, 0),
                55..=109 => (1, 1),
                110..=164 => (2, 1),
                165..=224 => (3, 2),
                225 => (4, 2),
                _ => (7, 3),
            },
            // VOS 3 range VCORE 0.95V - 1.05V
            Voltage::Scale3 => match rcc_aclk_mhz {
                0..=44 => (0, 0),
                45..=89 => (1, 1),
                90..=134 => (2, 1),
                135..=179 => (3, 2),
                180..=224 => (4, 2),
                _ => (7, 3),
            },
            _ => (7, 3),
        };

        let flash = unsafe { &(*FLASH::ptr()) };
        // Adjust flash wait states
        flash.acr.write(|w| unsafe {
            w.wrhighfreq().bits(progr_delay).latency().bits(wait_states)
        });
        while flash.acr.read().latency().bits() != wait_states {}
    }

    /// Freeze the core clocks, returning a Core Clocks Distribution
    /// and Reset (CCDR) object.
    ///
    /// `syscfg` is required to enable the I/O compensation cell.
    pub fn freeze(self, vos: Voltage, syscfg: &SYSCFG) -> Ccdr {
        let rcc = &self.rb;

        // sys_ck from PLL if needed, else HSE or HSI
        let srcclk = self.config.hse.unwrap_or(HSI);
        let (pll1_p_ck, pll1_q_ck, pll1_r_ck) = self.pll_setup(rcc);
        let sys_ck = pll1_p_ck.unwrap_or(Hertz(srcclk));

        // hsi_ck = HSI. This routine does not support HSIDIV != 1. To
        // do so it would need to ensure all PLLxON bits are clear
        // before changing the value of HSIDIV
        let hsi = HSI;
        assert!(rcc.cr.read().hsion().is_on());
        assert!(rcc.cr.read().hsidiv().is_div1());

        // csi_ck = CSI.
        let csi = CSI;

        // per_ck from HSI by default
        let (per_ck, ckpersel) =
            match (self.config.per_ck == self.config.hse, self.config.per_ck) {
                (true, Some(hse)) => (hse, CKPERSELW::HSE), // HSE
                (_, Some(CSI)) => (csi, CKPERSELW::CSI),    // CSI
                _ => (hsi, CKPERSELW::HSI),                 // HSI
            };

        // D1 Core Prescaler
        // Set to 1
        let d1cpre_bits = 0;
        let d1cpre_div = 1;
        let sys_d1cpre_ck = sys_ck.0 / d1cpre_div;

        // Timer prescaler selection
        let timpre = TIMPREW::DEFAULTX2;

        // Refer to part datasheet "General operating conditions"
        // table for (rev V). We do not assert checks for earlier
        // revisions which may have lower limits.
        #[cfg(any(
            feature = "stm32h742",
            feature = "stm32h743",
            feature = "stm32h753",
            feature = "stm32h750"
        ))]
        let (sys_d1cpre_ck_max, rcc_hclk_max, pclk_max) = match vos {
            Voltage::Scale0 => (480_000_000, 240_000_000, 120_000_000),
            Voltage::Scale1 => (400_000_000, 200_000_000, 100_000_000),
            Voltage::Scale2 => (300_000_000, 150_000_000, 75_000_000),
            _ => (200_000_000, 100_000_000, 50_000_000),
        };

        // Check resulting sys_d1cpre_ck
        assert!(sys_d1cpre_ck <= sys_d1cpre_ck_max);

        // Get ideal AHB clock
        let rcc_hclk = self.config.rcc_hclk.unwrap_or(sys_d1cpre_ck / 2);
        assert!(rcc_hclk <= rcc_hclk_max);

        // Estimate divisor
        let (hpre_bits, hpre_div) =
            match (sys_d1cpre_ck + rcc_hclk - 1) / rcc_hclk {
                0 => unreachable!(),
                1 => (HPREW::DIV1, 1),
                2 => (HPREW::DIV2, 2),
                3..=5 => (HPREW::DIV4, 4),
                6..=11 => (HPREW::DIV8, 8),
                12..=39 => (HPREW::DIV16, 16),
                40..=95 => (HPREW::DIV64, 64),
                96..=191 => (HPREW::DIV128, 128),
                192..=383 => (HPREW::DIV256, 256),
                _ => (HPREW::DIV512, 512),
            };

        // Calculate real AXI and AHB clock
        let rcc_hclk = sys_d1cpre_ck / hpre_div;
        assert!(rcc_hclk <= rcc_hclk_max);

        // Calculate ppreN dividers and real rcc_pclkN frequencies
        ppre_calculate! {
            (ppre1, ppre1_bits):
                (self, rcc_hclk, rcc_pclk1, pclk_max, rcc_timx_ker_ck, timpre),
            (ppre2, ppre2_bits):
                (self, rcc_hclk, rcc_pclk2, pclk_max, rcc_timy_ker_ck, timpre),
            (ppre3, ppre3_bits): (self, rcc_hclk, rcc_pclk3, pclk_max),
            (ppre4, ppre4_bits): (self, rcc_hclk, rcc_pclk4, pclk_max),
        }

        // Flash setup
        Self::flash_setup(sys_d1cpre_ck, vos);

        // Ensure CSI is on and stable
        rcc.cr.modify(|_, w| w.csion().on());
        while rcc.cr.read().csirdy().is_not_ready() {}

        // HSE
        let hse_ck = match self.config.hse {
            Some(hse) => {
                // Ensure HSE is on and stable
                rcc.cr.modify(|_, w| w.hseon().on().hsebyp().not_bypassed());
                while rcc.cr.read().hserdy().is_not_ready() {}

                Some(Hertz(hse))
            }
            None => None,
        };

        // PLL1
        if pll1_p_ck.is_some() {
            // Enable PLL and wait for it to stabilise
            rcc.cr.modify(|_, w| w.pll1on().on());
            while rcc.cr.read().pll1rdy().is_not_ready() {}
        }

        // Core Prescaler / AHB Prescaler / APB3 Prescaler
        rcc.d1cfgr.modify(|_, w| unsafe {
            w.d1cpre()
                .bits(d1cpre_bits)
                .d1ppre() // D1 contains APB3
                .bits(ppre3_bits)
                .hpre()
                .variant(hpre_bits)
        });
        // Ensure core prescaler value is valid before future lower
        // core voltage
        while rcc.d1cfgr.read().d1cpre().bits() != d1cpre_bits {}

        // APB1 / APB2 Prescaler
        rcc.d2cfgr.modify(|_, w| unsafe {
            w.d2ppre1() // D2 contains APB1
                .bits(ppre1_bits)
                .d2ppre2() // D2 also contains APB2
                .bits(ppre2_bits)
        });

        // APB4 Prescaler
        rcc.d3cfgr.modify(|_, w| unsafe {
            w.d3ppre() // D3 contains APB4
                .bits(ppre4_bits)
        });

        // Peripheral Clock (per_ck)
        rcc.d1ccipr.modify(|_, w| w.ckpersel().variant(ckpersel));

        // Set timer clocks prescaler setting
        rcc.cfgr.modify(|_, w| w.timpre().variant(timpre));

        // Select system clock source
        let swbits = match (pll1_p_ck.is_some(), self.config.hse.is_some()) {
            (true, _) => SWW::PLL1 as u8,
            (false, true) => SWW::HSE as u8,
            _ => SWW::HSI as u8,
        };
        rcc.cfgr.modify(|_, w| unsafe { w.sw().bits(swbits) });
        while rcc.cfgr.read().sws().bits() != swbits {}

        // IO compensation cell - Requires CSI clock and SYSCFG
        assert!(rcc.cr.read().csirdy().is_ready());
        rcc.apb4enr.modify(|_, w| w.syscfgen().enabled());

        // Enable the compensation cell, using back-bias voltage code
        // provide by the cell.
        syscfg.cccsr.modify(|_, w| {
            w.en().set_bit().cs().clear_bit().hslv().clear_bit()
        });
        while syscfg.cccsr.read().ready().bit_is_clear() {}

        // Return frozen clock configuration
        Ccdr {
            ahb1: AHB1 { _0: () },
            ahb3: AHB3 { _0: () },
            ahb4: AHB4 { _0: () },
            apb1: APB1 { _0: () },
            apb2: APB2 { _0: () },
            apb3: APB3 { _0: () },
            apb4: APB4 { _0: () },
            clocks: CoreClocks {
                hclk: Hertz(rcc_hclk),
                pclk1: Hertz(rcc_pclk1),
                pclk2: Hertz(rcc_pclk2),
                pclk3: Hertz(rcc_pclk3),
                pclk4: Hertz(rcc_pclk4),
                ppre1,
                ppre2,
                ppre3,
                ppre4,
                csi_ck: Some(Hertz(csi)),
                hsi_ck: Some(Hertz(hsi)),
                per_ck: Some(Hertz(per_ck)),
                hse_ck,
                pll1_p_ck,
                pll1_q_ck,
                pll1_r_ck,
                pll2_p_ck: None,
                pll2_q_ck: None,
                pll2_r_ck: None,
                pll3_p_ck: None,
                pll3_q_ck: None,
                pll3_r_ck: None,
                timx_ker_ck: Hertz(rcc_timx_ker_ck),
                timy_ker_ck: Hertz(rcc_timy_ker_ck),
                sys_ck,
                c_ck: Hertz(sys_d1cpre_ck),
            },
            d3ccipr: D3CCIPR { _0: () },
            rb: self.rb,
        }
    }
}

/// Frozen core clock frequencies
///
/// The existence of this value indicates that the core clock
/// configuration can no longer be changed
#[derive(Clone, Copy)]
pub struct CoreClocks {
    hclk: Hertz,
    pclk1: Hertz,
    pclk2: Hertz,
    pclk3: Hertz,
    pclk4: Hertz,
    ppre1: u8,
    ppre2: u8,
    ppre3: u8,
    ppre4: u8,
    csi_ck: Option<Hertz>,
    hsi_ck: Option<Hertz>,
    per_ck: Option<Hertz>,
    hse_ck: Option<Hertz>,
    pll1_p_ck: Option<Hertz>,
    pll1_q_ck: Option<Hertz>,
    pll1_r_ck: Option<Hertz>,
    pll2_p_ck: Option<Hertz>,
    pll2_q_ck: Option<Hertz>,
    pll2_r_ck: Option<Hertz>,
    pll3_p_ck: Option<Hertz>,
    pll3_q_ck: Option<Hertz>,
    pll3_r_ck: Option<Hertz>,
    timx_ker_ck: Hertz,
    timy_ker_ck: Hertz,
    sys_ck: Hertz,
    c_ck: Hertz,
}

/// Getters for pclk and ppre
macro_rules! pclk_ppre_getter {
    ($(($pclk:ident, $ppre:ident),)+) => {
        $(
            /// Returns the frequency of the APBn
            pub fn $pclk(&self) -> Hertz {
                self.$pclk
            }
            /// Returns the prescaler of the APBn
            pub fn $ppre(&self) -> u8 {
                self.$ppre
            }
        )+
    };
}

/// Getters for optional clocks
macro_rules! optional_ck_getter {
    ($($opt_ck:ident,)+) => {
        $(
            /// Returns the frequency of optional clock $opt_ck
            pub fn $opt_ck(&self) -> Option<Hertz> {
                self.$opt_ck
            }
        )+
    };
}

/// Getters for pll clocks
macro_rules! pll_getter {
    ($($pll_ck:ident,)+) => {
        $(
            /// Returns the frequency of the PLLx output
            pub fn $pll_ck(&self) -> Option<Hertz> {
                self.$pll_ck
            }
        )+
    };
}

impl CoreClocks {
    /// Returns the frequency of AHB1,2,3 busses
    pub fn hclk(&self) -> Hertz {
        self.hclk
    }

    /// Returns the frequency of the AXI bus
    pub fn aclk(&self) -> Hertz {
        self.hclk // Same as HCLK
    }

    pclk_ppre_getter! {
        (pclk1, ppre1),
        (pclk2, ppre2),
        (pclk3, ppre3),
        (pclk4, ppre4),
    }

    optional_ck_getter! {
        csi_ck,
        hsi_ck,
        per_ck,
        hse_ck,
    }

    pll_getter! {
        pll1_p_ck,
        pll1_q_ck,
        pll1_r_ck,
        pll2_p_ck,
        pll2_q_ck,
        pll2_r_ck,
        pll3_p_ck,
        pll3_q_ck,
        pll3_r_ck,
    }

    /// Returns the input frequency to the SCGU
    pub fn sys_ck(&self) -> Hertz {
        self.sys_ck
    }

    /// Returns the input frequency to the SCGU - ALIAS
    pub fn sysclk(&self) -> Hertz {
        self.sys_ck
    }

    /// Returns the CK_INT frequency for timers on APB1
    pub fn timx_ker_ck(&self) -> Hertz {
        self.timx_ker_ck
    }

    /// Returns the CK_INT frequency for timers on APB2
    pub fn timy_ker_ck(&self) -> Hertz {
        self.timy_ker_ck
    }

    /// Returns the core frequency
    pub fn c_ck(&self) -> Hertz {
        self.c_ck
    }
}
