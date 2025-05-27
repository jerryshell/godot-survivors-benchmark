class_name Player
extends CharacterBody2D

@export var game_level: GameLevel
@export var sprite: Sprite2D
@export var animation_player: AnimationPlayer
@export var move_speed := 150


func _ready() -> void:
	game_level.player = self


func _physics_process(delta: float) -> void:
	var move_dir := Input.get_vector("move_left", "move_right", "move_up", "move_down")

	if move_dir != Vector2.ZERO:
		animation_player.play("run")

	velocity = move_dir * move_speed

	move_and_slide()
