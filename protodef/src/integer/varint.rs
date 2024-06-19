use std::ops::Deref;

use super::{impl_ops, Uvarint, VariableInteger};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Varint(pub i32);

impl VariableInteger for Varint {
    #[inline]
    fn byte_size(&self) -> usize {
        match self.0 {
            0 => 1,
            n => {
                let b = (n >> 31) ^ (n << 1);
                let bit_len = 32 - b.leading_zeros();
                (bit_len as f32 / 7.0).ceil() as usize
            }
        }
    }

    #[inline]
    fn write<W: byteorder::WriteBytesExt>(&self, w: &mut W) -> anyhow::Result<()> {
        let v = (self.0 >> 31) ^ (self.0 << 1);
        Uvarint(v as u32).write(w)
    }

    #[inline]
    fn read<R>(r: &mut R) -> anyhow::Result<Self>
    where
        R: byteorder::ReadBytesExt,
        Self: Sized,
    {
        let Uvarint(v) = Uvarint::read(r)?;
        Ok(Varint(((v >> 1) as i32) ^ (-((v & 1) as i32))))
    }
}

impl PartialEq<i32> for Varint {
    fn eq(&self, other: &i32) -> bool {
        PartialEq::eq(&self.0, other)
    }
}
impl Deref for Varint {
    type Target = i32;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<i32> for Varint {
    #[inline]
    fn from(value: i32) -> Self {
        Varint(value)
    }
}
impl TryFrom<usize> for Varint {
    type Error = anyhow::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into()?))
    }
}
impl std::ops::Neg for Varint {
    type Output = i32;
    #[inline]
    #[track_caller]
    fn neg(self) -> Self::Output {
        -self.0
    }
}
impl_ops!(Varint);
