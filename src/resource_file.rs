use binread::io::{Read, Seek, SeekFrom};
use binread::prelude::*;
use binread::{FilePtr32, ReadOptions};
use log::{debug, info, trace};

#[derive(BinRead, Debug)]
#[br(little)]
struct GenericCharacterObj {
    pose_data: FilePtr32<PoseData>,
    sprite_data: FilePtr32<SpriteData>,
}

pub enum ObjectData {
    Pose(PoseData),
    Sprite(SpriteData),
}

#[derive(Debug)]
pub struct PoseData {
    poses: Vec<Pose>,
}

impl BinRead for PoseData {
    type Args = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        _options: &ReadOptions,
        _args: Self::Args,
    ) -> BinResult<Self> {
        debug!("Reading PoseData section");
        let mut result_vec: Vec<Pose> = Vec::new();

        // Starting offset for all pointers
        let mut ptr_read_options = ReadOptions::default();
        ptr_read_options.offset = reader.seek(SeekFrom::Current(0))?;

        debug!("PoseData pointers relative to `0x{:#X}`", ptr_read_options.offset);

        let mut total_dwords = 0;
        loop {
            let pose_ptr: FilePtr32<Pose> = FilePtr32::read_options(reader, &ptr_read_options, _args)?;
            
            total_dwords += 1;
            if pose_ptr.ptr == 0xFFFFFFFF {
                break
            }

            let pose = pose_ptr.into_inner();
            debug!("Got Pose: {:#?}", pose);

            result_vec.push(pose);
        }

        let ff_padding = (0x10 - (total_dwords % 0x10)) * 4; // Multiply by size of dword (u32) for bytes
        reader.seek(SeekFrom::Current(ff_padding))?;
        
        debug!("Skipped `{}` bytes of padding", ff_padding);

        Ok(
            PoseData {
                poses: result_vec,
            }
        )
    }
}

#[derive(BinRead, Debug)]
pub struct Pose {
    hitbox_count: u32,
    #[br(count = hitbox_count)]
    hitboxes: Vec<HitBox>,
    sprite_x_offset: i16,
    sprite_y_offset: i16,
    unknown: u32,
    sprite_index: u32,
}

#[derive(Debug, BinRead)]
pub struct HitBox {
    x_offset: i16,
    y_offset: i16,
    width: u16,
    height: u16,
    hitbox_type: u32,
}

#[derive(BinRead, Debug)]
pub struct SpriteData {}