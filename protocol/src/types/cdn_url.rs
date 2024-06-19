use protocol_core::BinaryStream;

#[derive(Debug, BinaryStream)]
pub struct CDNUrl {
    pub uuid_version: String,
    pub url: String,
}
