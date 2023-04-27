use quinn::{ConnectionError, ReadError, WriteError};
use std::array::TryFromSliceError;
use std::net::AddrParseError;
use std::str::Utf8Error;
use thiserror::Error;
use tokio::io;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error")]
    IoError(#[from] io::Error),
    #[error("Empty response")]
    EmptyResponse,
    #[error("Invalid response: {0}")]
    InvalidResponse(u8),
    #[error("Cannot parse integer")]
    CannotParseSlice(#[from] TryFromSliceError),
    #[error("Cannot parse UTF8")]
    CannotParseUtf8(#[from] Utf8Error),
    #[error("Cannot parse address")]
    CannotParseAddress(#[from] AddrParseError),
    #[error("Write error")]
    WriteError(#[from] WriteError),
    #[error("Read error")]
    ReadError(#[from] ReadError),
    #[error("Connection error")]
    ConnectionError(#[from] ConnectionError),
}