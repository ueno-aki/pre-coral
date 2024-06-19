use protocol_core::BinaryStream;

#[derive(Debug, BinaryStream)]
pub struct ResourcePackInfo {
    pub id: String,
    pub version: String,
    #[proto(order(LE))]
    pub size: u64,
    pub content_key: String,
    pub sub_pack_name: String,
    pub content_identity: String,
    pub has_scripts: bool,
    pub is_ray_tracing_capable: bool,
}
