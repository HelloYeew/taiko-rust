// Speed at which a slow note moves
pub const BASE_SPEED: f32 = 200.;

// X coordinate value at which notes spawn, should be out of screen
pub const SPAWN_POSITION: f32 = 500.;

// X coordinate value where the notes should be clicked
pub const TARGET_POSITION: f32 = -200.;

// Margin of error for clicking a note
pub const THRESHOLD: f32 = 20.;

/// Total distance traveled by a note, from spawn to target
pub const DISTANCE: f32 = TARGET_POSITION - SPAWN_POSITION;

// Y Axis of lanes
pub const LANE_Y_AXIS: f32 = 100.;