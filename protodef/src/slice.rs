use anyhow::Result;
use byteorder::{ReadBytesExt, WriteBytesExt, LE};

pub fn read_i32buffer<R: ReadBytesExt>(r: &mut R) -> Result<Vec<u8>> {
    let len = r.read_i32::<LE>()?.try_into()?;
    let mut buf = vec![0_u8; len];
    r.read_exact(&mut buf)?;
    Ok(buf)
}
pub fn read_i64buffer<R: ReadBytesExt>(r: &mut R) -> Result<Vec<u8>> {
    let len = r.read_i64::<LE>()?.try_into()?;
    let mut buf = vec![0_u8; len];
    r.read_exact(&mut buf)?;
    Ok(buf)
}

pub fn write_i32buffer<W: WriteBytesExt>(slice: impl AsRef<[u8]>, w: &mut W) -> Result<()> {
    let buf = slice.as_ref();
    w.write_i32::<LE>(buf.len().try_into()?)?;
    w.write_all(buf)?;
    Ok(())
}
pub fn write_i64buffer<W: WriteBytesExt>(slice: impl AsRef<[u8]>, w: &mut W) -> Result<()> {
    let buf = slice.as_ref();
    w.write_i64::<LE>(buf.len().try_into()?)?;
    w.write_all(buf)?;
    Ok(())
}
