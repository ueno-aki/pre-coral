use protocol_core::{
    slice_codec::{decode_u16slice, encode_u16slice},
    BinaryStream,
};

use crate::types::resource_pack_client_responce_status::ResponseStatus;

#[derive(Debug, BinaryStream)]
pub struct ResourcePackClientResponse {
    pub response: ResponseStatus,
    #[proto(encode_with(encode_u16slice), decode_with(decode_u16slice))]
    pub downloading_packs: Vec<String>,
}
