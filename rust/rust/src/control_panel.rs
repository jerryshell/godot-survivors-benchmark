use crate::*;

#[derive(GodotClass)]
#[class(base=CanvasLayer, init)]
pub struct ControlPanel {
    base: Base<CanvasLayer>,

    #[export]
    game_level: Option<Gd<GameLevel>>,

    #[export]
    fps_label: Option<Gd<Label>>,

    #[export]
    enemy_count_label: Option<Gd<Label>>,

    #[export]
    spawn_enemy_button: Option<Gd<Button>>,

    #[export]
    spawn_enemy_x10_button: Option<Gd<Button>>,

    #[export]
    spawn_enemy_x100_button: Option<Gd<Button>>,
}

#[godot_api]
impl ICanvasLayer for ControlPanel {
    fn ready(&mut self) {
        let this = self.to_gd();
        let spawn_enemy = this.callable("spawn_enemy");
        let spawn_enemy_x1 = spawn_enemy.bind(&[1.to_variant()]);
        let spawn_enemy_x10 = spawn_enemy.bind(&[10.to_variant()]);
        let spawn_enemy_x100 = spawn_enemy.bind(&[100.to_variant()]);

        let spawn_enemy_button = self.spawn_enemy_button.as_mut().unwrap();
        spawn_enemy_button.connect("pressed", &spawn_enemy_x1);

        let spawn_enemy_x10_button = self.spawn_enemy_x10_button.as_mut().unwrap();
        spawn_enemy_x10_button.connect("pressed", &spawn_enemy_x10);

        let spawn_enemy_x100_button = self.spawn_enemy_x100_button.as_mut().unwrap();
        spawn_enemy_x100_button.connect("pressed", &spawn_enemy_x100);
    }

    fn process(&mut self, _delta: f64) {
        let fps_label = self.fps_label.as_mut().unwrap();
        fps_label.set_text(&GString::from(format!(
            "FPS: {}",
            Engine::singleton().get_frames_per_second()
        )));

        let tree = self.base().get_tree().unwrap();
        let enemy_count_label = self.enemy_count_label.as_mut().unwrap();
        enemy_count_label.set_text(&GString::from(format!(
            "Enemy Count: {}",
            tree.get_node_count_in_group("enemy"),
        )))
    }
}

#[godot_api]
impl ControlPanel {
    #[func]
    fn spawn_enemy(&mut self, count: i64) {
        let mut game_level = self.game_level.as_mut().unwrap().bind_mut();
        for _ in 0..count {
            game_level.spawn_enemy();
        }
    }
}
