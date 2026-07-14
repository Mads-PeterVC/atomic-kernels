use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BondList {
    edges: Vec<(usize, usize)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BondFrames {
    frames: Vec<Option<BondList>>,
}

impl BondList {
    pub fn new(edges: impl IntoIterator<Item = (usize, usize)>) -> Self {
        let mut canonical = BTreeSet::new();
        for (i, j) in edges {
            if i == j {
                continue;
            }
            canonical.insert((i.min(j), i.max(j)));
        }
        Self {
            edges: canonical.into_iter().collect(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &(usize, usize)> {
        self.edges.iter()
    }

    pub fn len(&self) -> usize {
        self.edges.len()
    }

    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }

    pub fn merged(&self, other: &BondList) -> Self {
        Self::new(self.iter().copied().chain(other.iter().copied()))
    }

    pub fn without(&self, other: &BondList) -> Self {
        let removals: BTreeSet<_> = other.iter().copied().collect();
        Self::new(self.iter().copied().filter(|edge| !removals.contains(edge)))
    }
}

impl BondFrames {
    pub fn new(frame_count: usize) -> Self {
        Self {
            frames: vec![None; frame_count],
        }
    }

    pub fn set(&mut self, frame_index: usize, bonds: BondList) {
        if frame_index < self.frames.len() {
            self.frames[frame_index] = Some(bonds);
        }
    }

    pub fn add(&mut self, frame_index: usize, bonds: BondList) {
        if frame_index >= self.frames.len() {
            return;
        }
        let merged = match self.frames[frame_index].take() {
            Some(existing) => existing.merged(&bonds),
            None => bonds,
        };
        self.frames[frame_index] = Some(merged);
    }

    pub fn remove(&mut self, frame_index: usize, bonds: &BondList) {
        if frame_index >= self.frames.len() {
            return;
        }
        if let Some(existing) = self.frames[frame_index].take() {
            let updated = existing.without(bonds);
            self.frames[frame_index] = (!updated.is_empty()).then_some(updated);
        }
    }

    pub fn clear(&mut self, frame_index: usize) {
        if frame_index < self.frames.len() {
            self.frames[frame_index] = None;
        }
    }

    pub fn get(&self, frame_index: usize) -> Option<&BondList> {
        self.frames
            .get(frame_index)
            .and_then(|bonds| bonds.as_ref())
    }

    pub fn resize(&mut self, frame_count: usize) {
        self.frames.resize(frame_count, None);
    }
}
