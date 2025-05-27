using Godot;

public partial class Explosion : Node2D
{
    [Export]
    public GpuParticles2D Particles;

    public override void _Ready()
    {
        Particles.Finished += QueueFree;
        Particles.Emitting = true;
    }
}
