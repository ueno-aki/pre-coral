use protocol_core::BinaryStream;

use crate::types::login_request_tokens::LoginRequestTokens;

#[derive(Debug, BinaryStream)]
pub struct Login {
    #[proto(order(BE))]
    pub protocol_version: i32,
    pub tokens: LoginRequestTokens,
}
