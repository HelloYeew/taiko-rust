use bevy::prelude::*;

/// Keeps the textures and materials for note
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