//! # Serial Audio Interface - Inter-IC Sound
//!
//! Inter-IC Sound.
//!
//! ```
//! let d1 = gpioc.pc1.into_alternate_af2();
//! let ck1 = gpioe.pe2.into_alternate_af2();
//! let pins = (ck1, d1);
//!
//! // Configure SAI for PDM mode
//! let mut sai = dp.SAI1.pdm((ck1, d1), 1_024.khz(), ccdr.periperhal.SAI1, &ccdr.clocks);
//!
//! let _ = block!(sai.read_data()).unwrap();
//! ```
#![allow(unused_imports)]

use core::convert::TryInto;

use crate::rcc::{rec, CoreClocks, ResetEnable};
use crate::sai::{GetClkSAI, Sai, SaiChannel, INTERFACE};
use crate::stm32;
use crate::stm32::{SAI1, SAI4};
use crate::time::Hertz;

use crate::Never;

// use crate::gpio::gpiob::PB2;
// use crate::gpio::gpioc::PC1;
// use crate::gpio::gpiod::PD6;
use crate::gpio::gpioe::{PE2, PE3, PE4, PE5, PE6};
use crate::gpio::gpiof::{PF6, PF7, PF8, PF9};
// use crate::gpio::gpiof::PG7;
use crate::gpio::{Alternate, AF6};

use embedded_hal::i2s::FullDuplex;

const NUM_SLOTS: u8 = 16;

pub enum I2SMode {
    MasterTx = 0b00,
    MasterRx = 0b01,
    SlaveTx = 0b10,
    SlaveRx = 0b11,
}

#[derive(Copy, Clone)]
pub enum I2SBitRate {
    BITS_8 = 0b001,
    BITS_10 = 0b010,
    BITS_16 = 0b100,
    BITS_20 = 0b101,
    BITS_24 = 0b110,
    BITS_32 = 0b111,
}

#[derive(Copy, Clone)]
enum I2SSlotSize {
    BITS_16 = 0b01,
    BITS_32 = 0b10,
}

#[derive(Copy, Clone)]
pub enum I2SProtocol {
    LSB,
    MSB,
}

#[derive(Copy, Clone)]
pub enum I2SSynchronization {
    MASTER = 0b00,
    INTERNAL = 0b01,
    EXTERNAL = 0b10,
}

#[derive(Copy, Clone)]
pub enum I2SError {
    FUBAR,
}

#[derive(Copy, Clone)]
pub enum I2SOverSampling {
    Enabled = 2,
    Disabled = 1,
}

pub trait I2SPinsChA<SAI> {}

pub trait I2SPinsChB<SAI> {}

pub trait I2SPinMclkA<SAI> {}
pub trait I2SPinMclkB<SAI> {}
pub trait I2SPinSckA<SAI> {}
pub trait I2SPinSckB<SAI> {}
pub trait I2SPinFsA<SAI> {}
pub trait I2SPinFsB<SAI> {}
pub trait I2SPinSdA<SAI> {}
pub trait I2SPinSdB<SAI> {}

impl<SAI, MCLK, SCK, FS, SD1, SD2> I2SPinsChA<SAI>
    for (MCLK, SCK, FS, SD1, Option<SD2>)
where
    MCLK: I2SPinMclkA<SAI>,
    SCK: I2SPinSckA<SAI>,
    FS: I2SPinFsA<SAI>,
    SD1: I2SPinSdA<SAI>,
    SD2: I2SPinSdB<SAI>,
{
}

impl<SAI, MCLK, SCK, FS, SD1, SD2> I2SPinsChB<SAI>
    for (MCLK, SCK, FS, SD1, Option<SD2>)
where
    MCLK: I2SPinMclkB<SAI>,
    SCK: I2SPinSckB<SAI>,
    FS: I2SPinFsB<SAI>,
    SD1: I2SPinSdB<SAI>,
    SD2: I2SPinSdA<SAI>,
{
}

/*
RM0433 Rev 7 Reference Manual
pg 2257
An I/O line controller manages a set of 4 dedicated pins (SD, SCK, FS, MCLK) for a given
audio block in the SAI. Some of these pins can be shared if the two subblocks are declared
as synchronous to leave some free to be used as general purpose I/Os. The MCLK pin can
be output, or not, depending on the application, the decoder requirement and whether the
audio block is configured as the master.
If one SAI is configured to operate synchronously with another one, even more I/Os can be
freed (except for pins SD_x).
*/
/*
DS12556 Rev 5
STM32H750VB STM32H750ZB
STM32H750IB STM32H750XB

SAI1A Pins
PE2 AF6 SAI1_MCLK_A
PG7 AF6 SAI1_MCLK_A

PE5 AF6 SAI1_SCK_A

PE4 AF6 SAI1_FS_A

PB2 AF6 SAI1_SD_A
PC1 AF6 SAI1_SD_A
PD6 AF6 SAI1_SD_A
PE6 AF6 SAI1_SD_A

SAI1AB
PF7 AF6 SAI1_MCLK_B

PF8 AF6 SAI1_SCK_B

PF9 AF6 SAI1_FS_B

PE3 AF6 SAI1_SD_B
PF6 AF6 SAI1_SD_B

TODO
-Pins for SAI2-4

*/

impl I2SPinMclkA<SAI1> for PE2<Alternate<AF6>> {}
impl I2SPinSckA<SAI1> for PE5<Alternate<AF6>> {}
impl I2SPinFsA<SAI1> for PE4<Alternate<AF6>> {}
impl I2SPinSdA<SAI1> for PE6<Alternate<AF6>> {}

impl I2SPinMclkB<SAI1> for PF7<Alternate<AF6>> {}
impl I2SPinSckB<SAI1> for PF8<Alternate<AF6>> {}
impl I2SPinFsB<SAI1> for PF9<Alternate<AF6>> {}
impl I2SPinSdB<SAI1> for PE3<Alternate<AF6>> {}
/// I2S Interface
pub struct I2S {
    using_dma: bool,
}
impl INTERFACE for I2S {}

/// Trait to extend SAI periperhals
pub trait SaiI2sExt<SAI>: Sized {
    type Rec: ResetEnable;
    fn i2s_ch_a<PINS, T>(
        self,
        _pins: PINS,
        audio_freq: T,
        bit_rate: I2SBitRate,
        prec: Self::Rec,
        clocks: &CoreClocks,
    ) -> Sai<SAI, I2S>
    where
        PINS: I2SPinsChA<Self>,
        T: Into<Hertz>;
    fn i2s_ch_b<PINS, T>(
        self,
        _pins: PINS,
        audio_freq: T,
        bit_rate: I2SBitRate,
        prec: Self::Rec,
        clocks: &CoreClocks,
    ) -> Sai<SAI, I2S>
    where
        PINS: I2SPinsChB<Self>,
        T: Into<Hertz>;
}

impl SaiI2sExt<SAI1> for SAI1 {
    type Rec = rec::Sai1;
    fn i2s_ch_a<PINS, T>(
        self,
        _pins: PINS,
        audio_freq: T,
        bit_rate: I2SBitRate,
        prec: rec::Sai1,
        clocks: &CoreClocks,
    ) -> Sai<Self, I2S>
    where
        PINS: I2SPinsChA<Self>,
        T: Into<Hertz>,
    {
        Sai::i2s_sai1_ch_a(
            self,
            _pins,
            audio_freq.into(),
            bit_rate,
            prec,
            clocks,
        )
    }
    fn i2s_ch_b<PINS, T>(
        self,
        _pins: PINS,
        audio_freq: T,
        bit_rate: I2SBitRate,
        prec: rec::Sai1,
        clocks: &CoreClocks,
    ) -> Sai<Self, I2S>
    where
        PINS: I2SPinsChB<Self>,
        T: Into<Hertz>,
    {
        Sai::i2s_sai1_ch_b(
            self,
            _pins,
            audio_freq.into(),
            bit_rate,
            prec,
            clocks,
        )
    }
}

impl Sai<SAI1, I2S> {
    pub fn i2s_sai1_ch_a<PINS>(
        sai: SAI1,
        _pins: PINS,
        audio_freq: Hertz,
        bit_rate: I2SBitRate,
        prec: rec::Sai1,
        clocks: &CoreClocks,
    ) -> Self
    where
        PINS: I2SPinsChA<SAI1>,
    {
        // Clock config
        let nbslot: u8 = 2;
        assert!(nbslot <= NUM_SLOTS);
        let (frame_length, slot_size) = match bit_rate {
            I2SBitRate::BITS_8 => (16 * (nbslot / 2), I2SSlotSize::BITS_16),
            I2SBitRate::BITS_10 => (32 * (nbslot / 2), I2SSlotSize::BITS_16),
            I2SBitRate::BITS_16 => (32 * (nbslot / 2), I2SSlotSize::BITS_16),
            I2SBitRate::BITS_20 => (64 * (nbslot / 2), I2SSlotSize::BITS_32),
            I2SBitRate::BITS_24 => (64 * (nbslot / 2), I2SSlotSize::BITS_32),
            I2SBitRate::BITS_32 => (64 * (nbslot / 2), I2SSlotSize::BITS_32),
        };

        /* Configure Master Clock Divider using the following formula :
        - If NODIV = 1 :
            MCKDIV[5:0] = SAI_CK_x / (audio_frequncy * (frame_length + 1))
        - If NODIV = 0 :
            MCKDIV[5:0] = SAI_CK_x / (audio_frequncy * (oversampling_rate + 1) * 256) */

        let ker_ck_a = SAI1::sai_a_ker_ck(&prec, clocks)
            .expect("SAI kernel clock must run!");
        // Divider enabled
        let mclk_div = (ker_ck_a.0)
            / (audio_freq.0 * (I2SOverSampling::Disabled as u32) * 256);
        let mclk_div: u8 = mclk_div
            .try_into()
            .expect("SAI kernel clock is out of range for required MCLK");

        // Configure SAI peripeheral
        let mut per_sai = Sai {
            rb: sai,
            master_channel: SaiChannel::ChannelA,
            slave_channel: Some(SaiChannel::ChannelB),
            interface: I2S { using_dma: false },
        };

        // RCC enable, reset
        per_sai.sai_rcc_init(prec);

        // 16 bits in register correspond to 1 slot each max 16 slots
        let slot_en_bits: u16 = (2_u32.pow(nbslot.into()) - 1) as u16;

        //     if (protocol == SAI_I2S_LSBJUSTIFIED)
        //   {
        //     if (datasize == SAI_PROTOCOL_DATASIZE_16BITEXTENDED)
        //     {
        //       hsai->SlotInit.FirstBitOffset = 16;
        //     }
        //     if (datasize == SAI_PROTOCOL_DATASIZE_24BIT)
        //     {
        //       hsai->SlotInit.FirstBitOffset = 8;
        //     }
        //   }
        let first_bit_offset = 0;

        let master_ch = match per_sai.master_channel {
            SaiChannel::ChannelA => &per_sai.rb.cha,
            SaiChannel::ChannelB => &per_sai.rb.chb,
        };

        /*  Follow the sequence below to configure the SAI interface in DMA mode:
            1. Configure SAI and FIFO threshold levels to specify when the DMA request will be
            launched.
            2. Configure SAI DMA channel.
            3. Enable the DMA.
            4. Enable the SAI interface.
            Note: Before configuring the SAI block, the SAI DMA channel must be disabled.
        */

        i2s_config_channel(
            master_ch,
            I2SMode::MasterTx,
            I2SSynchronization::MASTER,
            mclk_div,
            bit_rate,
            nbslot,
            frame_length,
            first_bit_offset,
            slot_size,
            slot_en_bits,
        );

        if let Some(slave_channel) = &per_sai.slave_channel {
            let slave_ch = match slave_channel {
                SaiChannel::ChannelA => &per_sai.rb.cha,
                SaiChannel::ChannelB => &per_sai.rb.chb,
            };
            i2s_config_channel(
                slave_ch,
                I2SMode::SlaveRx,
                I2SSynchronization::INTERNAL,
                mclk_div,
                bit_rate,
                nbslot,
                frame_length,
                first_bit_offset,
                slot_size,
                slot_en_bits,
            );
        }

        // Configure DMA

        per_sai
    }

    pub fn i2s_sai1_ch_b<PINS>(
        sai: SAI1,
        _pins: PINS,
        _audio_freq: Hertz,
        _bit_rate: I2SBitRate,
        _prec: rec::Sai1,
        _clocks: &CoreClocks,
    ) -> Self
    where
        PINS: I2SPinsChB<SAI1>,
    {
        let per_sai = Sai {
            rb: sai,
            master_channel: SaiChannel::ChannelB,
            slave_channel: None,
            interface: I2S { using_dma: false },
        };
        per_sai
    }


    pub fn config_dma(&mut self, dma: stm32::DMA1, dmamux: stm32::DMAMUX1, ptr1: u32, ptr2: u32) {
        if let Some(slave_channel) = &self.slave_channel {
            match slave_channel {
                SaiChannel::ChannelA => {
                    self.rb.cha.cr1.modify(|_, w| w.dmaen().enabled())
                }
                SaiChannel::ChannelB => {
                    self.rb.chb.cr1.modify(|_, w| w.dmaen().enabled())
                }
            }
        };
        match self.master_channel {
            SaiChannel::ChannelA => {
                self.rb.cha.cr1.modify(|_, w| w.dmaen().enabled())
            }
            SaiChannel::ChannelB => {
                self.rb.chb.cr1.modify(|_, w| w.dmaen().enabled())
            }
        }

        &dma.st[0].cr.modify(|_, w| {
            w.dmeie()
                .enabled()
                // .htie().enabled()
                // .teie().enabled()
                .tcie().enabled()
                // .pfctrl().
                .dir()
                .peripheral_to_memory()
                .circ()
                .enabled() // Circular mode implies DMA flow control
                .minc()
                .incremented()
                .pl()
                .high()
                .psize()
                .bits32()
                .msize()
                .bits32()
        });
        &dma.st[1].cr.modify(|_, w| {
            w.dmeie()
                .enabled()
                // .htie().enabled()
                // .teie().enabled()
                .tcie().enabled()
                // .pfctrl().
                .dir()
                .memory_to_peripheral()
                .circ()
                .enabled() // Circular mode implies DMA flow control
                .minc()
                .incremented()
                .pl()
                .high()
                .psize()
                .bits32()
                .msize()
                .bits32()
        });
        // Direct mode enabled i.e. disable FIFO
        &dma.st[0].fcr.modify(|_, w| w.dmdis().enabled());
        &dma.st[1].fcr.modify(|_, w| w.dmdis().enabled());
        // DMA Memory Address
        unsafe {
            &dma.st[0].m0ar.modify(|_, w| w.bits(ptr1 as u32) );
            &dma.st[1].m0ar.modify(|_, w| w.bits(ptr2 as u32) );
        }

         &dmamux.ccr[0].modify(|_, w| {
            w.dmareq_id().sai1a_dma()
            .soie().enabled()
            .se().enabled()
            .spol().rising_edge()
            .nbreq().bits(0b1)
        });
        &dmamux.rgcr[0].modify(|_, w| {
            w.ge().enabled()
            .gpol().rising_edge()
            .gnbreq().bits(0b1)
        });
        &dmamux.ccr[1].modify(|_, w| {
            w.dmareq_id().sai1b_dma()
            .soie().disabled()
            .se().disabled()
            .spol().rising_edge()
            .nbreq().bits(0b1)
        });

    }

    pub fn enable(&mut self) {
        // Enable slave first "recommended" per ref doc
        if let Some(slave_channel) = &self.slave_channel {
            match slave_channel {
                SaiChannel::ChannelA => {
                    self.rb.cha.cr1.modify(|_, w| w.saien().enabled())
                }
                SaiChannel::ChannelB => {
                    self.rb.chb.cr1.modify(|_, w| w.saien().enabled())
                }
            }
        };
        match self.master_channel {
            SaiChannel::ChannelA => {
                self.rb.cha.cr1.modify(|_, w| w.saien().enabled())
            }
            SaiChannel::ChannelB => {
                self.rb.chb.cr1.modify(|_, w| w.saien().enabled())
            }
        }
    }

    pub fn disable(&mut self) {
        // Master must be disabled first
        match self.master_channel {
            SaiChannel::ChannelA => {
                self.rb.cha.cr1.modify(|_, w| w.saien().disabled())
            }
            SaiChannel::ChannelB => {
                self.rb.chb.cr1.modify(|_, w| w.saien().disabled())
            }
        }
        if let Some(slave_channel) = &self.slave_channel {
            match slave_channel {
                SaiChannel::ChannelA => {
                    self.rb.cha.cr1.modify(|_, w| w.saien().disabled())
                }
                SaiChannel::ChannelB => {
                    self.rb.chb.cr1.modify(|_, w| w.saien().disabled())
                }
            }
        };
    }
}

impl FullDuplex<u32> for I2S {
    type Error = I2SError;

    fn try_read(&mut self) -> nb::Result<(u32, u32), Self::Error> {
        let data: u32 = 0;

        //TODO: Read Data

        Ok((data, data))
    }

    fn try_send(
        &mut self,
        _left_word: u32,
        _right_word: u32,
    ) -> nb::Result<(), Self::Error> {
        //TODO: Send Data
        Ok(())
    }
}

fn i2s_config_channel(
    audio_ch: &stm32::sai4::CH,
    mode_bits: I2SMode,
    sync_bits: I2SSynchronization,
    mclk_div: u8,
    bit_rate: I2SBitRate,
    nbslot: u8,
    frame_length: u8,
    first_bit_offset: u8,
    slot_size: I2SSlotSize,
    slot_en_bits: u16,
) {
    /*  Compute ClockStrobing according to mode
        Tx = Falling = 1 = true
        Rx = Rising =  0 = false
    */
    let clock_strobe = match mode_bits {
        I2SMode::SlaveTx => true,
        I2SMode::MasterTx => true,
        I2SMode::MasterRx => false,
        I2SMode::SlaveRx => false,
    };
    unsafe {
        audio_ch.cr1.modify(|_, w| {
            w.mode()
                .bits(mode_bits as u8)
                .prtcfg()
                .free()
                .ds()
                .bits(bit_rate as u8)
                .lsbfirst()
                .msb_first()
                .ckstr()
                .bit(clock_strobe)
                .syncen()
                .bits(sync_bits as u8)
                .mono()
                .stereo()
                //OUT DRIV
                //SAI EN
                .dmaen()
                .enabled()
                .nodiv()
                .master_clock() // No division from MCLK to SCK
                .mckdiv()
                .bits(mclk_div + 1)
            //OSR defaults to 0
        });
        audio_ch.cr2.modify(|_, w| {
            w.fth()
                .empty()
                .fflush()
                .clear_bit()
                .tris()
                .clear_bit()
                .mute()
                .clear_bit()
                .muteval()
                .clear_bit()
                .muteval()
                .clear_bit()
                .cpl()
                .clear_bit()
                // CPL, unused with companding off
                .comp()
                .no_companding()
        });
        audio_ch.frcr.modify(|_, w| {
            w.frl()
                .bits(frame_length - 1)
                .fsall()
                .bits(frame_length / 2)
                .fsdef()
                .set_bit() // left/right channels enabled
                .fspol()
                .set_bit() // FS active high
                .fsoff()
                .clear_bit()
        });
        audio_ch.slotr.modify(|_, w| {
            w.fboff()
                .bits(first_bit_offset)
                .slotsz()
                .bits(slot_size as u8)
                .nbslot()
                .bits(nbslot)
                .sloten()
                .bits(slot_en_bits)
        });
    }
}
