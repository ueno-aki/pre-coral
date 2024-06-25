use protocol_core::BinaryStream;
use protodef::integer::Varint;

#[derive(Debug, BinaryStream)]
pub struct SetTime {
    pub time: Varint,
}
