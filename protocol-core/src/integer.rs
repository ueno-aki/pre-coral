use protodef::integer::*;

use crate::{Decoder, Encoder};

macro_rules! impl_codec_for_variableint {
    ($($int:ty),*) => {
        $(
            impl Encoder for $int {
                #[inline]
                fn encode<W: protodef::WriteBytesExt>(self, w: &mut W) -> anyhow::Result<()> {
                    <$int as ::protodef::integer::VariableInteger>::write(&self, w)
                }
            }
            impl Decoder for $int {
                #[inline]
                fn decode<R>(r: &mut R) -> anyhow::Result<Self>
                where
                    R: protodef::ReadBytesExt,
                    Self: Sized,
                {
                    <$int as ::protodef::integer::VariableInteger>::read(r)
                }
            }
        )*
    };
}

impl_codec_for_variableint! {
    Uvarint,Varint,Uvarlong,Varlong
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use anyhow::Result;
    use protodef::integer::*;

    use crate::{Decoder, Encoder};

    #[test]
    fn read_unsigned_variables() -> Result<()> {
        let buf = [
            0xaa, 0x55, 0xff, 0xff, 0xff, 0xff, 0x0f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0x01, 0xaa, 0x55,
        ];
        let mut cursor = Cursor::new(&buf);
        assert_eq!(Uvarint(0x2aaa), Uvarint::decode(&mut cursor)?);
        assert_eq!(Uvarint(u32::MAX), Uvarint::decode(&mut cursor)?);
        assert_eq!(Uvarlong(u64::MAX), Uvarlong::decode(&mut cursor)?);
        assert_eq!(Uvarlong(0x2aaa), Uvarlong::decode(&mut cursor)?);
        Ok(())
    }

    #[test]
    fn read_signed_variables() -> Result<()> {
        let buf = [
            0xd4, 0xaa, 0x01, 0xff, 0xff, 0xff, 0xff, 0x0f, 0xfe, 0xff, 0xff, 0xff, 0x0f,
        ];
        let mut cursor = Cursor::new(&buf);
        assert_eq!(Varint(0x2aaa), Varint::decode(&mut cursor)?);
        assert_eq!(Varint(i32::MIN), Varint::decode(&mut cursor)?);
        assert_eq!(Varint(i32::MAX), Varint::decode(&mut cursor)?);

        let buf = [
            0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x1, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0x01, 0xd4, 0xaa, 0x01,
        ];
        let mut cursor = Cursor::new(&buf);
        assert_eq!(Varlong(i64::MAX), Varlong::decode(&mut cursor)?);
        assert_eq!(Varlong(i64::MIN), Varlong::decode(&mut cursor)?);
        assert_eq!(Varlong(0x2aaa), Varlong::decode(&mut cursor)?);
        Ok(())
    }

    #[test]
    fn write_unsigned_variables() -> Result<()> {
        let mut buf = Vec::<u8>::new();
        Uvarint(u32::MAX).encode(&mut buf)?;
        Uvarint(0b10101010101010).encode(&mut buf)?;
        assert_eq!(vec![0xff, 0xff, 0xff, 0xff, 0x0f, 0xaa, 0x55], buf);
        buf.clear();

        Uvarlong(u64::MAX).encode(&mut buf)?;
        Uvarint(0b10101010101010).encode(&mut buf)?;
        assert_eq!(
            vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01, 0xaa, 0x55],
            buf
        );
        buf.clear();
        Ok(())
    }

    #[test]
    fn write_signed_variables() -> Result<()> {
        let mut buf = Vec::<u8>::new();
        Varint(i32::MAX).encode(&mut buf)?;
        Varint(i32::MIN).encode(&mut buf)?;
        Varint(0b10101010101010).encode(&mut buf)?;
        assert_eq!(
            vec![0xfe, 0xff, 0xff, 0xff, 0x0f, 0xff, 0xff, 0xff, 0xff, 0x0f, 0xd4, 0xaa, 0x01],
            buf
        );
        buf.clear();

        Varlong(i64::MAX).encode(&mut buf)?;
        Varlong(i64::MIN).encode(&mut buf)?;
        Varlong(0b10101010101010).encode(&mut buf)?;
        assert_eq!(
            vec![
                0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x1, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0x1, 0xd4, 0xaa, 0x01
            ],
            buf
        );
        buf.clear();
        Ok(())
    }
}
