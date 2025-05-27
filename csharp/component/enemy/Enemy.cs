using Godot;

public partial class Enemy : CharacterBody2D
{
    [Export]
    public GameLevel GameLevel;

    [Export]
    public Sprite2D Sprite;

    [Export]
    public AnimationPlayer AnimationPlayer;

    [Export]
    public int MoveSpeed = 80;

    [Export]
    public int Hp = 100;

    public override void _PhysicsProcess(double delta)
    {
        if (GameLevel.Player == null)
        {
            return;
        }

        var moveDir = GlobalPosition.DirectionTo(GameLevel.Player.GlobalPosition);

        if (moveDir != Vector2.Zero)
        {
            AnimationPlayer.Play("run");
        }

        Velocity = moveDir * MoveSpeed;

        MoveAndSlide();
    }

    public void TakeDamage(int Damage)
    {
        if (Hp <= 0)
        {
            return;
        }

        Hp -= Damage;
        Flash();

        if (Hp <= 0)
        {
            GameLevel.SpawnExplosion(GlobalPosition);
            CallDeferred(Node.MethodName.QueueFree);
        }
    }

    public void Flash()
    {
        var tween = CreateTween().SetEase(Tween.EaseType.In);
        tween.TweenProperty(this, "modulate", new Color(10, 10, 10), 0.2);
        tween.TweenProperty(this, "modulate", Colors.White, 0.1);
    }
}
