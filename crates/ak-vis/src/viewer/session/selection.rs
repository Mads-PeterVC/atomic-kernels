use ak_core::Trajectory;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectionFrames {
    pub(super) frames: Vec<Vec<bool>>,
    pub(super) ordered: Vec<Vec<usize>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SelectedImageAtom {
    pub atom_index: usize,
    pub image_offset: [i32; 3],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImageSelectionFrames {
    pub(super) frames: Vec<Vec<SelectedImageAtom>>,
}

impl SelectionFrames {
    pub fn new(traj: &Trajectory) -> Self {
        Self {
            frames: (0..traj.len())
                .map(|index| vec![false; traj.view(index).positions.len()])
                .collect(),
            ordered: vec![Vec::new(); traj.len()],
        }
    }

    pub fn get(&self, frame_index: usize) -> Option<&[bool]> {
        self.frames.get(frame_index).map(Vec::as_slice)
    }

    pub fn replace(&mut self, frame_index: usize, selection: Vec<bool>) {
        if frame_index < self.frames.len() && self.frames[frame_index].len() == selection.len() {
            self.frames[frame_index] = selection;
            self.ordered[frame_index] = self.frames[frame_index]
                .iter()
                .enumerate()
                .filter_map(|(index, selected)| selected.then_some(index))
                .collect();
        }
    }

    pub fn add(&mut self, frame_index: usize, selection: &[bool]) {
        if let Some(current) = self.frames.get_mut(frame_index) {
            if current.len() != selection.len() {
                return;
            }
            for (slot, selected) in current.iter_mut().zip(selection.iter().copied()) {
                *slot |= selected;
            }
            let ordered = &mut self.ordered[frame_index];
            for (index, selected) in selection.iter().copied().enumerate() {
                if selected && current[index] && !ordered.contains(&index) {
                    ordered.push(index);
                }
            }
        }
    }

    pub fn remove(&mut self, frame_index: usize, selection: &[bool]) {
        if let Some(current) = self.frames.get_mut(frame_index) {
            if current.len() != selection.len() {
                return;
            }
            for (slot, selected) in current.iter_mut().zip(selection.iter().copied()) {
                if selected {
                    *slot = false;
                }
            }
            self.ordered[frame_index].retain(|index| current.get(*index).copied().unwrap_or(false));
        }
    }

    pub fn clear(&mut self, frame_index: usize) {
        if let Some(current) = self.frames.get_mut(frame_index) {
            current.fill(false);
            self.ordered[frame_index].clear();
        }
    }

    pub fn append_empty_for_atom_count(&mut self, atom_count: usize) {
        self.frames.push(vec![false; atom_count]);
        self.ordered.push(Vec::new());
    }

    pub fn selected_main_images(&self, frame_index: usize) -> Vec<SelectedImageAtom> {
        self.selected_indices(frame_index)
            .into_iter()
            .map(|atom_index| SelectedImageAtom {
                atom_index,
                image_offset: [0, 0, 0],
            })
            .collect()
    }

    pub fn selected_indices(&self, frame_index: usize) -> Vec<usize> {
        self.ordered.get(frame_index).cloned().unwrap_or_default()
    }

    pub(crate) fn set_order(&mut self, frame_index: usize, ordered: Vec<usize>) {
        if frame_index < self.ordered.len() {
            self.ordered[frame_index] = ordered;
        }
    }

    pub(crate) fn mask_from_images(atom_count: usize, atoms: &[SelectedImageAtom]) -> Vec<bool> {
        let mut mask = vec![false; atom_count];
        for atom in atoms {
            if atom.atom_index < atom_count {
                mask[atom.atom_index] = true;
            }
        }
        mask
    }

    pub(crate) fn ordered_atoms_from_images(atoms: &[SelectedImageAtom]) -> Vec<usize> {
        let mut ordered = Vec::new();
        for atom in atoms {
            if !ordered.contains(&atom.atom_index) {
                ordered.push(atom.atom_index);
            }
        }
        ordered
    }
}

impl ImageSelectionFrames {
    pub fn new(frame_count: usize) -> Self {
        Self {
            frames: vec![Vec::new(); frame_count],
        }
    }

    pub fn get(&self, frame_index: usize) -> Option<&[SelectedImageAtom]> {
        self.frames.get(frame_index).map(Vec::as_slice)
    }

    pub fn selected(&self, frame_index: usize) -> Vec<SelectedImageAtom> {
        self.frames.get(frame_index).cloned().unwrap_or_default()
    }

    pub fn replace(&mut self, frame_index: usize, selection: Vec<SelectedImageAtom>) {
        if frame_index < self.frames.len() {
            self.frames[frame_index] = dedup_image_selection(selection);
        }
    }

    pub fn add(&mut self, frame_index: usize, selection: &[SelectedImageAtom]) {
        let Some(current) = self.frames.get_mut(frame_index) else {
            return;
        };
        for atom in selection.iter().copied() {
            if !current.contains(&atom) {
                current.push(atom);
            }
        }
    }

    pub fn remove(&mut self, frame_index: usize, selection: &[SelectedImageAtom]) {
        let Some(current) = self.frames.get_mut(frame_index) else {
            return;
        };
        current.retain(|atom| !selection.contains(atom));
    }

    pub fn clear(&mut self, frame_index: usize) {
        if frame_index < self.frames.len() {
            self.frames[frame_index].clear();
        }
    }

    pub fn append_empty_frame(&mut self) {
        self.frames.push(Vec::new());
    }

    pub(super) fn replace_single(&mut self, frame_index: usize, atom: SelectedImageAtom) -> bool {
        let Some(current) = self.frames.get_mut(frame_index) else {
            return false;
        };
        if current.as_slice() == [atom] {
            return false;
        }
        current.clear();
        current.push(atom);
        true
    }

    pub(super) fn toggle(&mut self, frame_index: usize, atom: SelectedImageAtom) -> bool {
        let Some(current) = self.frames.get_mut(frame_index) else {
            return false;
        };
        if let Some(index) = current.iter().position(|selected| *selected == atom) {
            current.remove(index);
        } else {
            current.push(atom);
        }
        true
    }
}

fn dedup_image_selection(selection: Vec<SelectedImageAtom>) -> Vec<SelectedImageAtom> {
    let mut deduped = Vec::new();
    for atom in selection {
        if !deduped.contains(&atom) {
            deduped.push(atom);
        }
    }
    deduped
}
