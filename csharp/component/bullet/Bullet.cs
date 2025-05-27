using Godot;

public partial class Bullet : Area2D
{
    [Export]
    public Timer DurationTimer;

    public int MoveSpeed = 300;
    public Vector2 MoveDir = Vector2.Right;
    public int Damage = 50;

    public override void _Ready()
    {
        BodyEntered += OnBodyEndtered;
        DurationTimer.Timeout += QueueFree;

        Rotation = MoveDir.Angle();
    }

    public override void _PhysicsProcess(double delta)
    {
        GlobalPosition += MoveSpeed * MoveDir * (float)delta;
    }

    public void OnBodyEndtered(Node2D body)
    {
        if (body is not Enemy enemy)
        {
            return;
        }

        if (enemy.Hp <= 0)
        {
            return;
        }

        enemy.TakeDamage(Damage);

        CallDeferred(Node.MethodName.QueueFree);
    }
}
