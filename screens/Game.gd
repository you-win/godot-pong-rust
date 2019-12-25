extends Node2D

func _ready():
	pass

func _process(delta: float) -> void:
	if Input.is_action_just_pressed("ui_select"):
		self.get_tree().reload_current_scene()
