use std::io::Cursor;
use std::io::Write;

use azalea_buf::McBuf;
use azalea_buf::McBufReadable;
use azalea_core::position::BlockPos;
use azalea_core::resource_location::ResourceLocation;
use azalea_protocol_macros::ServerboundGamePacket;

use crate::packets::BufReadError;
use crate::packets::McBufWritable;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundSetJigsawBlockPacket {
    pub pos: BlockPos,
    pub name: ResourceLocation,
    pub target: ResourceLocation,
    pub pool: ResourceLocation,
    pub final_state: String,
    pub joint: String,
    #[var]
    pub selection_priority: i32,
    #[var]
    pub placement_priority: i32,
}

pub enum JointType {
    Rollable,
    Aligned,
}

impl McBufReadable for JointType {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let name = String::read_from(buf)?;
        match name.as_str() {
            "rollable" => Ok(JointType::Rollable),
            "aligned" => Ok(JointType::Aligned),
            _ => Err(BufReadError::UnexpectedStringEnumVariant { id: name }),
        }
    }
}

impl McBufWritable for JointType {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        match self {
            JointType::Rollable => "rollable".to_string().write_into(buf)?,
            JointType::Aligned => "aligned".to_string().write_into(buf)?,
        };
        Ok(())
    }
}
