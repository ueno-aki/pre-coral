use std::ops::Deref;

use anyhow::bail;

use super::{impl_ops, VariableInteger};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Uvarlong(pub u64);

pub const UVARLONG_MAX_BYTE_COUNT: u64 = 10;

pub const UVARLONG_MAX_LAST_VALUE: u64 = 0b1;

impl VariableInteger for Uvarlong {
    #[inline]
    fn byte_size(&self) -> usize {
        match self.0 {
            0 => 1,
            n => {
                let bit_len = n.ilog2() + 1;
                (bit_len as f32 / 7.0).ceil() as usize
            }
        }
    }

    #[inline]
    fn write<W: byteorder::WriteBytesExt>(&self, w: &mut W) -> anyhow::Result<()> {
        let mut v = self.0;
        while (v & !0x7f) != 0 {
            w.write_u8((v & 0x7f | 0x80) as u8)?;
            v >>= 7;
        }
        w.write_u8(v as u8)?;
        Ok(())
    }

    #[inline]
    fn read<R>(r: &mut R) -> anyhow::Result<Self>
    where
        R: byteorder::ReadBytesExt,
        Self: Sized,
    {
        let mut value: u64 = 0;
        let mut i: u64 = 0;
        loop {
            let b = r.read_u8()? as u64;
            if i == UVARLONG_MAX_BYTE_COUNT - 1 {
                if b > UVARLONG_MAX_LAST_VALUE {
                    bail!("varint is too big.");
                }
                value |= (b & 0x7f) << (i * 7);
                return Ok(value.into());
            }
            value |= (b & 0x7f) << (i * 7);
            if b & 0x80 == 0 {
                return Ok(value.into());
            }
            i += 1;
        }
    }
}

impl PartialEq<u64> for Uvarlong {
    fn eq(&self, other: &u64) -> bool {
        PartialEq::eq(&self.0, other)
    }
}
impl Deref for Uvarlong {
    type Target = u64;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<u64> for Uvarlong {
    #[inline]
    fn from(value: u64) -> Self {
        Self(value)
    }
}
impl From<usize> for Uvarlong {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value as u64)
    }
}
impl_ops!(Uvarlong);
