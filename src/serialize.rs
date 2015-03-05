//! Load and save projects

use std::default::Default;
use std::error::FromError;
use std::io::{Error,Read,Write};
use std::fs::File;
use std::fs::PathExt;
use std::path::Path;

use rustc_serialize::json::{DecoderError,EncoderError};
use rustc_serialize::json::{decode,encode};

use super::Tg;

#[derive(Debug)]
pub enum SerialiseError {
    Io(Error),
    Decoder(DecoderError),
    Encoder(EncoderError)
}

impl FromError<Error> for SerialiseError {
    fn from_error(err: Error) -> SerialiseError {
        SerialiseError::Io(err)
    }
}

impl FromError<DecoderError> for SerialiseError {
    fn from_error(err: DecoderError) -> SerialiseError {
        SerialiseError::Decoder(err)
    }
}

impl FromError<EncoderError> for SerialiseError {
    fn from_error(err: EncoderError) -> SerialiseError {
        SerialiseError::Encoder(err)
    }
}

pub type SerialiseResult<T> = Result<T, SerialiseError>;

pub fn load(path: &Path) -> SerialiseResult<Tg> {
    if !path.exists() {
        info!("Path {:?} does not exist, creating defaults", path);
        return Ok(Default::default());
    } else {
        info!("Loading projects from {:?}", path);
        let mut file = try!(File::open(path));
        let mut data = String::new();
        try!(file.read_to_string(&mut data));
        let projects = try!(decode(data.as_slice()));
        return Ok(projects);
    }
}

pub fn save(projects: Tg, path: &Path) -> SerialiseResult<()> {
    info!("Saving projects to {:?}", path);
    let data = try!(encode(&projects));
    let mut file = try!(File::create(path));
    try!(write!(&mut file, "{}", data));
    return Ok(());
}
