use protocol_core::BinaryStream;

#[derive(Debug, BinaryStream)]
pub struct StackAddonPack {
    pub id: String,
    pub version: String,
    pub sub_pack_name: String,
}
