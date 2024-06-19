use anyhow::bail;
use protocol_core::{Decoder, Encoder};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MessageType {
    Raw = 0,
    Chat = 1,
    Translate = 2,
    Popup = 3,
    JukeboxPopup = 4,
    Tip = 5,
    SystemMessage = 6,
    Whisper = 7,
    Announcement = 8,
    JsonWhisper = 9,
    Json = 10,
    JsonAnnouncement = 11,
}

impl Encoder for MessageType {
    fn encode<W: protodef::WriteBytesExt>(self, w: &mut W) -> anyhow::Result<()> {
        w.write_u8(self as u8)?;
        Ok(())
    }
}

impl Decoder for MessageType {
    fn decode<R>(r: &mut R) -> anyhow::Result<Self>
    where
        R: protodef::ReadBytesExt,
        Self: Sized,
    {
        use MessageType::*;
        Ok(match r.read_u8()? {
            0 => Raw,
            1 => Chat,
            2 => Translate,
            3 => Popup,
            4 => JukeboxPopup,
            5 => Tip,
            6 => SystemMessage,
            7 => Whisper,
            8 => Announcement,
            9 => JsonWhisper,
            10 => Json,
            11 => JsonAnnouncement,
            n => bail!("Cannot convert {n} into MessageType"),
        })
    }
}
