class_name Bullet
extends Area2D

@export var duration_timer: Timer

var move_speed := 300
var move_dir := Vector2.RIGHT
var damage := 50


func _ready() -> void:
	body_entered.connect(on_body_entered)
	duration_timer.timeout.connect(queue_free)

	rotation = move_dir.angle()


func _physics_process(delta: float) -> void:
	global_position += move_speed * move_dir * delta


func on_body_entered(body: Node2D) -> void:
	var enemy: Enemy = body

	if enemy.hp <= 0:
		return

	enemy.take_damage(damage)

	queue_free.call_deferred()
