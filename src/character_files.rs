use binread::{FilePtr32};
use binread::prelude::*;


#[derive(BinRead, Debug)]
pub struct SolResource {
    object_1: FilePtr32<CharacterObject>,
}

#[derive(BinRead, Debug)]
struct CharacterObject {

}