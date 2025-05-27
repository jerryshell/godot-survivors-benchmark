use crate::*;

#[derive(GodotClass)]
#[class(base=CharacterBody2D, init)]
pub struct Player {
    base: Base<CharacterBody2D>,

    #[export]
    game_level: Option<Gd<GameLevel>>,

    #[export]
    sprite: Option<Gd<Sprite2D>>,

    #[export]
    animation_player: Option<Gd<AnimationPlayer>>,

    #[export]
    #[init(val = 150.0)]
    move_speed: f32,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn ready(&mut self) {
        let this = self.to_gd();
        let mut game_level = match &mut self.game_level {
            Some(game_level) => game_level.bind_mut(),
            None => return,
        };
        game_level.player = Some(this);
    }

    fn physics_process(&mut self, _delta: f64) {
        let move_dir =
            Input::singleton().get_vector("move_left", "move_right", "move_up", "move_down");

        if move_dir != Vector2::ZERO {
            let animation_player = self.animation_player.as_mut().unwrap();
            animation_player.play_ex().name("run").done();
        }

        let speed = self.move_speed;
        let mut base = self.base_mut();
        base.set_velocity(move_dir * speed);

        base.move_and_slide();
    }
}
