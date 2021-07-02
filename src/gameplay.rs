use bevy::{
    prelude::*,
    log::LogPlugin,
};
use crate::consts::*;
use crate::types::*;
use crate::ScoreResource;

// Keeps the textures and materials for note
struct NoteMaterialResource {
    don_texture: Handle<ColorMaterial>,
    kat_texture: Handle<ColorMaterial>,
    target_texture: Handle<ColorMaterial>,
}
impl FromWorld for NoteMaterialResource {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();

        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let don_handle = asset_server.load("images/don-small.png");
        let kat_handle = asset_server.load("images/kat-small.png");
        let target_handle = asset_server.load("images/note-target.png");
        NoteMaterialResource {
            don_texture: materials.add(don_handle.into()),
            kat_texture: materials.add(kat_handle.into()),
            target_texture: materials.add(target_handle.into()),
        }
    }
}

// Texture for lanes
pub struct LaneMaterialResource {
    lane_texture: Handle<ColorMaterial>,
    front_lane_texture: Handle<ColorMaterial>,
}
impl FromWorld for LaneMaterialResource {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();

        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let lane_handle = asset_server.load("images/note-lane.png");
        let front_lane_handle = asset_server.load("images/note-lane-front.png");
        LaneMaterialResource {
            lane_texture: materials.add(lane_handle.into()),
            front_lane_texture: materials.add(front_lane_handle.into()),
        }
    }
}

// Spawn Lane
struct Lanes;

fn setup_lane(mut commands: Commands, materials: Res<LaneMaterialResource>) {
    let transform = Transform::from_translation(Vec3::new(0.,LANE_Y_AXIS, 1.));
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.lane_texture.clone(),
            sprite: Sprite::new(Vec2::new(1366., 200.)),
            transform,
            ..Default::default()
        })
        .insert(Lanes);
}

fn setup_front_lane(mut commands: Commands, materials: Res<LaneMaterialResource>) {
    let transform = Transform::from_translation(Vec3::new(0.,LANE_Y_AXIS, 2.));
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.front_lane_texture.clone(),
            sprite: Sprite::new(Vec2::new(1366., 200.)),
            transform,
            ..Default::default()
        })
        .insert(Lanes);
}


// Note component
struct Note {
    speed: Speed,
    types: NoteTypes,
}

// Spawn notes
fn spawn_notes(
    mut commands: Commands,
    mut song_config: ResMut<SongConfig>,
    materials: Res<NoteMaterialResource>,
    time: Res<Time>,
) {
    // We get the current time since startup (secs) and the time since the last iteration (secs_last),
    // this way we check if any notes should spawn in this window

    // Set songs to start after 3 seconds
    let secs = time.seconds_since_startup() - 3.;
    let secs_last = secs - time.delta_seconds_f64();

    // Counter of how many notes we need to spawn and remove from list
    let mut remove_counter = 0;
    for note in &song_config.notes {
        // List is ordered, so we just check until an item fails
        // Check if note should be spawned at any point between last frame and this frame
        if secs_last < note.spawn_time && note.spawn_time < secs {
            remove_counter += 1;

            // Get the correct material according to note types
            let material = match note.types {
                NoteTypes::Don => materials.don_texture.clone(),
                NoteTypes::Kat =>  materials.kat_texture.clone(),
            };

            let transform = Transform::from_translation(Vec3::new(SPAWN_POSITION, 120., 1.));

            commands
                .spawn_bundle(SpriteBundle {
                    material,
                    sprite: Sprite::new(Vec2::new(100., 100.)),
                    transform,
                    ..Default::default()
                })
                .insert(Note {
                    speed: note.speed,
                    types: note.types,
                });
            debug!("Note spawned");
        } else {
            break;
        }
    }

    // Remove the notes we have spawned from the list
    for _ in 0..remove_counter {
        song_config.notes.remove(0);
    }
}

// Move the notes forward
fn move_notes(time: Res<Time>, mut query: Query<(&mut Transform, &Note)>) {
    for (mut transform, note) in query.iter_mut() {
        transform.translation.x -= time.delta_seconds() * note.speed.value();
    }
}

struct TargetNote;

fn setup_target_notes(mut commands: Commands, materials: Res<NoteMaterialResource>) {
    let transform = Transform::from_translation(Vec3::new(TARGET_POSITION, 120., 1.));
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.target_texture.clone(),
            sprite: Sprite::new(Vec2::new(90., 90.)),
            transform,
            ..Default::default()
        })
        .insert(TargetNote);
}

fn despawn_notes(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Note)>,
    keyboard_input: Res<Input<KeyCode>>,

    mut score: ResMut<ScoreResource>,
) {
    for (entity, transform, note) in query.iter() {
        let pos = transform.translation.x;

        // Check if note is inside clicking threshold
        if (TARGET_POSITION - THRESHOLD..=TARGET_POSITION + THRESHOLD).contains(&pos)
            && note.types.key_just_pressed(&keyboard_input)
        {
            commands.entity(entity).despawn();

            // Add score and correct note on score UI
            let _points = score.increase_correct(TARGET_POSITION - pos);
        }

        // Despawn notes after they leave the screen
        if pos <= 2. * TARGET_POSITION {
            commands.entity(entity).despawn();

            // Add fail note on score UI
            // TODO: Make fail score change on pass goal, not on destroy entity
            score.increase_fails();
        }
    }
}

// Notes plugin
pub struct NotesPlugin;
impl Plugin for NotesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            // Initialize Resources
            .init_resource::<NoteMaterialResource>()
            .init_resource::<LaneMaterialResource>()
            .add_system(setup_target_notes.system().after("lane"))
            .add_system(setup_lane.system().label("lane"))
            .add_system(setup_front_lane.system().label("front_lane").after("lane"))
            // Add systems
            .add_system(spawn_notes.system())
            .add_system(move_notes.system())
            .add_system(despawn_notes.system());
    }
}