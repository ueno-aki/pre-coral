use std::ops::Deref;

use super::{impl_ops, Uvarlong, VariableInteger};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Varlong(pub i64);

impl VariableInteger for Varlong {
    #[inline]
    fn byte_size(&self) -> usize {
        match self.0 {
            0 => 1,
            n => {
                let b = (n >> 63) ^ (n << 1);
                let bit_len = 64 - b.leading_zeros();
                (bit_len as f32 / 7.0).ceil() as usize
            }
        }
    }

    #[inline]
    fn write<W: byteorder::WriteBytesExt>(&self, w: &mut W) -> anyhow::Result<()> {
        let v = (self.0 >> 63) ^ (self.0 << 1);
        Uvarlong(v as u64).write(w)
    }

    #[inline]
    fn read<R>(r: &mut R) -> anyhow::Result<Self>
    where
        R: byteorder::ReadBytesExt,
        Self: Sized,
    {
        let Uvarlong(v) = Uvarlong::read(r)?;
        Ok(Varlong(((v >> 1) as i64) ^ (-((v & 1) as i64))))
    }
}

impl PartialEq<i64> for Varlong {
    #[inline]
    fn eq(&self, other: &i64) -> bool {
        PartialEq::eq(&self.0, other)
    }
}
impl Deref for Varlong {
    type Target = i64;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<i64> for Varlong {
    #[inline]
    fn from(value: i64) -> Self {
        Varlong(value)
    }
}
impl From<Varlong> for i64 {
    #[inline]
    fn from(value: Varlong) -> Self {
        value.0
    }
}
impl TryFrom<usize> for Varlong {
    type Error = anyhow::Error;
    #[inline]
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into()?))
    }
}
impl TryFrom<Varlong> for usize {
    type Error = anyhow::Error;
    #[inline]
    fn try_from(value: Varlong) -> Result<Self, Self::Error> {
        Ok(value.0.try_into()?)
    }
}
impl std::ops::Neg for Varlong {
    type Output = Self;
    #[inline]
    #[track_caller]
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}
impl_ops!(Varlong);
