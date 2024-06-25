use from_num::from_num;
use protocol_core::{Decoder, Encoder};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[from_num(u8)]
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
        MessageType::from_u8(r.read_u8()?)
    }
}
