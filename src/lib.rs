#[derive(Debug, Clone, PartialEq)]
pub struct VideoClip {
    pub path: String,
    pub start_secs: f64,
    pub end_secs: f64,
}

#[derive(Debug, Default)]
pub struct ReelEditor {
    clips: Vec<VideoClip>,
}

impl ReelEditor {
    pub fn new() -> Self { Self { clips: Vec::new() } }
    pub fn add_clip(&mut self, path: &str, start: f64, end: f64) {
        self.clips.push(VideoClip { path: path.into(), start_secs: start, end_secs: end });
    }
    pub fn total_duration(&self) -> f64 {
        self.clips.iter().map(|c| (c.end_secs - c.start_secs).max(0.0)).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reel_editor_timeline_duration_aaa() {
        // Arrange
        let mut reel = ReelEditor::new();
        // Act
        reel.add_clip("intro.mp4", 0.0, 5.5);
        reel.add_clip("main.mp4", 10.0, 25.0);
        // Assert
        assert!((reel.total_duration() - 20.5).abs() < 1e-6);
    }
}
