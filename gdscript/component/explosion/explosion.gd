class_name Explosion
extends Node2D

@export var particles: GPUParticles2D


func _ready() -> void:
	particles.finished.connect(queue_free)
	particles.emitting = true
