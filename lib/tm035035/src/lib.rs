#![no_std]
#![feature(never_type)]

use embedded_hal::blocking::spi::Transfer;
use embedded_hal::digital::v2::InputPin;

pub struct Tm035035<S, DR> {
    spi: S,
    data_ready: DR,
}

pub enum GlidePointError<S, DR>
where
    S: Transfer<u8>,
    DR: InputPin,
{
    TransferError(S::Error),
    DataReadyError(DR::Error),
}

impl Tm035035<!, !> {
    pub fn try_new<S, DR>(spi: S, data_ready: DR) -> Result<Tm035035<S, DR>, GlidePointError<S, DR>>
    where
        S: Transfer<u8>,
        DR: InputPin,
    {
        let mut glide_point = Tm035035 { spi, data_ready };

        glide_point.reset()?;

        glide_point.write_byte(0x03, 0b_0000_0000)?;
        glide_point.write_byte(0x05, 0b_0001_1110)?;
        glide_point.write_byte(0x04, 0b_0000_0011)?;

        Ok(glide_point)
    }
}

impl<S, DR> Tm035035<S, DR>
where
    S: Transfer<u8>,
    DR: InputPin,
{
    const READ_CMD: u8 = 0xa0;
    const WRITE_CMD: u8 = 0x80;

    const MAX_X: f32 = 2047.0;
    const MAX_Y: f32 = 1535.0;

    fn transfer<'a>(&mut self, buf: &'a mut [u8]) -> Result<&'a [u8], GlidePointError<S, DR>> {
        self.spi
            .transfer(buf)
            .map_err(|e| GlidePointError::TransferError(e))
    }

    fn read_bytes<'a, const LEN: usize>(
        &mut self,
        addr: u8,
        output: &'a mut [u8; LEN],
    ) -> Result<&'a [u8; LEN], GlidePointError<S, DR>> {
        // TODO When full const generics are stabilized, convert this to [0u8; LEN + 3]
        let buf = &mut [0u8; 16][..LEN + 3];
        buf[..2].copy_from_slice(&[Self::READ_CMD | addr, 0xfc]);
        buf[2..3 + LEN].fill(0xfc);
        self.transfer(&mut buf[..3 + LEN])?;
        output.copy_from_slice(&buf[3..3 + LEN]);
        Ok(output)
    }

    fn write_byte(&mut self, addr: u8, byte: u8) -> Result<(), GlidePointError<S, DR>> {
        self.transfer(&mut [Self::WRITE_CMD | addr, byte])?;
        Ok(())
    }

    fn clear_flags(&mut self) -> Result<(), GlidePointError<S, DR>> {
        let result = self.write_byte(0x02, 0x00)?;
        Ok(result)
    }

    fn data_ready(&self) -> Result<bool, GlidePointError<S, DR>> {
        self.data_ready
            .is_high()
            .map_err(|e| GlidePointError::DataReadyError(e))
    }

    fn reset(&mut self) -> Result<(), GlidePointError<S, DR>> {
        self.write_byte(0x03, 0b_0000_0001)?;
        while !self.data_ready()? {}
        self.clear_flags()?;
        Ok(())
    }

    fn read_touch(&mut self) -> Result<Option<Touch>, GlidePointError<S, DR>> {
        Ok(if self.data_ready()? {
            let mut buf = [0u8; 4];
            self.read_bytes(0x14, &mut buf)?;
            self.clear_flags()?;

            if buf == [0, 0, 0, 0] {
                // "Z idle" packet
                Some(Touch::NotTouched)
            } else {
                let [x_low, y_low, xy_high, pressure] = buf;
                let x = x_low as u16 | (((xy_high & 0x0f) as u16) << 8);
                let y = y_low as u16 | (((xy_high & 0xf0) as u16) << 4);

                let x = (x as f32 / Self::MAX_X) * 2.0 - 1.0;
                let y = (y as f32 / Self::MAX_Y) * 2.0 - 1.0;
                let z = pressure as f32 / 255.0;

                Some(Touch::Touched(x, y, z))
            }
        } else {
            None
        })
    }
}

pub enum Touch {
    Touched(f32, f32, f32),
    NotTouched,
}

impl<S: Transfer<u8>, DR: InputPin> Iterator for Tm035035<S, DR> {
    type Item = Result<Touch, GlidePointError<S, DR>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.read_touch().transpose()
    }
}
