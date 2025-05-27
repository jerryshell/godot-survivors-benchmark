using Godot;

public partial class Player : CharacterBody2D
{
    [Export]
    public GameLevel GameLevel;

    [Export]
    public Sprite2D Sprite;

    [Export]
    public AnimationPlayer AnimationPlayer;

    [Export]
    public int MoveSpeed = 150;

    public override void _Ready()
    {
        GameLevel.Player = this;
    }

    public override void _PhysicsProcess(double delta)
    {
        var moveDir = Input.GetVector("move_left", "move_right", "move_up", "move_down");

        if (moveDir != Vector2.Zero)
        {
            AnimationPlayer.Play("run");
        }

        Velocity = moveDir * MoveSpeed;

        MoveAndSlide();
    }
}
