use config::Register;
use core::cell::RefCell;
use core::convert::TryInto;
use embedded_hal::blocking::spi::Transfer;
use embedded_hal::digital::v2::OutputPin;
use firmware::update::Update;

use config::ctrl1xl::*;
use config::ctrl4c::*;

mod config;

type SixOf<T> = (T, T, T, T, T, T);

pub enum ImuError<S: Transfer<u8>> {
    WrongIdentity(u8),
    ChipSelectFailed,
    TransferFailed(S::Error),
}

pub struct Lsm6ds33<S, C> {
    spi: RefCell<S>,
    cs: RefCell<C>,

    x: RefCell<f32>,
    y: RefCell<f32>,
    z: RefCell<f32>,

    pitch: RefCell<f32>,
    roll: RefCell<f32>,
    yaw: RefCell<f32>,
}

impl Lsm6ds33<!, !> {
    pub fn try_new<S: Transfer<u8>, C: OutputPin>(
        spi: S,
        cs: C,
    ) -> Result<Lsm6ds33<S, C>, ImuError<S>> {
        let mut imu = Lsm6ds33 {
            spi: RefCell::new(spi),
            cs: RefCell::new(cs),

            x: Default::default(),
            y: Default::default(),
            z: Default::default(),

            pitch: Default::default(),
            roll: Default::default(),
            yaw: Default::default(),
        };

        let mut buf = [0x00; 1];
        imu.read_bytes(0x0f, &mut buf)?;

        // Initialize configuration registers
        imu.configure(Ctrl1Xl(OdrXl::DataRate12_5Hz, FsXl::TwoG, BwXl::Bw50Hz))?;
        imu.configure(Ctrl4C(
            XlBwScalOdr::ByBwXl,
            SleepG::GyroWake,
            Int2OnInt1::Int1AndInt2,
            FifoTempEn::TempDataDisable,
            DrdyMask::DrdyMaskDisable,
            I2cDisable::I2cDisable,
            StopOnFth::FifoDepthUnlimited,
        ))?;

        match buf {
            [0x69] => Ok(imu),
            _ => Err(ImuError::WrongIdentity(buf[0])),
        }
    }
}

impl<S: Transfer<u8>, C: OutputPin> Lsm6ds33<S, C> {
    fn transfer<'a>(&self, buf: &'a mut [u8]) -> Result<&'a [u8], ImuError<S>> {
        self.cs
            .borrow_mut()
            .set_low()
            .map_err(|_| ImuError::<S>::ChipSelectFailed)?;
        let result = self
            .spi
            .borrow_mut()
            .transfer(buf)
            .map_err(ImuError::TransferFailed);
        self.cs
            .borrow_mut()
            .set_high()
            .map_err(|_| ImuError::<S>::ChipSelectFailed)?;
        result
    }

    fn read_bytes<'a, const LEN: usize>(
        &self,
        addr: u8,
        output: &'a mut [u8; LEN],
    ) -> Result<&'a [u8; LEN], ImuError<S>> {
        let buf = &mut [0u8; 16][..LEN + 1];
        buf[0] = addr | 0b_1000_0000;
        self.transfer(buf)?;
        output.copy_from_slice(&buf[1..]);

        Ok(output)
    }

    fn write_bytes<const LEN: usize>(
        &mut self,
        addr: u8,
        input: &[u8; LEN],
    ) -> Result<(), ImuError<S>> {
        // TODO When full const generics are stabilized, convert this to &mut [0u8; LEN + 1]
        let buf = &mut [0u8; 16][..LEN + 1];
        buf[0] = addr & 0b_0111_1111;
        buf[1..].copy_from_slice(input);
        self.transfer(buf)?;
        Ok(())
    }

    pub fn configure<R: Register>(&mut self, register: R) -> Result<(), ImuError<S>> {
        self.write_bytes(register.address(), &[register.value()])?;
        Ok(())
    }
}

impl<S, C> Lsm6ds33<S, C> {
    pub fn primitives<'a>(&'a self) -> SixOf<ImuIter<'a>> {
        (
            ImuIter(&self.x),
            ImuIter(&self.y),
            ImuIter(&self.z),
            ImuIter(&self.pitch),
            ImuIter(&self.roll),
            ImuIter(&self.yaw),
        )
    }
}

pub struct ImuIter<'a>(&'a RefCell<f32>);
impl<'a> Iterator for ImuIter<'a> {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.0.borrow().clone())
    }
}

impl<S: Transfer<u8>, C: OutputPin> Update for Lsm6ds33<S, C> {
    fn update(&self) {
        let mut buf = [0u8; 12];
        let buf = self.read_bytes(0x22, &mut buf).unwrap_or_else(|_| panic!());

        self.pitch
            .replace(i16::from_le_bytes(buf[0..2].try_into().unwrap()).into());
        self.roll
            .replace(i16::from_le_bytes(buf[2..4].try_into().unwrap()).into());
        self.yaw
            .replace(i16::from_le_bytes(buf[4..6].try_into().unwrap()).into());
        self.x
            .replace(i16::from_le_bytes(buf[6..8].try_into().unwrap()).into());
        self.y
            .replace(i16::from_le_bytes(buf[8..10].try_into().unwrap()).into());
        self.z
            .replace(i16::from_le_bytes(buf[10..12].try_into().unwrap()).into());
    }
}
