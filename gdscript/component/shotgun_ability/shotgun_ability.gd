class_name ShotgunAbility
extends Node

@export var game_level: GameLevel
@export var bullet_scene: PackedScene
@export var attack_timer: Timer


func _ready() -> void:
	attack_timer.timeout.connect(attack)


func attack() -> void:
	var enemy: Enemy = find_nearest_enemy()
	if not enemy:
		return

	spawn_bullet(enemy, 0)
	spawn_bullet(enemy, -PI / 9)
	spawn_bullet(enemy, PI / 9)


func spawn_bullet(target: Node2D, rotationOffset: float) -> void:
	var bullet: Bullet = bullet_scene.instantiate()
	bullet.global_position = game_level.player.global_position
	bullet.move_dir = bullet.global_position.direction_to(target.global_position).rotated(
		rotationOffset
	)
	game_level.object_root.add_child(bullet)


func find_nearest_enemy() -> Enemy:
	var enemy: Enemy = null
	var enemy_distance_squared: float = INF

	var enemy_list := get_tree().get_nodes_in_group("enemy")
	for current_enemy: Enemy in enemy_list:
		var distance_squared := current_enemy.global_position.distance_squared_to(
			game_level.player.global_position
		)
		if distance_squared < enemy_distance_squared:
			enemy = current_enemy
			enemy_distance_squared = distance_squared

	return enemy
