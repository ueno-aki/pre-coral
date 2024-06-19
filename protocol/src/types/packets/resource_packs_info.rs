use protocol_core::{Decoder, Encoder};
use protodef::{integer::Uvarint, LE};

use crate::types::{behavior_pack_info::BehaviorPackInfo, cdn_url::CDNUrl};

#[derive(Debug)]
pub struct ResourcePacksInfo {
    pub res_pack_required: bool,
    pub has_addon_packs: bool,
    pub has_scripts: bool,
    pub force_server_packs_enabled: bool,
    pub behavior_packs: Vec<BehaviorPackInfo>,
    pub resource_packs: Vec<ResourcePacksInfo>,
    pub cdn_urls: Vec<CDNUrl>,
}

impl Encoder for ResourcePacksInfo {
    fn encode<W: protodef::WriteBytesExt>(self, w: &mut W) -> anyhow::Result<()> {
        let Self {
            res_pack_required,
            has_addon_packs,
            has_scripts,
            force_server_packs_enabled,
            behavior_packs,
            resource_packs,
            cdn_urls,
        } = self;
        res_pack_required.encode(w)?;
        has_addon_packs.encode(w)?;
        has_scripts.encode(w)?;
        force_server_packs_enabled.encode(w)?;
        w.write_u16::<LE>(behavior_packs.len().try_into()?)?;
        for bp in behavior_packs {
            bp.encode(w)?;
        }
        w.write_u16::<LE>(resource_packs.len().try_into()?)?;
        for rp in resource_packs {
            rp.encode(w)?;
        }
        Uvarint::try_from(cdn_urls.len())?.encode(w)?;
        for cdn in cdn_urls {
            cdn.encode(w)?;
        }
        Ok(())
    }
}

impl Decoder for ResourcePacksInfo {
    fn decode<R>(r: &mut R) -> anyhow::Result<Self>
    where
        R: protodef::ReadBytesExt,
        Self: Sized,
    {
        let res_pack_required = bool::decode(r)?;
        let has_addon_packs = bool::decode(r)?;
        let has_scripts = bool::decode(r)?;
        let force_server_packs_enabled = bool::decode(r)?;
        let mut behavior_packs = Vec::new();
        for _ in 0..r.read_u16::<LE>()? {
            behavior_packs.push(BehaviorPackInfo::decode(r)?);
        }
        let mut resource_packs = Vec::new();
        for _ in 0..r.read_u16::<LE>()? {
            resource_packs.push(ResourcePacksInfo::decode(r)?);
        }
        let mut cdn_urls = Vec::new();
        for _ in 0..Uvarint::decode(r)?.0 {
            cdn_urls.push(CDNUrl::decode(r)?);
        }
        Ok(Self {
            res_pack_required,
            has_addon_packs,
            has_scripts,
            force_server_packs_enabled,
            behavior_packs,
            resource_packs,
            cdn_urls,
        })
    }
}
