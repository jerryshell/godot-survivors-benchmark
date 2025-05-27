class_name ControlPanel
extends CanvasLayer

@export var game_level: GameLevel
@export var fps_label: Label
@export var enemy_count_label: Label
@export var spawn_enemy_button: Button
@export var spawn_enemy_x10_button: Button
@export var spawn_enemy_x100_button: Button


func _ready() -> void:
	spawn_enemy_button.pressed.connect(spawn_enemy.bind(1))
	spawn_enemy_x10_button.pressed.connect(spawn_enemy.bind(10))
	spawn_enemy_x100_button.pressed.connect(spawn_enemy.bind(100))


func _process(delta: float) -> void:
	fps_label.text = "FPS: %s" % Engine.get_frames_per_second()
	enemy_count_label.text = "Enemy Count: %s" % get_tree().get_node_count_in_group("enemy")


func spawn_enemy(count: int = 1) -> void:
	for i in count:
		game_level.spawn_enemy()
