use protocol_core::{Decoder, Encoder};
use protodef::integer::Uvarint;

use crate::types::{experiments::Experiments, stack_addon_pack::StackAddonPack};

pub struct ResourcePackStack {
    pub texture_pack_required: bool,
    pub behavior_packs: Vec<StackAddonPack>,
    pub texture_packs: Vec<StackAddonPack>,
    pub base_game_version: String,
    pub experiments: Experiments,
    pub include_editor_packs: bool,
}

impl Encoder for ResourcePackStack {
    fn encode<W: protodef::WriteBytesExt>(self, w: &mut W) -> anyhow::Result<()> {
        let Self {
            texture_pack_required,
            behavior_packs,
            texture_packs,
            base_game_version,
            experiments,
            include_editor_packs,
        } = self;
        texture_pack_required.encode(w)?;
        Uvarint::try_from(behavior_packs.len())?.encode(w)?;
        for bp in behavior_packs {
            bp.encode(w)?;
        }
        Uvarint::try_from(texture_packs.len())?.encode(w)?;
        for rp in texture_packs {
            rp.encode(w)?;
        }
        base_game_version.encode(w)?;
        experiments.encode(w)?;
        include_editor_packs.encode(w)?;
        Ok(())
    }
}

impl Decoder for ResourcePackStack {
    fn decode<R>(r: &mut R) -> anyhow::Result<Self>
    where
        R: protodef::ReadBytesExt,
        Self: Sized,
    {
        let texture_pack_required = bool::decode(r)?;
        let mut behavior_packs = Vec::new();
        for _ in 0..Uvarint::decode(r)?.0 {
            behavior_packs.push(StackAddonPack::decode(r)?);
        }
        let mut texture_packs = Vec::new();
        for _ in 0..Uvarint::decode(r)?.0 {
            texture_packs.push(StackAddonPack::decode(r)?);
        }
        let base_game_version = String::decode(r)?;
        let experiments = Experiments::decode(r)?;
        let include_editor_packs = bool::decode(r)?;
        Ok(Self {
            texture_pack_required,
            behavior_packs,
            texture_packs,
            base_game_version,
            experiments,
            include_editor_packs,
        })
    }
}
