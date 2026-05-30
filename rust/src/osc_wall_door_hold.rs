use godot::prelude::*;
use godot::classes::{Node3D, CsgMesh3D, StaticBody3D, AudioStreamPlayer3D};

#[derive(GodotClass)]
#[class(init, base=Node3D)]
pub struct WallDoorHold {
    #[export]
    door_open: OnEditor<Gd<CsgMesh3D>>,
    #[export]
    static_door: OnEditor<Gd<StaticBody3D>>,
    #[export]
    on_sound: OnEditor<Gd<AudioStreamPlayer3D>>,
    base: Base<Node3D>,
}

#[godot_api]
impl WallDoorHold {
    #[func]
    fn body_entered(&self, body: Gd<Node3D>) {
        if body.is_in_group("player") {
            let mut door_open = self.door_open.clone();
            let mut static_door = self.static_door.clone();
            if door_open.get_position().x < 0.0 {
                door_open.set_position(Vector3::new(1.0, 0.0, 0.0));
                static_door.set_position(Vector3::new(-2.0, 0.0, 0.0));
            } else {
                door_open.set_position(Vector3::new(-1.0, 0.0, 0.0));
                static_door.set_position(Vector3::new(2.0, 0.0, 0.0));
            }
            let mut on_sound = self.on_sound.clone();
            on_sound.play();
        }
    }
}