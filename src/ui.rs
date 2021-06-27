use bevy::prelude::*;
use crate::consts::*;
use crate::types::*;

// Texture for UI (lanes, drum, background etc.)
struct UIMaterialResource {
    lane_texture: Handle<ColorMaterial>,
}
impl FromWorld for UIMaterialResource {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();

        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let lane_handle = asset_server.load("images/note-lane.png");
        UIMaterialResource {
            lane_texture: materials.add(lane_handle.into()),
        }
    }
}

// Spawn Lane
struct Lanes;

fn setup_lane(mut commands: Commands, materials: Res<UIMaterialResource>) {
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

// UI plugin
pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<UIMaterialResource>()
            .add_startup_system(setup_lane.system());
    }
}