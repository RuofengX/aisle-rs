use std::net::SocketAddr;

use bytes::Bytes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Command {
    Connect,
    Bind,
    UDP,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Request {
    Meta(ReqMeta),
    Data(Bytes),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReqMeta{
    pub cmd: Command,
    pub domain: String,
    pub dst: SocketAddr,
    pub data: Bytes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Response{
    Ok(RespData),
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RespData{
    pub srt: SocketAddr,
    pub data: Bytes,
}