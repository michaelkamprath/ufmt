// use core::{mem::MaybeUninit, slice, str, num::flt2dec};
use crate::{uDebug, uDisplay, uWrite, Formatter};

#[cfg(feature = "f32")]
impl uDebug for f32 {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), <W as uWrite>::Error>
    where
        W: uWrite + ?Sized,
    {
        #[cfg(not(feature = "std"))]
        use micromath::F32Ext;

        if self.is_nan() {
            return f.write_str("NaN");
        } else if self.is_infinite() {
            if self.is_sign_positive() {
                return f.write_str("Inf");
            } else {
                return f.write_str("-Inf");
            }
        } else if *self == 0.0 && self.is_sign_negative() {
            return f.write_str("-0.0");
        } else if *self == 0.0 && self.is_sign_positive() {
            return f.write_str("0.0");
        }

        let prec = 10000.0;
        let negative_flag = (*self).is_sign_negative();
        let base = (*self).trunc().abs() as i32;
        let decimal = if negative_flag {
            (-(*self).fract() * prec).round() as u32
        } else {
            ((*self).fract() * prec).round() as u32
        };

        f.write_str(if negative_flag { "-" } else { "" })?;
        uDebug::fmt(&base, f)?;
        f.write_char('.')?;
        if decimal != 0 {
            if decimal >= 1000 {
                // do nothing
            } else if decimal >= 100 {
                f.write_char('0')?;
            } else if decimal >= 10 {
                f.write_str("00")?;
            } else {
                f.write_str("000")?;
            }
            uDebug::fmt(&decimal, f)?;
        } else {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl uDisplay for f32 {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <f32 as uDebug>::fmt(self, f)
    }
}