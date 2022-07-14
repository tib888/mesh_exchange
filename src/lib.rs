pub mod obj;
pub mod binary_stl;
pub mod whereis;
pub mod content;

use bitflags::bitflags;
use std::io::{Read, Write};
use std::fs::File;
use std::io::BufReader;
use crate::obj::Obj;
use crate::binary_stl::BinaryStl;
use crate::content::*;

bitflags! {

    // Results in default value with bits: 0
    #[derive(Default)]
    pub struct Capabilities: u32 {    
        //list the data a mesh format is able to preserve:
        const UNITS = 1 << 0;
        const MESHES = 1 << 1;
        //const PolyLines = 1 << 0;
        //const Points = 1 << 0;

        //mesh vertex related:
        const DOUBLE_PRECISION_VERTICES = 1 << 8;
        const TEXTURE_COORDINATES = 1 << 9;
        const VERTEX_COLORS = 1 << 10;
        
        //mesh triangle related:
        const TRIANGLE_COLORS = 1 << 16;
        const TRIANGLE_NEIGHBORS = 1 << 17;
            
        //...
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Formats {
     Obj,
     BinaryStl,
     //TextualStl,
     //Dxf
}

pub trait Importer
{
    /// required - list the things which worth to import (the rest may be skipped by the importer)    
    fn deserialize(stream: &impl Read, required: Capabilities) -> Result<Content, String>;//TODO improve error type
}

pub trait Exporter
{    
    fn serialize(stream: &mut impl Write, data: &Content) -> Result<(), String>;//TODO improve error type
    
    /// list the things which can be preserved by this file format
    fn capabilities() -> Capabilities;
}

pub fn convert(input: &mut impl Read, input_format: Formats, output: &mut impl Write, output_format: Formats) -> Result<(), String> 
{
    let capabilities = match output_format {
        Formats::BinaryStl => Ok(BinaryStl::capabilities()),
        _ => Err("not supported output format".to_owned()),
    }?;

    let content = match input_format {
        Formats::Obj => Obj::deserialize(&input, capabilities),
        _ => Err("not supported input format".to_owned()),
    }?;

    match output_format {
        Formats::BinaryStl => BinaryStl::serialize(output, &content),
        _ => Err("not supported output format".to_owned()),
    }
}

//TODO async version (if async feature enabled?)
pub fn convert_file(input: &str, input_format: Formats, output: &str, output_format: Formats) -> Result<(), String> {        
    let input = File::open(input).map_err(|err| err.to_string())?;
    let mut buf_reader = BufReader::new(input);
    let mut output = File::create(output).map_err(|err| err.to_string())?;
    convert(&mut buf_reader, input_format, &mut output, output_format)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(convert_file("d:/CADFiles/_OBJ/lamp.obj", Formats::Obj, "d:/TEMP/lamp.stl", Formats::BinaryStl).is_ok());
    }
}