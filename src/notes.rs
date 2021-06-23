use bevy::prelude::*;
use crate::consts::*;

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
struct Note;

// Keep track of time when to spawn a new note
struct SpawnTimer(Timer);

// Spawn notes
fn spawn_notes(
    mut commands: Commands,
    materials: Res<NoteMaterialResource>,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let transform = Transform::from_translation(Vec3::new(SPAWN_POSITION, 0., 1.));
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.don_texture.clone(),
            sprite: Sprite::new(Vec2::new(140., 140.)),
            transform,
            ..Default::default()
        })
        .insert(Note);
}

// Move the notes forward
fn move_notes(time: Res<Time>, mut query: Query<(&mut Transform, &Note)>) {
    for (mut transform, _note) in query.iter_mut() {
        transform.translation.x -= time.delta_seconds() * BASE_SPEED;
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