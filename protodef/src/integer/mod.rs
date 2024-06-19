mod u_varint;
mod u_varlong;
mod varint;
mod varlong;

use byteorder::{ReadBytesExt, WriteBytesExt};
pub use u_varint::*;
pub use u_varlong::*;
pub use varint::*;
pub use varlong::*;

pub trait VariableInteger {
    fn byte_size(&self) -> usize;
    fn write<W: WriteBytesExt>(&self, w: &mut W) -> anyhow::Result<()>;
    fn read<R>(r: &mut R) -> anyhow::Result<Self>
    where
        R: ReadBytesExt,
        Self: Sized;
}

macro_rules! impl_ops {
    ($struc:ty) => {
        impl std::ops::Add for $struc {
            type Output = Self;
            #[inline]
            #[track_caller]
            fn add(self, rhs: Self) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }
        impl std::ops::AddAssign for $struc {
            #[inline]
            #[track_caller]
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0;
            }
        }
        impl std::ops::Mul for $struc {
            type Output = Self;
            #[inline]
            #[track_caller]
            fn mul(self, rhs: Self) -> Self::Output {
                Self(self.0 * rhs.0)
            }
        }
        impl std::ops::MulAssign for $struc {
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: Self) {
                self.0 *= rhs.0;
            }
        }
        impl std::ops::Rem for $struc {
            type Output = Self;
            #[inline]
            #[track_caller]
            fn rem(self, rhs: Self) -> Self::Output {
                Self(self.0 % rhs.0)
            }
        }
        impl std::ops::RemAssign for $struc {
            #[inline]
            #[track_caller]
            fn rem_assign(&mut self, rhs: Self) {
                self.0 %= rhs.0
            }
        }
        impl std::ops::Sub for $struc {
            type Output = Self;
            #[inline]
            #[track_caller]
            fn sub(self, rhs: Self) -> Self::Output {
                Self(self.0 - rhs.0)
            }
        }
        impl std::ops::SubAssign for $struc {
            #[inline]
            #[track_caller]
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0;
            }
        }
    };
}
pub(crate) use impl_ops;
