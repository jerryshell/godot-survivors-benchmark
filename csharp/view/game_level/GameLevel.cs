using Godot;

public partial class GameLevel : Node
{
    [Export]
    public PackedScene EnemyScene;

    [Export]
    public PackedScene ExplosionScene;

    [Export]
    public Timer EnemySpawnTimer;

    [Export]
    public Node2D ObjectRoot;

    public Player Player;

    public override void _Ready()
    {
        EnemySpawnTimer.Timeout += OnEnemySpawnTimerTimeout;
    }

    public void SpawnEnemy()
    {
        var enemy = EnemyScene.Instantiate<Enemy>();
        enemy.GameLevel = this;
        enemy.GlobalPosition = Player.GlobalPosition + Vector2.Right.Rotated((float)GD.RandRange(0.0, Mathf.Tau)) * 400;
        ObjectRoot.AddChild(enemy);
    }

    public void SpawnExplosion(Vector2 pos)
    {
        var explosion = ExplosionScene.Instantiate<Explosion>();
        explosion.GlobalPosition = pos;
        ObjectRoot.AddChild(explosion);
    }

    public void OnEnemySpawnTimerTimeout()
    {
        SpawnEnemy();
    }
}
