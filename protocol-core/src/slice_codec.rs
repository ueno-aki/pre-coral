use anyhow::Result;
use protodef::{integer::Uvarint, ReadBytesExt, WriteBytesExt, LE};

use crate::{Decoder, Encoder};

#[inline]
fn write_slice<W: WriteBytesExt, E: Encoder>(slice: Vec<E>, w: &mut W) -> Result<()> {
    for v in slice {
        v.encode(w)?;
    }
    Ok(())
}

#[inline]
pub fn encode_u16slice<W: WriteBytesExt, E: Encoder>(slice: Vec<E>, w: &mut W) -> Result<()> {
    w.write_u16::<LE>(slice.len().try_into()?)?;
    write_slice(slice, w)?;
    Ok(())
}
#[inline]
pub fn encode_u32slice<W: WriteBytesExt, E: Encoder>(slice: Vec<E>, w: &mut W) -> Result<()> {
    w.write_u32::<LE>(slice.len().try_into()?)?;
    write_slice(slice, w)?;
    Ok(())
}

#[inline]
pub fn encode_i16slice<W: WriteBytesExt, E: Encoder>(slice: Vec<E>, w: &mut W) -> Result<()> {
    w.write_i16::<LE>(slice.len().try_into()?)?;
    write_slice(slice, w)?;
    Ok(())
}
#[inline]
pub fn encode_i32slice<W: WriteBytesExt, E: Encoder>(slice: Vec<E>, w: &mut W) -> Result<()> {
    w.write_i32::<LE>(slice.len().try_into()?)?;
    write_slice(slice, w)?;
    Ok(())
}

#[inline]
pub fn encode_uvarint_slice<W: WriteBytesExt, E: Encoder>(slice: Vec<E>, w: &mut W) -> Result<()> {
    Uvarint::encode(slice.len().try_into()?, w)?;
    write_slice(slice, w)?;
    Ok(())
}

#[inline]
fn read_slice<R: ReadBytesExt, D: Decoder>(len: usize, r: &mut R) -> Result<Vec<D>> {
    let mut slice = Vec::new();
    for _ in 0..len {
        slice.push(D::decode(r)?)
    }
    Ok(slice)
}

#[inline]
pub fn decode_u16slice<R: ReadBytesExt, D: Decoder>(r: &mut R) -> Result<Vec<D>> {
    let len = r.read_u16::<LE>()? as usize;
    read_slice(len, r)
}
#[inline]
pub fn decode_u32slice<R: ReadBytesExt, D: Decoder>(r: &mut R) -> Result<Vec<D>> {
    let len = r.read_u32::<LE>()? as usize;
    read_slice(len, r)
}

#[inline]
pub fn decode_i16slice<R: ReadBytesExt, D: Decoder>(r: &mut R) -> Result<Vec<D>> {
    let len = r.read_i16::<LE>()? as usize;
    read_slice(len, r)
}
#[inline]
pub fn decode_i32slice<R: ReadBytesExt, D: Decoder>(r: &mut R) -> Result<Vec<D>> {
    let len = r.read_i32::<LE>()? as usize;
    read_slice(len, r)
}

#[inline]
pub fn decode_uvarint_slice<R: ReadBytesExt, D: Decoder>(r: &mut R) -> Result<Vec<D>> {
    let len = Uvarint::decode(r)?.into();
    read_slice(len, r)
}
