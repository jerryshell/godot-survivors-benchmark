use crate::*;

#[derive(GodotClass)]
#[class(base=Node2D, init)]
pub struct Explosion {
    base: Base<Node2D>,

    #[export]
    particles: Option<Gd<GpuParticles2D>>,
}

#[godot_api]
impl INode2D for Explosion {
    fn ready(&mut self) {
        let queue_free = self.base().callable("queue_free");

        let particles = match &mut self.particles {
            Some(particles) => particles,
            None => return,
        };

        particles.connect("finished", &queue_free);
        particles.set_emitting(true);
    }
}
