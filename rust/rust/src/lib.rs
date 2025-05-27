pub mod bullet;
pub mod control_panel;
pub mod enemy;
pub mod explosion;
pub mod game_level;
pub mod player;
pub mod shotgun_ability;

use bullet::*;
use enemy::*;
use explosion::*;
use game_level::*;
use godot::classes::*;
use godot::prelude::*;
use player::*;

struct Sbrs;

#[gdextension]
unsafe impl ExtensionLibrary for Sbrs {}
