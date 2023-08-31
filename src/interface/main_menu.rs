use elements::*;
use super::style::*;


/// Main menu constructor function
pub fn build_main_menu (commands: &mut Commands, asset_server: &Res<AssetServer>, ui_tree: &mut UiTree) -> Result<(), LunexError> {

    // ===========================================================
    // === LAYOUT ===
    
    //Create temporary UI tree
    let mut temporary_tree = UiTree::new();

    //Create the widgets and handle errors
    let main_menu = Widget::create(&mut temporary_tree, "main_menu", RelativeLayout::default().pack())?;

    //Background
    let background = Widget::create(&mut temporary_tree, &main_menu.end("background"), SolidLayout::new().with_width(1920.0).with_height(1080.0).with_scaling(SolidScale::Fill).pack())?;

    //Menu boundary
    let boundary = Widget::create(&mut temporary_tree, &main_menu.end("boundary"), SolidLayout::new().with_width(192.0).with_height(47.0).with_vertical_anchor(0.0).with_scaling(SolidScale::Fit).pack())?;
    
    let darkness_up = Widget::create(&mut temporary_tree, &boundary.end(""), RelativeLayout::new().with_rel_1(Vec2::new(0.0, -1500.0)).with_rel_2(Vec2::new(100.0, 0.0)).pack())?;
    let darkness_down = Widget::create(&mut temporary_tree, &boundary.end(""), RelativeLayout::new().with_rel_1(Vec2::new(0.0, 100.0)).with_rel_2(Vec2::new(100.0, 1500.0)).pack())?;
    let darkness_left = Widget::create(&mut temporary_tree, &boundary.end(""), RelativeLayout::new().with_rel_1(Vec2::new(-200.0, 0.0)).with_rel_2(Vec2::new(0.0, 100.0)).pack())?;
    let darkness_right = Widget::create(&mut temporary_tree, &boundary.end(""), RelativeLayout::new().with_rel_1(Vec2::new(100.0, 0.0)).with_rel_2(Vec2::new(300.0, 100.0)).pack())?;

    //Button
    let play_button = Widget::create(&mut temporary_tree, &boundary.end("play"), RelativeLayout::new().with_rel_1(Vec2::new(43.0, 75.0)).with_rel_2(Vec2::new(57.0, 85.0)).pack())?;

    // ===========================================================
    // === MERGE THE CHANGES ===

    ui_tree.merge(temporary_tree)?;


    // ===========================================================
    // === SPAWNING ===

    '_background: {
        commands.spawn(ImageElementBundle::new(background.clone(), &ImageParams::default().with_width(Some(100.0)).with_height(Some(100.0)), asset_server.load("images/interface/main_menu/background_0.png"), Vec2::new(1920.0,1080.0)));
        commands.spawn((
            ImageElementBundle::new(background.clone(), &ImageParams::default().with_depth(0.1), asset_server.load("images/interface/main_menu/background_1.png"), Vec2::new(1920.0,1080.0)),
            FastFlickerEffect::new(1.0, 0.02, 0.0, 0.8),
        ));
        commands.spawn((
            ImageElementBundle::new(background.clone(), &ImageParams::default().with_depth(0.2), asset_server.load("images/interface/main_menu/background_2.png"), Vec2::new(1920.0,1080.0)),
            FastFlickerEffect::new(3.0, 0.02, 0.3, 1.0),
        ));
        commands.spawn((
            ImageElementBundle::new(background.clone(), &ImageParams::default().with_depth(0.3), asset_server.load("images/interface/main_menu/background_3.png"), Vec2::new(1920.0,1080.0)),
            FastFlickerEffect::new(5.0, 0.01, 0.0, 1.5),
        ));
        commands.spawn((
            ImageElementBundle::new(background.clone(), &ImageParams::default().with_depth(0.4), asset_server.load("images/interface/main_menu/background_4.png"), Vec2::new(1920.0,1080.0)),
            FastFlickerEffect::new(7.0, 0.02, 0.1, 1.2),
        ));
    }

    '_planet: {
        let scale = 1.5;
        commands.spawn((
            ImageElementBundle::new(boundary.clone(), &ImageParams::center().with_depth(0.5).at(50.0, 100.0).with_scale(100.0*scale), asset_server.load("images/interface/main_menu/planet.png"), Vec2::new(1230.0,1341.0)),
            SlowRotation::new(f32::to_radians(-0.02)),
        ));
        commands.spawn((
            ImageElementBundle::new(boundary.clone(), &ImageParams::center().with_depth(0.5).at(50.0, 100.0).with_scale(200.0*scale), asset_server.load("images/interface/main_menu/planet_ring_lowres.png"), Vec2::new(1077.0,1065.0)),
            SlowRotation::new(f32::to_radians(0.02)),
        ));
    }

    '_darkness: {
        commands.spawn(ImageElementBundle::new(boundary.clone(), &ImageParams::default().with_depth(6.0).with_width(Some(100.0)).with_height(Some(100.0)), asset_server.load("images/interface/main_menu/shadow.png"), Vec2::new(1920.0,470.0)));

        commands.spawn((
            ElementBundle::new(darkness_up.clone(), Element::fullfill().with_depth(100.0)),
            VectorElementDarkness
        ));
        commands.spawn((
            ElementBundle::new(darkness_down.clone(), Element::fullfill().with_depth(100.0)),
            VectorElementDarkness
        ));
        commands.spawn((
            ElementBundle::new(darkness_left.clone(), Element::fullfill().with_depth(100.0)),
            VectorElementDarkness
        ));
        commands.spawn((
            ElementBundle::new(darkness_right.clone(), Element::fullfill().with_depth(100.0)),
            VectorElementDarkness
        ));
    }

    '_title: {
        let text_style = TextStyle {
            font: asset_server.load("fonts/interface/dune_rise/Dune_Rise.ttf"),
            font_size: 180.0,
            color: Color::ANTIQUE_WHITE,
        };
        commands.spawn(TextElementBundle::new(boundary.clone(), &TextParams::center().with_style(&text_style).at(50.0, 30.0).with_scale(13.0), "Stardawn"));
    }
    
    commands.spawn((
        ElementBundle::new(play_button.clone(), Element::fullfill().with_depth(100.0)),
        VectorElementRectangle
    ));

    let text_style = TextStyle {
        font: asset_server.load("fonts/interface/dune_rise/Dune_Rise.ttf"),
        font_size: 40.0,
        color: Color::ANTIQUE_WHITE,
    };
    commands.spawn(TextElementBundle::new(play_button.clone(), &TextParams::center().with_style(&text_style).at(50.0, 50.0).with_scale(30.0).with_height(Some(100.0)), "start"));

    // ===========================================================
    // === RETURN ===

    Result::Ok(())
}


mod elements {
    pub use bevy::prelude::*;
    pub use bevy_lunex::prelude::*;
    pub use bevy_vector_shapes::prelude::*;


    #[derive(Component)]
    pub struct VectorElementRectangle;
    pub fn vector_rectangle_update (mut painter: ShapePainter, query: Query<(&Transform, &VectorElementRectangle)>) {
        for (transform, _) in &query {

            painter.set_translation(transform.translation);
            painter.set_scale(Vec3::splat(1.0));

            let ww = transform.scale.x;
            let hh = transform.scale.y;

            painter.hollow = true;
            painter.color = Color::ANTIQUE_WHITE;
            painter.thickness = 0.5;
            painter.corner_radii = Vec4::splat(20.0);
            painter.rect(Vec2::new(ww, hh));


        }
    }

    #[derive(Component)]
    pub struct VectorElementDarkness;
    pub fn vector_darkness_update (mut painter: ShapePainter, query: Query<(&Transform, &VectorElementDarkness)>) {
        for (transform, _) in &query {

            painter.set_translation(transform.translation);
            painter.set_scale(Vec3::splat(1.0));

            let ww = transform.scale.x;
            let hh = transform.scale.y;

            painter.color = Color::BLACK;
            painter.thickness = 1.0;
            painter.rect(Vec2::new(ww, hh));


        }
    }


    /// # Slow Rotation Effect
    #[derive(Component, Default)]
    pub struct SlowRotation {
        ang_speed: f32,
    }
    impl SlowRotation {
        pub fn new (ang_speed: f32) -> SlowRotation {
            SlowRotation {
                ang_speed,
            }
        }
    }
    pub fn slow_rotation_update (mut query: Query<(&mut Transform, &SlowRotation)>) {
        for (mut transform, rotation) in &mut query {
            transform.rotate_z(rotation.ang_speed);
        }
    }

    #[derive(Component)]
    pub struct StartButton;
    pub fn start_button_update (trees: Query<&UiTree>, cursors: Query<&Cursor>, query: Query<(&Widget, &StartButton)>) {

        for tree in &trees {
            for (widget, _) in &query {

                for cursor in &cursors {
                    if widget.is_within(tree, &cursor.position_world().as_lunex(tree.offset)).unwrap() {
                        widget.re
                    }
                }
            }
        }
    }
}

/// Plugin with all Main Menu logic systems
pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, slow_rotation_update)
            .add_systems(Update, (
                vector_rectangle_update,
                vector_darkness_update
            ).after(element_update));
    }
}