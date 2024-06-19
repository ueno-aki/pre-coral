use anyhow::Result;
use byteorder::{ReadBytesExt, WriteBytesExt, LE};

pub fn decode_i32slice<R: ReadBytesExt>(r: &mut R) -> Result<Vec<u8>> {
    let len = r.read_i32::<LE>()?.try_into()?;
    let mut buf = vec![0_u8; len];
    r.read_exact(&mut buf)?;
    Ok(buf)
}
pub fn decode_i64slice<R: ReadBytesExt>(r: &mut R) -> Result<Vec<u8>> {
    let len = r.read_i64::<LE>()?.try_into()?;
    let mut buf = vec![0_u8; len];
    r.read_exact(&mut buf)?;
    Ok(buf)
}

pub fn encode_i32slice<W: WriteBytesExt>(slice: impl AsRef<[u8]>, w: &mut W) -> Result<()> {
    let buf = slice.as_ref();
    w.write_i32::<LE>(buf.len().try_into()?)?;
    w.write_all(buf)?;
    Ok(())
}
pub fn encode_i64slice<W: WriteBytesExt>(slice: impl AsRef<[u8]>, w: &mut W) -> Result<()> {
    let buf = slice.as_ref();
    w.write_i64::<LE>(buf.len().try_into()?)?;
    w.write_all(buf)?;
    Ok(())
}
