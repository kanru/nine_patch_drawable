//! # Parse and Scale Nine Patch Bitmaps
//!
//! This library provides the core functionality to read nine patch bitmaps
//! defined in [Android NinePatch drawables][1] and algorithms to scale them.
//!
//! [1]: https://developer.android.com/develop/ui/views/graphics/drawables#nine-patch

use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct RectF {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PatchKind {
    Unknown,
    Fixed,
    Stretching,
    Tiling,
}

#[derive(Debug, PartialEq)]
pub struct Section {
    pub start: f32,
    pub len: f32,
    pub kind: PatchKind,
}

#[derive(Debug, PartialEq)]
pub struct Patch {
    pub source: RectF,
    pub target: RectF,
    pub h_kind: PatchKind,
    pub v_kind: PatchKind,
}

#[derive(Debug)]
pub struct NinePatchDrawable {
    pub width: usize,
    pub height: usize,
    pub h_sections: Vec<Section>,
    pub v_sections: Vec<Section>,
    pub margin_left: f32,
    pub margin_top: f32,
    pub margin_right: f32,
    pub margin_bottom: f32,
}

#[derive(Debug)]
pub enum NinePatchError {
    InvalidBitmap,
    InvalidMargin,
}

impl Display for NinePatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NinePatchError::InvalidBitmap => write!(f, "invalid bitmap"),
            NinePatchError::InvalidMargin => write!(f, "invalid margin"),
        }
    }
}

impl NinePatchDrawable {
    /// Create a new nine patch drawable from a bitmap. The pixel format can be
    /// either RGBA(8) or BGRA(8).
    pub fn new(
        bitmap: &[u8],
        stride: usize,
        width: usize,
        height: usize,
    ) -> Result<NinePatchDrawable, NinePatchError> {
        if bitmap.len() != stride * height || stride < width * 4 || width < 3 || height < 3 {
            return Err(NinePatchError::InvalidBitmap);
        }

        let top_sections = h_sections(bitmap, 0, width);
        let left_sections = v_sections(bitmap, 0, stride, height);
        let right_sections = dbg!(v_sections(bitmap, (width - 1) * 4, stride, height));
        let bottom_sections = dbg!(h_sections(bitmap, (height - 1) * stride, width));

        if right_sections.len() != 0 && right_sections.len() != 3 {
            return Err(NinePatchError::InvalidMargin);
        }
        if bottom_sections.len() != 0 && bottom_sections.len() != 3 {
            return Err(NinePatchError::InvalidMargin);
        }

        Ok(NinePatchDrawable {
            width,
            height,
            h_sections: top_sections,
            v_sections: left_sections,
            margin_left: bottom_sections.first().map_or(0.0, |s| s.len),
            margin_top: right_sections.first().map_or(0.0, |s| s.len),
            margin_right: bottom_sections.last().map_or(0.0, |s| s.len),
            margin_bottom: right_sections.last().map_or(0.0, |s| s.len),
        })
    }

    pub fn scale_to(&self, width: usize, height: usize) -> Vec<Patch> {
        assert!(width >= self.width && height >= self.height);

        let stretching_width: f32 = self
            .h_sections
            .iter()
            .filter(|s| s.kind != PatchKind::Fixed)
            .map(|s| s.len)
            .sum();
        let stretching_height: f32 = self
            .v_sections
            .iter()
            .filter(|s| s.kind != PatchKind::Fixed)
            .map(|s| s.len)
            .sum();
        let fixed_width = self.width as f32 - stretching_width;
        let fixed_height = self.height as f32 - stretching_height;
        let mut left = 1.0;
        let mut top = 1.0;
        let mut prev_bottom = 1.0;
        let mut patches = vec![];
        for v in &self.v_sections {
            for h in &self.h_sections {
                let source = RectF {
                    left: h.start + 1.0,
                    top: v.start + 1.0,
                    right: h.start + 1.0 + h.len,
                    bottom: v.start + 1.0 + v.len,
                };
                let right = match h.kind {
                    PatchKind::Fixed => left + h.len,
                    _ => left + (h.len / stretching_width) * (width as f32 - fixed_width),
                };
                let bottom = match v.kind {
                    PatchKind::Fixed => top + v.len,
                    _ => top + (v.len / stretching_height) * (height as f32 - fixed_height),
                };
                patches.push(Patch {
                    source,
                    target: RectF {
                        left,
                        top,
                        right,
                        bottom,
                    },
                    h_kind: h.kind,
                    v_kind: v.kind,
                });
                left = right;
                prev_bottom = bottom;
            }
            left = 1.0;
            top = prev_bottom;
        }
        patches
    }
}

fn h_sections(bitmap: &[u8], offset: usize, width: usize) -> Vec<Section> {
    let mut start = 0.0;
    let mut len = 0.0;
    let mut kind = PatchKind::Unknown;
    let mut sections = vec![];
    for i in 1..width {
        let o = offset + i * 4;
        let i_kind = match (bitmap[o], bitmap[o + 1], bitmap[o + 2]) {
            (0xFF, 0xFF, 0xFF) => PatchKind::Fixed,
            _ => PatchKind::Stretching,
        };
        if i != (width - 1) && (kind == i_kind || kind == PatchKind::Unknown) {
            len += 1.0;
            kind = i_kind;
        } else {
            sections.push(Section { start, len, kind });
            (start, len, kind) = (start + len, 1.0, i_kind);
        }
    }
    sections
}

fn v_sections(bitmap: &[u8], offset: usize, advance: usize, height: usize) -> Vec<Section> {
    let mut start = 0.0;
    let mut len = 0.0;
    let mut kind = PatchKind::Unknown;
    let mut sections = vec![];
    for i in 1..height {
        let o = offset + i * advance;
        let i_kind = match (bitmap[o], bitmap[o + 1], bitmap[o + 2]) {
            (0xFF, 0xFF, 0xFF) => PatchKind::Fixed,
            _ => PatchKind::Stretching,
        };
        if i != (height - 1) && (kind == i_kind || kind == PatchKind::Unknown) {
            len += 1.0;
            kind = i_kind;
        } else {
            sections.push(Section { start, len, kind });
            (start, len, kind) = (start + len, 1.0, i_kind);
        }
    }
    sections
}

#[cfg(test)]
mod tests;
