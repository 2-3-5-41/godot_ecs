extends Node3D

var cam_rid: RID
var viewport_rid: RID
var viewport_texture: RID

func _ready():
	_create_viewport_from_server()
	_create_camera_from_server()

func _create_camera_from_server():
	# Create the camera in the rendering server with `RenderingServer.camera_create()`
	cam_rid = RenderingServer.camera_create()
	# Set camera's perspective using `RenderingServer.camera_set_perspective()`
	RenderingServer.camera_set_perspective(cam_rid, 90.0, 0.1, 4000.0)
	# Set camera to viewport with `RenderingServer.viewport_attach_camera()`
#	var viewport = get_viewport().get_viewport_rid()
	RenderingServer.viewport_attach_camera(viewport_rid, cam_rid)
	RenderingServer.camera_set_transform(cam_rid, Transform3D(Basis.IDENTITY, Vector3(1.0, 1.0, 1.0)))

func _create_viewport_from_server():
	viewport_rid = RenderingServer.viewport_create()
	RenderingServer.viewport_attach_to_screen(viewport_rid, Rect2(Vector2(200,200), Vector2(480, 480)))
	RenderingServer.viewport_set_render_direct_to_screen(viewport_rid, true)
