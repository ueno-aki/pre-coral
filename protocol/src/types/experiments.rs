use protocol_core::{
    slice_codec::{decode_u32slice, encode_u32slice},
    BinaryStream,
};

#[derive(Debug, BinaryStream)]
pub struct Experiments {
    #[proto(encode_with(encode_u32slice), decode_with(decode_u32slice))]
    pub list: Vec<ExperimentData>,
    pub experiments_previously_toggled: bool,
}

#[derive(Debug, BinaryStream)]
pub struct ExperimentData {
    pub toggle_name: String,
    pub enabled: bool,
}
