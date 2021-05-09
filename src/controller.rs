use embedded_hal::blocking::spi::Transfer;
use embedded_hal::digital::v2::OutputPin;

use crate::driver::imu::Lsm6ds33;

pub struct Frame {
    pub left_quad_n: bool,
    pub left_quad_e: bool,
    pub left_quad_s: bool,
    pub left_quad_w: bool,

    pub right_quad_n: bool,
    pub right_quad_e: bool,
    pub right_quad_s: bool,
    pub right_quad_w: bool,

    pub left_pad_x: f32,
    pub left_pad_y: f32,
    pub left_pad_click: bool,

    pub right_pad_x: f32,
    pub right_pad_y: f32,
    pub right_pad_click: bool,

    pub home: bool,

    pub select: bool,
    pub start: bool,

    pub left_bumper: bool,
    pub right_bumper: bool,

    pub left_trigger: f32,
    pub right_trigger: f32,

    pub left_grip: bool,
    pub right_grip: bool,

    pub accel_x: f32,
    pub accel_y: f32,
    pub accel_z: f32,

    pub gyro_x: f32,
    pub gyro_y: f32,
    pub gyro_z: f32,

    pub mag_x: f32,
    pub mag_y: f32,
    pub mag_z: f32,
}

impl core::fmt::Display for Frame {
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_fmt(format_args!(
            "\nQuads: left(N {}, E {}, S {}, W {}) right(N {}, E {}, S {} W {})",
            self.left_quad_n,
            self.left_quad_e,
            self.left_quad_s,
            self.left_quad_w,
            self.right_quad_n,
            self.right_quad_e,
            self.right_quad_s,
            self.right_quad_w,
        ))?;
        formatter.write_fmt(format_args!(
            "\nPads: left(X {:.02}, Y {:.02}, click {}) right(X {:.02}, Y {:.02}, click {})",
            self.left_pad_x,
            self.left_pad_y,
            self.left_pad_click,
            self.right_pad_x,
            self.right_pad_y,
            self.right_pad_click,
        ))?;
        formatter.write_fmt(format_args!(
            "\nselect {}, home {}, start {}",
            self.select, self.home, self.start,
        ))?;
        formatter.write_fmt(format_args!(
            "\nBumpers: left {}, right {}",
            self.left_bumper, self.right_bumper,
        ))?;
        formatter.write_fmt(format_args!(
            "\nTriggers: left {}, right {}",
            self.left_trigger, self.right_trigger,
        ))?;
        formatter.write_fmt(format_args!(
            "\nGrips: left {}, right {}",
            self.left_grip, self.right_grip,
        ))?;
        formatter.write_fmt(format_args!(
            "\nAccel: X {}, Y {}, Z {}",
            self.accel_x, self.accel_y, self.accel_z
        ))?;
        formatter.write_fmt(format_args!(
            "\nGyro: X {}, Y {}, Z {}",
            self.gyro_x, self.gyro_y, self.gyro_z
        ))?;
        formatter.write_fmt(format_args!(
            "\nMag: X {}, Y {}, Z {}",
            self.mag_x, self.mag_y, self.mag_z
        ))?;
        Ok(())
    }
}

pub struct Controller<S, C>(Lsm6ds33<S, C>);

impl<S, C> Controller<S, C> {
    pub fn new(imu: Lsm6ds33<S, C>) -> Self {
        Controller(imu)
    }
}

impl<S: Transfer<u8>, C: OutputPin> Iterator for Controller<S, C> {
    type Item = Frame;

    fn next(&mut self) -> Option<Self::Item> {
        let imu_item = self.0.next();
        let imu_result = imu_item?;
        if let Ok(((accel_x, accel_y, accel_z), (gyro_x, gyro_y, gyro_z))) = imu_result {
            Some(Frame {
                left_quad_n: false,
                left_quad_e: false,
                left_quad_s: false,
                left_quad_w: false,

                right_quad_n: false,
                right_quad_e: false,
                right_quad_s: false,
                right_quad_w: false,

                left_pad_x: 0.0,
                left_pad_y: 0.0,
                left_pad_click: false,

                right_pad_x: 0.0,
                right_pad_y: 0.0,
                right_pad_click: false,

                home: false,

                select: false,
                start: false,

                left_bumper: false,
                right_bumper: false,

                left_trigger: 0.0,
                right_trigger: 0.0,

                left_grip: false,
                right_grip: false,

                accel_x,
                accel_y,
                accel_z,

                gyro_x,
                gyro_y,
                gyro_z,

                mag_x: 0.0,
                mag_y: 0.0,
                mag_z: 0.0,
            })
        } else {
            Default::default()
        }
    }
}
