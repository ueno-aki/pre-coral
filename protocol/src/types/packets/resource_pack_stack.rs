use protocol_core::{
    slice_codec::{decode_uvarint_slice, encode_uvarint_slice},
    BinaryStream,
};

use crate::types::{experiments::Experiments, stack_addon_pack::StackAddonPack};

#[derive(Debug, BinaryStream)]
pub struct ResourcePackStack {
    pub texture_pack_required: bool,
    #[proto(encode_with(encode_uvarint_slice), decode_with(decode_uvarint_slice))]
    pub behavior_packs: Vec<StackAddonPack>,
    #[proto(encode_with(encode_uvarint_slice), decode_with(decode_uvarint_slice))]
    pub texture_packs: Vec<StackAddonPack>,
    pub base_game_version: String,
    pub experiments: Experiments,
    pub include_editor_packs: bool,
}
