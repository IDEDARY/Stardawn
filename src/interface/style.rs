use bevy::prelude::*;
use mathio::tween;

/// # Fast Flicker Effect
#[derive(Component, Default)]
pub struct FastFlickerEffect {
    x: f32,
    x_speed: f32,
    x_min: f32,
    x_max: f32,
}
impl FastFlickerEffect {
    pub fn new (x_offset: f32, x_speed: f32, x_min: f32, x_max: f32) -> FastFlickerEffect {
        FastFlickerEffect {
            x: x_offset,
            x_speed,
            x_min,
            x_max,
        }
    }
}
pub fn fast_flicker_effect_update (mut query: Query<(&mut Sprite, &mut FastFlickerEffect)>) {
    for (mut sprite, mut flicker) in &mut query {
        flicker.x += flicker.x_speed;
        let alpha = tween(flicker.x_min, flicker.x_max, flicker.x.sin()/2.0 + 0.5);
        sprite.color.set_a(alpha);
    }
}



pub struct InterfacePlugin;
impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, fast_flicker_effect_update);
    }
}