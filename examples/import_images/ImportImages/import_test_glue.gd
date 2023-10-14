extends Control

@export var import_gdext: ImportTest

# Called when the node enters the scene tree for the first time.
func _ready():
	if import_gdext:
		get_viewport().connect("files_dropped", import_gdext.on_files_dropped)
	else:
		push_error("`ImportTest` node not found!")
