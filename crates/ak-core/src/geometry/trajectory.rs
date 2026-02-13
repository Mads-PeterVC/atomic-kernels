use crate::{Structure, StructureView};

pub struct Trajectory {
    frames: Vec<Structure>,
}

impl Trajectory {
    pub fn new(frames: Vec<Structure>) -> Self {
        Self { frames }
    }

    pub fn view<'a>(&'a self, index: usize) -> StructureView<'a> {
        let view = self.frames[index].view();
        view
    }

    pub fn append(&mut self, frame: Structure) {
        self.frames.push(frame);
    }

    pub fn len(&self) -> usize {
        self.frames.len()
    }
}

#[cfg(test)]
mod test {

    use crate::{Structure, Trajectory};

    fn test_structure() -> Structure {
        let positions = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]].to_vec();
        let numbers = [1, 1].to_vec();
        let cell = [[10.0, 0.0, 0.0], [0.0, 10.0, 0.0], [0.0, 0.0, 10.0]];
        let pbc = [false, false, false];
        let structure = Structure::new(positions.clone(), numbers.clone(), cell, pbc);
        structure
    }

    fn test_trajectory() -> Trajectory {
        let structure = test_structure();
        let struc_vec = vec![structure.clone(), structure.clone()];
        let trajectory = Trajectory::new(struc_vec);
        trajectory
    }

    #[test]
    fn test_trajectory_len() {
        let trajectory = test_trajectory();
        assert_eq!(trajectory.len(), 2);
    }

    #[test]
    fn test_trajectory_new() {
        let structure = test_structure();
        let mut trajectory = test_trajectory();
        trajectory.append(structure);

        assert_eq!(trajectory.len(), 3);
    }
}
