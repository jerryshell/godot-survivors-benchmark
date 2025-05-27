use crate::*;

#[derive(GodotClass)]
#[class(base=Node, init)]
pub struct ShotgunAbility {
    base: Base<Node>,

    #[export]
    game_level: Option<Gd<GameLevel>>,

    #[export]
    bullet_scene: Option<Gd<PackedScene>>,

    #[export]
    attack_timer: Option<Gd<Timer>>,
}

#[godot_api]
impl INode for ShotgunAbility {
    fn ready(&mut self) {
        let this = self.to_gd();
        let attack = this.callable("attack");

        let attack_timer = self.attack_timer.as_mut().unwrap();
        attack_timer.connect("timeout", &attack);
    }
}

#[godot_api]
impl ShotgunAbility {
    #[func]
    fn attack(&mut self) {
        let enemy = match self.find_nearest_enemy() {
            Some(enemy) => enemy,
            None => return,
        };

        self.spawn_bullet(enemy.clone(), 0.0);
        self.spawn_bullet(enemy.clone(), -real_consts::PI / 9.0);
        self.spawn_bullet(enemy, real_consts::PI / 9.0);
    }

    #[func]
    fn spawn_bullet(&mut self, target: Gd<Enemy>, rotation_offset: f32) {
        let mut game_level = self.game_level.as_mut().unwrap().bind_mut();

        let pos = {
            let player = game_level.player.as_ref().unwrap().bind();
            player.base().get_global_position()
        };

        let object_root = game_level.object_root.as_mut().unwrap();

        let move_dir = pos
            .direction_to(target.get_global_position())
            .rotated(rotation_offset);

        let bullet_scene = self.bullet_scene.as_ref().unwrap();
        let mut bullet = bullet_scene.instantiate_as::<Bullet>();
        bullet.set_global_position(pos);
        bullet.set("move_dir", &move_dir.to_variant());
        object_root.add_child(&bullet);
    }

    #[func]
    fn find_nearest_enemy(&mut self) -> Option<Gd<Enemy>> {
        let game_level = self.game_level.as_ref().unwrap().bind();
        let player_position = {
            let player = game_level.player.as_ref().unwrap().bind();
            player.base().get_global_position()
        };

        let mut enemy: Option<Gd<Enemy>> = None;
        let mut enemy_distance_squared = real::INFINITY;

        let enemy_list = self.base().get_tree().unwrap().get_nodes_in_group("enemy");
        for current_enemy in enemy_list.iter_shared() {
            let current_enemy: Gd<Enemy> = current_enemy.cast();
            let distance_squared = current_enemy
                .get_global_position()
                .distance_squared_to(player_position);
            if distance_squared < enemy_distance_squared {
                enemy = Some(current_enemy);
                enemy_distance_squared = distance_squared;
            }
        }

        enemy
    }
}
