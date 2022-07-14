use super::*;

pub struct Obj;

impl Importer for Obj
{
    fn deserialize(stream: &impl Read, required: Capabilities) -> Result<Content, String> {
        Err("".to_string())
    }
}
