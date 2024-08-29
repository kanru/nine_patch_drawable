//! # Parse and Scale Nine Patch Bitmaps
//!
//! This library provides the core functionality to read nine patch bitmaps
//! defined in [Android NinePatch drawables][1] and algorithms to scale them.
//!
//! [1]: https://developer.android.com/develop/ui/views/graphics/drawables#nine-patch

pub struct Patch {
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
}
