use anyhow::bail;
use protocol_core::{Decoder, Encoder};
use protodef::BE;

pub enum PlayStatus {
    LoginSuccess = 0,
    LoginFailedClientOld = 1,
    LoginFailedServerOld = 2,
    PlayerSpawn = 3,
    LoginFailedInvalidTenant = 4,
    LoginFailedEditionMismatchEduToVanilla = 5,
    LoginFailedEditionMismatchVanillaToEdu = 6,
    LoginFailedServerFullSubClient = 7,
    LoginFailedEditorMismatchEditorToVanilla = 8,
    LoginFailedEditorMismatchVanillaToEditor = 9,
}
impl Encoder for PlayStatus {
    fn encode<W: protodef::WriteBytesExt>(self, w: &mut W) -> anyhow::Result<()> {
        w.write_i32::<BE>(self as i32)?;
        Ok(())
    }
}
impl Decoder for PlayStatus {
    fn decode<R>(r: &mut R) -> anyhow::Result<Self>
    where
        R: protodef::ReadBytesExt,
        Self: Sized,
    {
        use PlayStatus::*;
        Ok(match r.read_i32::<BE>()? {
            0 => LoginSuccess,
            1 => LoginFailedClientOld,
            2 => LoginFailedServerOld,
            3 => PlayerSpawn,
            4 => LoginFailedInvalidTenant,
            5 => LoginFailedEditionMismatchEduToVanilla,
            6 => LoginFailedEditionMismatchVanillaToEdu,
            7 => LoginFailedServerFullSubClient,
            8 => LoginFailedEditorMismatchEditorToVanilla,
            9 => LoginFailedEditorMismatchVanillaToEditor,
            n => bail!("Connot convert {n} into PlayStatus"),
        })
    }
}
