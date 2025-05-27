use crate::*;

#[derive(GodotClass)]
#[class(base=Area2D, init)]
pub struct Bullet {
    base: Base<Area2D>,

    #[export]
    duration_timer: Option<Gd<Timer>>,

    #[var]
    #[init(val = 300.0)]
    move_speed: f32,

    #[var]
    #[init(val = Vector2::RIGHT)]
    move_dir: Vector2,

    #[var]
    #[init(val = 50)]
    damage: i64,
}

#[godot_api]
impl IArea2D for Bullet {
    fn ready(&mut self) {
        let mut this = self.to_gd();
        let on_body_entered = this.callable("on_body_entered");
        this.connect("body_entered", &on_body_entered);

        {
            let queue_free = self.base().callable("queue_free");
            self.duration_timer
                .as_mut()
                .unwrap()
                .connect("timeout", &queue_free);
        }

        let angle = self.move_dir.angle();
        let mut base = self.base_mut();
        base.set_rotation(angle);
    }

    fn physics_process(&mut self, delta: f64) {
        let velocity = self.move_speed * self.move_dir;
        self.base_mut().translate(velocity * delta as f32);
    }
}

#[godot_api]
impl Bullet {
    #[func]
    fn on_body_entered(&mut self, body: Gd<Node2D>) {
        let mut enemy: Gd<Enemy> = body.cast();
        let mut enemy = enemy.bind_mut();

        if enemy.hp <= 0 {
            return;
        }

        enemy.take_damage(self.damage);

        self.base_mut().call_deferred("queue_free", &[]);
    }
}
