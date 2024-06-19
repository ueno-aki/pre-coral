use protocol_core::BinaryStream;
use protodef::integer::Varint;

#[derive(Debug, BinaryStream)]
pub struct Disconnect {
    pub reason: Varint,
    pub skip_message: bool,
    #[proto(dependency(!skip_message))]
    pub message: Option<String>,
}
