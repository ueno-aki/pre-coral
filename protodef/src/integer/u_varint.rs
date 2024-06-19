use std::ops::Deref;

use anyhow::bail;

use super::{impl_ops, VariableInteger};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Uvarint(pub u32);

pub const UVARINT_MAX_BYTE_COUNT: u32 = 5;

pub const UVARINT_MAX_LAST_VALUE: u32 = 0b1111;

impl VariableInteger for Uvarint {
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
        let mut value: u32 = 0;
        let mut i: u32 = 0;
        loop {
            let b = r.read_u8()? as u32;
            if i == UVARINT_MAX_BYTE_COUNT - 1 {
                if b > UVARINT_MAX_LAST_VALUE {
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

impl PartialEq<u32> for Uvarint {
    fn eq(&self, other: &u32) -> bool {
        PartialEq::eq(&self.0, other)
    }
}
impl Deref for Uvarint {
    type Target = u32;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<u32> for Uvarint {
    #[inline]
    fn from(value: u32) -> Self {
        Self(value)
    }
}
impl TryFrom<usize> for Uvarint {
    type Error = anyhow::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into()?))
    }
}
impl_ops!(Uvarint);
