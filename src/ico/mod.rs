#[macro_use]
mod macros;

mod bmpdepth;
mod icondir;
mod image;
mod restype;

pub use self::{
    icondir::{IconDir, IconDirEntry},
    image::IconImage,
    restype::ResourceType,
};

//===========================================================================//
