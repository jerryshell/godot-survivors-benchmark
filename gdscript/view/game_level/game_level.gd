class_name GameLevel
extends Node

@export var enemy_scene: PackedScene
@export var explosion_scene: PackedScene
@export var enemy_spawn_timer: Timer
@export var object_root: Node2D

var player: Player


func _ready() -> void:
	enemy_spawn_timer.timeout.connect(on_enemy_spawn_timer_timeout)


func spawn_enemy() -> void:
	var enemy: Enemy = enemy_scene.instantiate()
	enemy.game_level = self
	enemy.global_position = (
		player.global_position + Vector2.RIGHT.rotated(randf_range(0, TAU)) * 400
	)
	object_root.add_child(enemy)


func spawn_explosion(pos: Vector2) -> void:
	var explosion: Explosion = explosion_scene.instantiate()
	explosion.global_position = pos
	object_root.add_child(explosion)


func on_enemy_spawn_timer_timeout() -> void:
	spawn_enemy()
