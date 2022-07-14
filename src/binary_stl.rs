use super::*;

pub struct BinaryStl;

impl Exporter for BinaryStl
{
    fn serialize(stream: &mut impl Write, data: &Content) -> Result<(), String> {
        Ok(())
    }

    fn capabilities() -> Capabilities {
        Capabilities::MESHES | Capabilities::TRIANGLE_COLORS | Capabilities::UNITS
    }
}
