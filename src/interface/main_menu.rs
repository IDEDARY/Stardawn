use bevy_lunex::prelude::*;
use bevy::prelude::*;

/// Main menu constructor function
pub fn build_main_menu (commands: &mut Commands, asset_server: &Res<AssetServer>, ui_tree: &mut UITree) -> Result<(), String> {

    //Create temporary UI tree
    let mut temporary_tree = UITree::new();

    //Create the widgets and handle errors
    let background = Widget::create(&mut temporary_tree, "background", Layout::Solid::default().with_scaling(SolidScale::Fit).pack())?;

    //Merge the temporary tree to main tree if nothing failed so far
    ui_tree.merge(temporary_tree)?;

    //Spawn the elements
    commands.spawn(ImageElementBundle::new(background.clone(), &ImageParams::default(), asset_server.load("images/interface/main_menu/background_0"), Vec2::new(1920.0,1080.0)));

    Result::Ok(())
}











/*
commands.spawn(ImageElementBundle::new(background.clone(), &ImageParams::default().with_depth(0.1), asset_server.load("images/interface/main_menu/background_1"), Vec2::new(1920.0,1080.0)));
commands.spawn(ImageElementBundle::new(background.clone(), &ImageParams::default().with_depth(0.2), asset_server.load("images/interface/main_menu/background_2"), Vec2::new(1920.0,1080.0)));
commands.spawn(ImageElementBundle::new(background.clone(), &ImageParams::default().with_depth(0.3), asset_server.load("images/interface/main_menu/background_3"), Vec2::new(1920.0,1080.0)));
commands.spawn(ImageElementBundle::new(background.clone(), &ImageParams::default().with_depth(0.4), asset_server.load("images/interface/main_menu/background_4"), Vec2::new(1920.0,1080.0)));
*/