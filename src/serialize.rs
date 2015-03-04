//! Load and save projects

use std::collections::HashMap;
use std::error::FromError;
use std::io::{Error,Read,Write};
use std::fs::File;
use std::fs::PathExt;
use std::path::Path;

use rustc_serialize::json::{DecoderError,EncoderError};
use rustc_serialize::json::{decode,encode};

use super::Project;

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

pub type Projects = HashMap<String, Project>;

pub fn load(path: &Path) -> SerialiseResult<Projects> {
    if !path.exists() {
        return Ok(HashMap::new());
    } else {
        let mut file = try!(File::open(path));
        let mut data = String::new();
        try!(file.read_to_string(&mut data));
        let projects = try!(decode(data.as_slice()));
        return Ok(projects);
    }
 }

pub fn save(projects: Projects, path: &Path) -> SerialiseResult<()> {
    let data = try!(encode(&projects));
    let mut file = try!(File::create(path));
    try!(write!(&mut file, "{}", data));
    return Ok(());
}
