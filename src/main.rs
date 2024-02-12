use bevy::prelude::*;

mod camera;
mod player;
mod world;

use camera::CameraPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlayerPlugin, CameraPlugin, WorldPlugin))
        .run();
}
