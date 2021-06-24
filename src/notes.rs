use bevy::prelude::*;
use crate::consts::*;
use crate::types::*;

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

            // Get the correct material according to speed
            let material = match note.types {
                NoteTypes::Don => materials.don_texture.clone(),
                NoteTypes::Kat =>  materials.kat_texture.clone(),
            };

            let transform = Transform::from_translation(Vec3::new(SPAWN_POSITION, 150., 1.));

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
    let transform = Transform::from_translation(Vec3::new(TARGET_POSITION, 150., 1.));
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
) {
    for (entity, transform, note) in query.iter() {
        let pos = transform.translation.x;

        // Check if note is inside clicking threshold
        if (TARGET_POSITION - THRESHOLD..=TARGET_POSITION + THRESHOLD).contains(&pos)
            && note.types.key_just_pressed(&keyboard_input)
        {
            commands.entity(entity).despawn();
        }

        // Despawn notes after they leave the screen
        if pos <= 2. * TARGET_POSITION {
            commands.entity(entity).despawn();
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
            .add_startup_system(setup_target_notes.system())
            // Add systems
            .add_system(spawn_notes.system())
            .add_system(move_notes.system())
            .add_system(despawn_notes.system());
    }
}