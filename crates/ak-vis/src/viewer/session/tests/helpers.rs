use ak_core::Structure;

use crate::viewer::session::SelectedImageAtom;

pub(super) fn test_structure(x: f64) -> Structure {
    Structure::new(
        vec![[x, 0.0, 0.0], [x + 1.0, 0.0, 0.0]],
        vec![1, 1],
        [[10.0, 0.0, 0.0], [0.0, 10.0, 0.0], [0.0, 0.0, 10.0]],
        [false, false, false],
    )
}

pub(super) fn test_structure4() -> Structure {
    Structure::new(
        vec![
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
        ],
        vec![1, 1, 1, 1],
        [[10.0, 0.0, 0.0], [0.0, 10.0, 0.0], [0.0, 0.0, 10.0]],
        [false, false, false],
    )
}

pub(super) fn test_structure_with_numbers(x: f64, numbers: &[i32]) -> Structure {
    let positions = (0..numbers.len())
        .map(|index| [x + index as f64, 0.0, 0.0])
        .collect();
    Structure::new(
        positions,
        numbers.to_vec(),
        [[10.0, 0.0, 0.0], [0.0, 10.0, 0.0], [0.0, 0.0, 10.0]],
        [false, false, false],
    )
}

pub(super) fn main_image(atom_index: usize) -> SelectedImageAtom {
    SelectedImageAtom {
        atom_index,
        image_offset: [0, 0, 0],
    }
}
