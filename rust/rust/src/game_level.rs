use godot::global::randf_range;

use crate::*;

#[derive(GodotClass)]
#[class(base=Node, init)]
pub struct GameLevel {
    base: Base<Node>,

    #[export]
    enemy_scene: Option<Gd<PackedScene>>,

    #[export]
    explosion_scene: Option<Gd<PackedScene>>,

    #[export]
    enemy_spawn_timer: Option<Gd<Timer>>,

    #[export]
    pub object_root: Option<Gd<Node2D>>,

    #[var]
    pub player: Option<Gd<Player>>,
}

#[godot_api]
impl INode for GameLevel {
    fn ready(&mut self) {
        let this = self.to_gd();
        let on_enemy_spawn_timer_timeout = this.callable("on_enemy_spawn_timer_timeout");

        let enemy_spawn_timer = self.enemy_spawn_timer.as_mut().unwrap();
        enemy_spawn_timer.connect("timeout", &on_enemy_spawn_timer_timeout);
    }
}

#[godot_api]
impl GameLevel {
    #[func]
    pub fn spawn_enemy(&mut self) {
        let this = self.to_gd();

        let player = self.player.as_ref().unwrap();
        let player_position = player.get_global_position();
        let enemy_position = player_position
            + Vector2::RIGHT.rotated(randf_range(0.0, real_consts::TAU as f64) as f32) * 400.0;

        let enemy_scene = self.enemy_scene.as_ref().unwrap();
        let mut enemy = enemy_scene.instantiate_as::<Enemy>();
        enemy.set("game_level", &this.to_variant());
        enemy.set_global_position(enemy_position);

        let object_root = self.object_root.as_mut().unwrap();
        object_root.add_child(&enemy);
    }

    #[func]
    pub fn spawn_explosion(&mut self, pos: Vector2) {
        let explosion_scene = self.explosion_scene.as_ref().unwrap();
        let mut explosion = explosion_scene.instantiate_as::<Explosion>();
        explosion.set_global_position(pos);

        let object_root = self.object_root.as_mut().unwrap();
        object_root.add_child(&explosion);
    }

    #[func]
    fn on_enemy_spawn_timer_timeout(&mut self) {
        self.spawn_enemy();
    }
}
