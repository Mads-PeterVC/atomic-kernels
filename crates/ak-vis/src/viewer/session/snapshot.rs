use super::{ImageSelectionFrames, SelectedImageAtom, SelectionFrames};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SupercellSettings {
    pub repeats: [u32; 3],
    pub ghost_repeated_images: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ViewerSnapshot {
    pub current_frame: usize,
    pub selection: SelectionFrames,
    pub image_selection: ImageSelectionFrames,
    pub supercell: SupercellSettings,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DisplayAtom {
    pub identity: SelectedImageAtom,
    pub position: [f64; 3],
    pub is_main_cell: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct ViewerSessionClosed;

impl std::fmt::Display for ViewerSessionClosed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "viewer session is no longer available")
    }
}

impl std::error::Error for ViewerSessionClosed {}

impl Default for ViewerSnapshot {
    fn default() -> Self {
        Self {
            current_frame: 0,
            selection: SelectionFrames {
                frames: Vec::new(),
                ordered: Vec::new(),
            },
            image_selection: ImageSelectionFrames { frames: Vec::new() },
            supercell: SupercellSettings::default(),
        }
    }
}

impl Default for SupercellSettings {
    fn default() -> Self {
        Self {
            repeats: [0, 0, 0],
            ghost_repeated_images: true,
        }
    }
}

pub(super) fn clamp_frame(index: usize, len: usize) -> usize {
    if len == 0 { 0 } else { index.min(len - 1) }
}

fn axis_offsets(repeat_extent: u32) -> Vec<i32> {
    let repeat_extent = repeat_extent as i32;
    (-repeat_extent..=repeat_extent).collect()
}

pub(super) fn supercell_offsets(repeats: [u32; 3]) -> Vec<[i32; 3]> {
    let mut offsets = Vec::new();
    for ia in axis_offsets(repeats[0]) {
        for ib in axis_offsets(repeats[1]) {
            for ic in axis_offsets(repeats[2]) {
                offsets.push([ia, ib, ic]);
            }
        }
    }
    offsets.sort_by_key(|offset| {
        (
            offset.iter().map(|value| value.abs()).sum::<i32>(),
            offset[0].abs(),
            offset[1].abs(),
            offset[2].abs(),
            offset[0],
            offset[1],
            offset[2],
        )
    });
    offsets
}

pub(super) fn scaled_cell_translation(cell: ak_core::geometry::Cell, offset: [i32; 3]) -> [f64; 3] {
    let a = cell.a();
    let b = cell.b();
    let c = cell.c();
    let shift = a * offset[0] as f64 + b * offset[1] as f64 + c * offset[2] as f64;
    [shift[0], shift[1], shift[2]]
}
