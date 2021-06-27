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