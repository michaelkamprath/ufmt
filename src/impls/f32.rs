// use core::{mem::MaybeUninit, slice, str, num::flt2dec};
use crate::{uDebug, uDisplay, uWrite, Formatter};

impl uDebug for f32 {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), <W as uWrite>::Error>
    where
        W: uWrite + ?Sized,
    {
        #[cfg(not(feature = "std"))]
        use micromath::F32Ext;

        let prec = 10000.0;
        let negative_flag = (*self).is_sign_negative();
        let base = (*self).trunc() as i32;
        let decimal = if negative_flag { (-(*self).fract() * prec) as u32 } else { ((*self).fract() * prec) as u32 };

        f.write_str(if negative_flag { "-" } else { "" })?;
        uDebug::fmt(&base, f)?;
        f.write_char('.')?;
        if decimal != 0 {
            if decimal >= 1000 {
                uDebug::fmt(&decimal, f)?;
            } else if decimal >= 100 {
                f.write_char('0')?;
                uDebug::fmt(&decimal, f)?;
            } else if decimal >= 10 {
                f.write_str("00")?;
                uDebug::fmt(&decimal, f)?;
            } else {
                f.write_str("000")?;
                uDebug::fmt(&decimal, f)?;
            }
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