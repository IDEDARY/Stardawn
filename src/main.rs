//# Bevy imports
use bevy::prelude::*;
use bevy_lunex::prelude::*;
use bevy_vector_shapes::prelude::*;


mod interface;
use interface::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set (
            WindowPlugin {
                primary_window: Some(Window {
                    title: "Stardawn".into(),
                    ..default()
                }),
                ..default()
            }
        ))
        .add_plugins(Shape2dPlugin::default())
        .add_plugins(LunexUiPlugin)

        .add_systems(Startup, setup)

        .add_systems(Update, vector_rectangle_update.after(element_update))

        //Debug
        //.add_plugins(LunexUiDebugPlugin)

        .add_plugins(InterfacePlugin)
        .add_plugins(MainMenuPlugin)

        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    //# Spawn the camera
    commands.spawn((
        Camera2dBundle {
            transform: Transform {
                translation: Vec3 { x: 0., y: 0., z: 1000. },
                ..default()
            },
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: bevy::core_pipeline::tonemapping::Tonemapping::AcesFitted,
            ..default()
        },
        bevy::core_pipeline::bloom::BloomSettings {
            intensity: 0.2,
            low_frequency_boost: 0.8,
            low_frequency_boost_curvature: 0.95,
            high_pass_frequency: 0.9,
            prefilter_settings: bevy::core_pipeline::bloom::BloomPrefilterSettings {
                threshold: 0.25,
                threshold_softness: 0.1,
            },
            composite_mode: bevy::core_pipeline::bloom::BloomCompositeMode::Additive,
        }
    ));

    
    let mut ui_tree = UiTree::new();
    build_main_menu(&mut commands, &asset_server, &mut ui_tree).unwrap();
    

    println!("{}", ui_tree.get_map_debug());
    println!("{}", ui_tree.get_map());
    commands.spawn (ui_tree);
    

}

pub trait Pastel {
    fn pastel(&self) -> Color;
}

impl Pastel for Color {
    fn pastel(&self) -> Color {
        *self + Color::WHITE * 0.25
    }
}

#[derive(Component)]
pub struct VectorElementRectangle;
pub fn vector_rectangle_update (mut painter: ShapePainter, query: Query<(&Transform, &VectorElementRectangle)>) {
    for (transform, _) in &query {

        painter.set_translation(transform.translation);
        painter.set_scale(Vec3::splat(1.0));

        let ww = transform.scale.x;
        let hh = transform.scale.y;

        //painter.hollow = true;
        painter.color = Color::MIDNIGHT_BLUE.pastel();
        painter.thickness = 10.0;
        painter.corner_radii = Vec4::splat(20.0);
        painter.rect(Vec2::new(ww, hh));

        /*painter.set_scale(transform.scale);

        painter.hollow = true;
        painter.color = Color::MIDNIGHT_BLUE;
        painter.thickness = 5.0;
        painter.corner_radii = Vec4::splat(10.0);
        painter.rect(Vec2::splat(20.0));
    
        painter.translate(Vec3::new(10.0, 10.0, 0.0));
    
        painter.hollow = true;
        painter.color = Color::SEA_GREEN;
        painter.thickness = 5.0;
        painter.corner_radii = Vec4::splat(10.0);
        painter.rect(Vec2::splat(20.0));*/
    }
}