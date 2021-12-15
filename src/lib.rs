pub use camera::*;

pub mod camera;

#[cfg(test)]
mod tests {
    use xiapi_sys::XI_RETURN;
    use serial_test::serial;
    use approx::assert_abs_diff_eq;

    use crate::open_device;

    #[test]
    #[serial]
    fn start_stop_acquisition() -> Result<(), XI_RETURN> {
        let cam = open_device(None)?;
        let acq = cam.start_acquisition()?;
        acq.stop_acquisition()?;
        Ok(())
    }

    #[test]
    #[serial]
    fn set_get_exposure() -> Result<(), XI_RETURN> {
        let mut cam = open_device(None)?;
        cam.set_exposure(12_345.0)?;
        let exp = cam.exposure()?;
        assert_abs_diff_eq!(exp, 12_345.0, epsilon = 10.0);
        Ok(())
    }
}