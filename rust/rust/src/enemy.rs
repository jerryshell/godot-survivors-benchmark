use crate::*;

#[derive(GodotClass)]
#[class(base=CharacterBody2D, init)]
pub struct Enemy {
    base: Base<CharacterBody2D>,

    #[export]
    game_level: Option<Gd<GameLevel>>,

    #[export]
    sprite: Option<Gd<Sprite2D>>,

    #[export]
    animation_player: Option<Gd<AnimationPlayer>>,

    #[export]
    #[init(val = 80.0)]
    move_speed: f32,

    #[export]
    #[init(val = 100)]
    pub hp: i64,
}

#[godot_api]
impl ICharacterBody2D for Enemy {
    fn physics_process(&mut self, _delta: f64) {
        let global_position = self.base().get_global_position();

        let move_dir = {
            let game_level = match &mut self.game_level {
                Some(game_level) => game_level.bind(),
                None => return,
            };
            let player = match &game_level.player {
                Some(player) => player.bind(),
                None => return,
            };
            global_position.direction_to(player.base().get_global_position())
        };

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

#[godot_api]
impl Enemy {
    #[func]
    pub fn take_damage(&mut self, damage: i64) {
        if self.hp <= 0 {
            return;
        }

        self.hp -= damage;
        self.flash();

        let global_position = self.base().get_global_position();

        if self.hp <= 0 {
            {
                let mut game_level = self.game_level.as_mut().unwrap().bind_mut();
                game_level.spawn_explosion(global_position);
            }
            self.base_mut().call_deferred("queue_free", &[]);
        }
    }

    #[func]
    fn flash(&mut self) {
        let mut tween = { self.base_mut().create_tween().unwrap() };
        tween.set_ease(tween::EaseType::IN);
        {
            tween.tween_property(
                &self.to_gd(),
                "modulate",
                &Color {
                    r: 10.0,
                    g: 10.0,
                    b: 10.0,
                    a: 1.0,
                }
                .to_variant(),
                0.2,
            );
        }
        {
            tween.tween_property(&self.to_gd(), "modulate", &Color::WHITE.to_variant(), 0.2);
        }
    }
}
