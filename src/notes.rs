use bevy::prelude::*;
use crate::consts::*;
use crate::types::*;

// Keeps the textures and materials for note
struct NoteMaterialResource {
    don_texture: Handle<ColorMaterial>,
    kat_texture: Handle<ColorMaterial>,
    goal_texture: Handle<ColorMaterial>,
}
impl FromWorld for NoteMaterialResource {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();

        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let don_handle = asset_server.load("images/don-small.png");
        let kat_handle = asset_server.load("images/kat-small.png");
        let goal_handle = asset_server.load("images/note-goal.png");
        NoteMaterialResource {
            don_texture: materials.add(don_handle.into()),
            kat_texture: materials.add(kat_handle.into()),
            goal_texture: materials.add(goal_handle.into()),
        }
    }
}

// Note component
struct Note {
    speed: Speed,
}

// Keep track of time when to spawn a new note
struct SpawnTimer(Timer);

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

            // Get the correct material according to speed
            let material = match note.speed {
                Speed::Slow => materials.don_texture.clone(),
                Speed::Medium => materials.don_texture.clone(),
                Speed::Fast => materials.don_texture.clone(),
            };

            let mut transform = Transform::from_translation(Vec3::new(SPAWN_POSITION, 150., 1.));

            commands
                .spawn_bundle(SpriteBundle {
                    material,
                    sprite: Sprite::new(Vec2::new(100., 100.)),
                    transform,
                    ..Default::default()
                })
                .insert(Note {
                    speed: note.speed,
                });
        } else {
            break;
        }
    }

    // Remove the arrows we have spawned from the list
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

// Notes plugin
pub struct NotesPlugin;
impl Plugin for NotesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            // Initialize Resources
            .init_resource::<NoteMaterialResource>()
            .insert_resource(SpawnTimer(Timer::from_seconds(1.0, true)))
            // Add systems
            .add_system(spawn_notes.system())
            .add_system(move_notes.system());
    }
}