using Godot;

public partial class ControlPanel : CanvasLayer
{
    [Export]
    public GameLevel GameLevel;

    [Export]
    public Label FpsLabel;

    [Export]
    public Label EnemyCountLabel;

    [Export]
    public Button SpawnEnemyButton;

    [Export]
    public Button SpawnEnemyX10Button;

    [Export]
    public Button SpawnEnemyX100Button;

    public override void _Ready()
    {
        SpawnEnemyButton.Pressed += () => SpawnEnemy(1);
        SpawnEnemyX10Button.Pressed += () => SpawnEnemy(10);
        SpawnEnemyX100Button.Pressed += () => SpawnEnemy(100);
    }

    public override void _Process(double delta)
    {
        FpsLabel.Text = $"FPS: {Engine.GetFramesPerSecond()}";
        EnemyCountLabel.Text = $"Enemy Count: {GetTree().GetNodeCountInGroup("enemy")}";
    }

    public void SpawnEnemy(int count = 1)
    {
        for (int i = 0; i < count; i++)
        {
            GameLevel.SpawnEnemy();
        }
    }
}
