use std::path::PathBuf;

const DEFAULT_PREROLL_FRAMES: u32 = 4;
const DEFAULT_STABLE_FRAMES: u32 = 2;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HeadlessRenderConfig {
    pub path: PathBuf,
    pub width: u32,
    pub height: u32,
    pub preroll_frames: u32,
    pub stable_frames: u32,
}

impl HeadlessRenderConfig {
    pub fn new(path: impl Into<PathBuf>, width: u32, height: u32) -> Self {
        Self {
            path: path.into(),
            width,
            height,
            preroll_frames: DEFAULT_PREROLL_FRAMES,
            stable_frames: DEFAULT_STABLE_FRAMES,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeadlessRenderError {
    message: String,
}

impl HeadlessRenderError {
    pub(super) fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for HeadlessRenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for HeadlessRenderError {}

impl From<std::io::Error> for HeadlessRenderError {
    fn from(value: std::io::Error) -> Self {
        Self::new(value.to_string())
    }
}
