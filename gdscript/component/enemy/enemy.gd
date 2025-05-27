class_name Enemy
extends CharacterBody2D

@export var game_level: GameLevel
@export var sprite: Sprite2D
@export var animation_player: AnimationPlayer
@export var move_speed := 80
@export var hp := 100


func _physics_process(delta: float) -> void:
	if not game_level.player:
		return

	var move_dir := global_position.direction_to(game_level.player.global_position)

	if move_dir != Vector2.ZERO:
		animation_player.play("run")

	velocity = move_dir * move_speed

	move_and_slide()


func take_damage(damage: int) -> void:
	if hp <= 0:
		return

	hp -= damage
	flash()

	if hp <= 0:
		game_level.spawn_explosion.call_deferred(global_position)
		queue_free.call_deferred()


func flash() -> void:
	var tween := create_tween().set_ease(Tween.EASE_IN)
	tween.tween_property(self, "modulate", Color(10, 10, 10), 0.2)
	tween.tween_property(self, "modulate", Color.WHITE, 0.1)
