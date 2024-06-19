use protocol_core::{Decoder, Encoder};
use protodef::{BE, LE};

#[derive(Debug)]
pub struct Experiments {
    pub list: Vec<ExperimentData>,
    pub experiments_previously_toggled: bool,
}
impl Encoder for Experiments {
    fn encode<W: protodef::WriteBytesExt>(self, w: &mut W) -> anyhow::Result<()> {
        w.write_u32::<LE>(self.list.len().try_into()?)?;
        for ExperimentData {
            toggle_name,
            enabled,
        } in self.list
        {
            toggle_name.encode(w)?;
            enabled.encode(w)?;
        }
        self.experiments_previously_toggled.encode(w)?;
        Ok(())
    }
}
impl Decoder for Experiments {
    fn decode<R>(r: &mut R) -> anyhow::Result<Self>
    where
        R: protodef::ReadBytesExt,
        Self: Sized,
    {
        let mut list = Vec::new();
        for _ in 0..r.read_u32::<BE>()? {
            list.push(ExperimentData {
                toggle_name: String::decode(r)?,
                enabled: bool::decode(r)?,
            });
        }
        let experiments_previously_toggled = bool::decode(r)?;
        Ok(Self {
            list,
            experiments_previously_toggled,
        })
    }
}
#[derive(Debug)]
pub struct ExperimentData {
    pub toggle_name: String,
    pub enabled: bool,
}
