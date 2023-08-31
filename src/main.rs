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
                    //transparent: true,
                    ..default()
                }),
                ..default()
            }
        ))
        //.insert_resource(ClearColor(Color::NONE))
        .add_plugins(Shape2dPlugin::default())
        .add_plugins(LunexUiPlugin)

        .add_systems(Startup, setup)

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