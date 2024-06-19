use protocol_core::BinaryStream;

#[derive(Debug, BinaryStream)]
pub struct ServerToClientHandshake {
    pub handshake_webtoken: String,
}
