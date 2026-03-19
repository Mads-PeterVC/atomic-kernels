#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlaybackSpeedPreset {
    pub label: &'static str,
    pub frames_per_second: f32,
}

const PLAYBACK_SPEED_PRESETS: [PlaybackSpeedPreset; 3] = [
    PlaybackSpeedPreset {
        label: "1x",
        frames_per_second: 2.0,
    },
    PlaybackSpeedPreset {
        label: "2x",
        frames_per_second: 5.0,
    },
    PlaybackSpeedPreset {
        label: "4x",
        frames_per_second: 10.0,
    },
];

#[derive(bevy::prelude::Resource, Clone, Debug, PartialEq)]
pub struct PlaybackState {
    pub is_playing: bool,
    pub speed_index: usize,
    pub current_frame: usize,
    pub total_frames: usize,
    pub follow_tail: bool,
}

impl Default for PlaybackState {
    fn default() -> Self {
        Self {
            is_playing: false,
            speed_index: 1,
            current_frame: 0,
            total_frames: 0,
            follow_tail: false,
        }
    }
}

impl PlaybackState {
    pub fn speed_presets() -> &'static [PlaybackSpeedPreset] {
        &PLAYBACK_SPEED_PRESETS
    }

    pub fn current_speed(&self) -> PlaybackSpeedPreset {
        PLAYBACK_SPEED_PRESETS
            .get(self.speed_index)
            .copied()
            .unwrap_or(PLAYBACK_SPEED_PRESETS[1])
    }

    pub fn set_speed_index(&mut self, index: usize) {
        if index < PLAYBACK_SPEED_PRESETS.len() {
            self.speed_index = index;
        }
    }

    pub fn scrubber_fraction(&self) -> f32 {
        if self.total_frames <= 1 {
            return 0.0;
        }

        self.current_frame as f32 / (self.total_frames.saturating_sub(1)) as f32
    }
}
