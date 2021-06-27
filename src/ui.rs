use bevy::prelude::*;
use crate::consts::*;

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

// Setup time UI
fn setup_ui(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let font: Handle<Font> = asset_server.load("fonts/DFPKanTeiRyu-XB.ttf");
    let material = color_materials.add(Color::NONE.into());

    commands
        // Time text node
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(10.),
                    top: Val::Px(10.),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: material.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Time: 0.0",
                        TextStyle {
                            font_size: 40.0,
                            font: font.clone(),
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                })
                .insert(TimeText);
        })
        .commands()
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(10.),
                    bottom: Val::Px(10.),
                    ..Default::default()
                },
                ..Default::default()
            },
            material,
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Score: 0. Corrects: 0. Fails: 0",
                        TextStyle {
                            font_size: 40.0,
                            font: font.clone(),
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                })
                .insert(ScoreText);
        });
}

struct TimeText;

fn update_time_text(time: Res<Time>, mut query: Query<(&mut Text, &TimeText)>) {
    // Song must start 3 seconds after real time
    let secs = time.seconds_since_startup() - 3.;

    // Don't do anything before the song starts
    if secs < 0. {
        return;
    }

    for (mut text, _marker) in query.iter_mut() {
        text.sections[0].value = format!("Time: {:.2}", secs);
    }
}

struct ScoreText;

// UI plugin
pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<UIMaterialResource>()
            .add_startup_system(setup_lane.system())
            .add_startup_system(setup_ui.system())
            .add_system(update_time_text.system());
    }
}