use smallvec::SmallVec;
use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq)]
pub struct Face {
    pub atoms: SmallVec<[usize; 4]>,
    pub color: [f32; 4],
}

#[derive(Clone, Debug, PartialEq)]
pub struct FaceList {
    faces: Vec<Face>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FaceFrames {
    frames: Vec<Option<FaceList>>,
}

impl Face {
    pub fn new(atoms: impl IntoIterator<Item = usize>, color: [f32; 4]) -> Option<Self> {
        let atoms: SmallVec<[usize; 4]> = atoms.into_iter().collect();
        if atoms.len() < 3 {
            return None;
        }
        let unique: BTreeSet<_> = atoms.iter().copied().collect();
        if unique.len() != atoms.len() {
            return None;
        }
        Some(Self { atoms, color })
    }

    pub fn color(self) -> bevy::color::Color {
        bevy::color::Color::srgba(self.color[0], self.color[1], self.color[2], self.color[3])
    }

    fn canonical_atoms(&self) -> Vec<usize> {
        canonicalize_face_atoms(&self.atoms)
    }
}

impl FaceList {
    pub fn new(faces: impl IntoIterator<Item = Face>) -> Self {
        let mut seen = BTreeSet::new();
        let mut canonical = Vec::new();

        for face in faces {
            let key = face.canonical_atoms();
            if seen.insert(key) {
                canonical.push(face);
            }
        }

        Self { faces: canonical }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Face> {
        self.faces.iter()
    }

    pub fn len(&self) -> usize {
        self.faces.len()
    }

    pub fn is_empty(&self) -> bool {
        self.faces.is_empty()
    }

    pub fn validated_for_atom_count(&self, atom_count: usize) -> Self {
        Self::new(
            self.faces
                .iter()
                .filter(|face| face.atoms.iter().all(|&index| index < atom_count))
                .cloned(),
        )
    }

    pub fn merged(&self, other: &FaceList) -> Self {
        Self::new(self.iter().cloned().chain(other.iter().cloned()))
    }

    pub fn without(&self, other: &FaceList) -> Self {
        let removals: BTreeSet<_> = other.iter().map(Face::canonical_atoms).collect();
        Self::new(
            self.faces
                .iter()
                .filter(|face| !removals.contains(&face.canonical_atoms()))
                .cloned(),
        )
    }
}

impl FaceFrames {
    pub fn new(frame_count: usize) -> Self {
        Self {
            frames: vec![None; frame_count],
        }
    }

    pub fn set(&mut self, frame_index: usize, faces: FaceList) {
        if frame_index < self.frames.len() {
            self.frames[frame_index] = Some(faces);
        }
    }

    pub fn add(&mut self, frame_index: usize, faces: FaceList) {
        if frame_index >= self.frames.len() {
            return;
        }
        let merged = match self.frames[frame_index].take() {
            Some(existing) => existing.merged(&faces),
            None => faces,
        };
        self.frames[frame_index] = Some(merged);
    }

    pub fn remove(&mut self, frame_index: usize, faces: &FaceList) {
        if frame_index >= self.frames.len() {
            return;
        }
        if let Some(existing) = self.frames[frame_index].take() {
            let updated = existing.without(faces);
            self.frames[frame_index] = (!updated.is_empty()).then_some(updated);
        }
    }

    pub fn clear(&mut self, frame_index: usize) {
        if frame_index < self.frames.len() {
            self.frames[frame_index] = None;
        }
    }

    pub fn get(&self, frame_index: usize) -> Option<&FaceList> {
        self.frames
            .get(frame_index)
            .and_then(|faces| faces.as_ref())
    }

    pub fn resize(&mut self, frame_count: usize) {
        self.frames.resize(frame_count, None);
    }
}

fn canonicalize_face_atoms(atoms: &[usize]) -> Vec<usize> {
    let forward = minimum_face_rotation(atoms);
    let mut reversed = atoms.to_vec();
    reversed.reverse();
    let reversed = minimum_face_rotation(&reversed);
    if reversed < forward {
        reversed
    } else {
        forward
    }
}

fn minimum_face_rotation(atoms: &[usize]) -> Vec<usize> {
    let mut best = atoms.to_vec();
    for shift in 1..atoms.len() {
        let rotated: Vec<usize> = atoms
            .iter()
            .cycle()
            .skip(shift)
            .take(atoms.len())
            .copied()
            .collect();
        if rotated < best {
            best = rotated;
        }
    }
    best
}
