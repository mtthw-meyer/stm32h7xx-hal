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
use crate::stm32::{SAI1, SAI4};
use crate::time::Hertz;

use crate::Never;

// use crate::gpio::gpiob::PB2;
// use crate::gpio::gpioc::PC1;
// use crate::gpio::gpiod::PD6;
use crate::gpio::gpioe::{PE2, PE3, PE4, PE5, PE6};
// use crate::gpio::gpioe::{PF6, PF7, PF8, PF9};
// use crate::gpio::gpiof::PG7;
use crate::gpio::{Alternate, AF6};

use embedded_hal::i2s::FullDuplex;

pub enum I2SDataSize {
    BIT_8,
    BIT_10,
    BIT_16,
    BIT_20,
    BIT_24,
    BIT_32,
}

pub enum I2SProtocol {
    LSB,
    MSB,
}

pub enum I2sError {
    FUBAR,
}

pub trait I2SPins<SAI> {

}


pub trait I2SPinMclkA<SAI> {}
pub trait I2SPinSdA<SAI> {}
pub trait I2SPinSckA<SAI> {}
pub trait I2SPinFsA<SAI> {}
pub trait I2SPinMclkB<SAI> {}
pub trait I2SPinSdB<SAI> {}
pub trait I2SPinSckB<SAI> {}
pub trait I2SPinFsB<SAI> {}



impl<SAI, MCLKA, SCKA, FSA, SDA, SDB> I2SPins<SAI> for (MCLKA, SCKA, FSA, SDA, SDB)
where
    MCLKA: I2SPinMclkA<SAI>,
    SCKA: I2SPinSdA<SAI>,
    FSA: I2SPinSckA<SAI>,
    SDA: I2SPinFsA<SAI>,
    SDB: I2SPinSdB<SAI>,
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
impl I2SPinSdA  <SAI1> for PE6<Alternate<AF6>> {}
impl I2SPinSckA <SAI1> for PE5<Alternate<AF6>> {}
impl I2SPinFsA  <SAI1> for PE4<Alternate<AF6>> {}
impl I2SPinSdB  <SAI1> for PE3<Alternate<AF6>> {}

/// I2S Interface
pub struct I2S {
    
}
impl INTERFACE for I2S {}

/// Trait to extend SAI periperhals
pub trait SaiI2sExt<SAI>: Sized {
    type Rec: ResetEnable;
    fn i2s<PINS, T> (
        self,
        _pins: PINS,
        clock: T,
        prec: Self::Rec,
        clocks: &CoreClocks,
    ) -> Sai<SAI, I2S>
    where
        PINS: I2SPins<Self>,
        T: Into<Hertz>;
}

impl SaiI2sExt<SAI1> for SAI1 {
    type Rec = rec::Sai1;
    fn i2s<PINS, T> (
        self,
        _pins: PINS,
        clock: T,
        prec: rec::Sai1,
        clocks: &CoreClocks,
    ) -> Sai<Self, I2S>
    where
        PINS: I2SPins<Self>,
        T: Into<Hertz>,
    {
        Sai::i2s_sai1(self, _pins, clock.into(), prec, clocks)
    }
}

impl Sai<SAI1, I2S> {
    pub fn i2s_sai1<PINS>(
        sai: SAI1,
        _pins: PINS,
        clock: Hertz,
        prec: rec::Sai1,
        clocks: &CoreClocks,
    ) -> Self
    where
        PINS: I2SPins<SAI1>,
    {
        //TODO: Setup
        let mut sai_setup = Sai {
            rb: sai,
            master_channel: SaiChannel::ChannelA,
            slave_channel: Some(SaiChannel::ChannelB),
            interface: I2S {
                
            },
        };
        sai_setup
    }
}

  
impl FullDuplex<u32> for I2S {
    type Error = I2sError;

    fn try_read(&mut self) -> nb::Result<(u32, u32), Self::Error> {
        let data: u32 = 0;

        //TODO: Read Data

        Ok((data,data))
    }

    fn try_send(&mut self, left_word: u32, right_word: u32) -> nb::Result<(), Self::Error> {
        //TODO: Send Data
        Ok(())
    }
}
