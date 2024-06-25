use protocol_core::{
    slice_codec::{decode_u16slice, decode_uvarint_slice, encode_u16slice, encode_uvarint_slice},
    BinaryStream,
};

use crate::types::{behavior_pack_info::BehaviorPackInfo, cdn_url::CDNUrl};

#[derive(Debug, BinaryStream)]
pub struct ResourcePacksInfo {
    pub res_pack_required: bool,
    pub has_addon_packs: bool,
    pub has_scripts: bool,
    pub force_server_packs_enabled: bool,
    #[proto(encode_with(encode_u16slice), decode_with(decode_u16slice))]
    pub behavior_packs: Vec<BehaviorPackInfo>,
    #[proto(encode_with(encode_u16slice), decode_with(decode_u16slice))]
    pub resource_packs: Vec<ResourcePacksInfo>,
    #[proto(encode_with(encode_uvarint_slice), decode_with(decode_uvarint_slice))]
    pub cdn_urls: Vec<CDNUrl>,
}
