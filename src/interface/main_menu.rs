use bevy_lunex::prelude::*;
use bevy::prelude::*;

/// Main menu constructor function
pub fn build_main_menu (commands: &mut Commands, asset_server: &Res<AssetServer>, ui_tree: &mut UiTree) -> Result<(), LunexError> {

    //Create temporary UI tree
    let mut temporary_tree = UiTree::new();

    //Create the widgets and handle errors
    let widget = Widget::create(&mut temporary_tree, "handle", layout::Relative::default().pack())?;
    let background = Widget::create(&mut temporary_tree, &widget.end("background"), layout::Solid::default().with_width(1920.0).with_height(1080.0).with_scaling(SolidScale::Fill).pack())?;

    //Merge the temporary tree to main tree if nothing failed so far
    ui_tree.merge(temporary_tree)?;

    //Make it visible
    widget.fetch_mut(ui_tree, "").unwrap().set_visibility(true);

    //Spawn the elements
    commands.spawn(ImageElementBundle::new(background.clone(), &ImageParams::default().with_width(Some(100.0)).with_height(Some(100.0)), asset_server.load("images/interface/main_menu/background_0.png"), Vec2::new(1920.0,1080.0)));
    commands.spawn(ImageElementBundle::new(background.clone(), &ImageParams::default().with_depth(0.1), asset_server.load("images/interface/main_menu/background_1.png"), Vec2::new(1920.0,1080.0)));
    commands.spawn(ImageElementBundle::new(background.clone(), &ImageParams::default().with_depth(0.2), asset_server.load("images/interface/main_menu/background_2.png"), Vec2::new(1920.0,1080.0)));
    commands.spawn(ImageElementBundle::new(background.clone(), &ImageParams::default().with_depth(0.3), asset_server.load("images/interface/main_menu/background_3.png"), Vec2::new(1920.0,1080.0)));
    commands.spawn(ImageElementBundle::new(background.clone(), &ImageParams::default().with_depth(0.4), asset_server.load("images/interface/main_menu/background_4.png"), Vec2::new(1920.0,1080.0)));

    Result::Ok(())
}