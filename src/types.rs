use crate::consts::*;
use bevy::input::{keyboard::KeyCode, Input};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NoteTypes {
    Don,
    Kat
}
impl NoteTypes {
    // Checks if a key that corresponds to a note has been pressed
    pub fn key_just_pressed(&self, input: &Input<KeyCode>) -> bool {
        let keys = match self {
            NoteTypes::Don => [KeyCode::F, KeyCode::J],
            NoteTypes::Kat => [KeyCode::R, KeyCode::I],
        };

        keys.iter().any(|code| input.just_pressed(*code))
    }

    /// Returns the y coordinate for a note
    pub fn y(&self) -> f32 {
        match self {
            NoteTypes::Don => 150.,
            NoteTypes::Kat => 150.,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Speed {
    Slow,
    Medium,
    Fast,
}
impl Speed {
    /// Returns actual speed at which the note should move
    pub fn value(&self) -> f32 {
        BASE_SPEED * self.multiplier()
    }
    /// Speed multiplier
    pub fn multiplier(&self) -> f32 {
        match self {
            Speed::Slow => 1.,
            Speed::Medium => 1.2,
            Speed::Fast => 1.5,
        }
    }
}

#[derive(Clone, Copy, Debug)]
// Keep track of when each note should spawn and it's speed
pub struct NoteTime {
    pub spawn_time: f64,
    pub speed: Speed,
}

#[derive(Debug)]
pub struct SongConfig {
    pub notes: Vec<NoteTime>,
}

impl NoteTime {
    fn new(click_time: f64, speed: Speed) -> Self {
        let speed_value = speed.value();
        Self {
            spawn_time: click_time - (DISTANCE / speed_value) as f64,
            speed
        }
    }
}

pub fn load_config() -> SongConfig {
    SongConfig {
        notes: vec![
            NoteTime::new(1., Speed::Slow),
            NoteTime::new(2., Speed::Slow),
            NoteTime::new(3., Speed::Slow),
            NoteTime::new(4., Speed::Medium),
            NoteTime::new(5., Speed::Fast),
        ],
    }
}