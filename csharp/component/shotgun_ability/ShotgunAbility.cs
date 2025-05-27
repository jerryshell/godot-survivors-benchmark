using Godot;

public partial class ShotgunAbility : Node
{
    [Export]
    public GameLevel GameLevel;

    [Export]
    public PackedScene BulletScene;

    [Export]
    public Timer AttackTimer;

    public override void _Ready()
    {
        AttackTimer.Timeout += Attack;
    }

    public void Attack()
    {
        var enemy = FindNearestEnemy();
        if (enemy == null)
        {
            return;
        }

        SpawnBullet(enemy, 0);
        SpawnBullet(enemy, -Mathf.Pi / 9);
        SpawnBullet(enemy, Mathf.Pi / 9);
    }

    public void SpawnBullet(Enemy target, float rotationOffset)
    {
        var bullet = BulletScene.Instantiate<Bullet>();
        bullet.GlobalPosition = GameLevel.Player.GlobalPosition;
        bullet.MoveDir = bullet.GlobalPosition.DirectionTo(target.GlobalPosition).Rotated(rotationOffset);
        GameLevel.ObjectRoot.AddChild(bullet);
    }

    public Enemy FindNearestEnemy()
    {
        Enemy enemy = null;
        float enemyDistanceSquared = float.PositiveInfinity;

        var enemyList = GetTree().GetNodesInGroup("enemy");

        foreach (Enemy currentEnemy in enemyList)
        {
            float distanceSquared = currentEnemy.GlobalPosition.DistanceSquaredTo(GameLevel.Player.GlobalPosition);
            if (distanceSquared < enemyDistanceSquared)
            {
                enemy = currentEnemy;
                enemyDistanceSquared = distanceSquared;
            }
        }

        return enemy;
    }
}
