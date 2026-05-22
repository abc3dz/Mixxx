use godot::prelude::*;
use godot::classes::{Camera3D, ICamera3D, Node3D};

#[derive(GodotClass)]
#[class(base=Camera3D)]
pub struct CameraFollow {
    #[export]
    target_path: NodePath,
    #[export]
    smooth: f32,
    offset: Vector3,
    base: Base<Camera3D>,
}

#[godot_api]
impl ICamera3D for CameraFollow {
    fn init(base: Base<Camera3D>) -> Self {
        Self {
            target_path: NodePath::default(),
            smooth: 5.0,
            offset: Vector3::new(0.0, 6.0, 5.0),
            base,
        }
    }

    fn process(&mut self, delta: f64) {
        let path = self.target_path.clone();
        let target = self.base().get_node_or_null(&path);  // ✅ ใช้ &path

        if let Some(node) = target {
            let target_pos = node.cast::<Node3D>().get_global_position();
            let current = self.base().get_global_position();
            let desired = target_pos + self.offset;
            let lerped = current.lerp(desired, (self.smooth * delta as f32).min(1.0));
            self.base_mut().set_global_position(lerped);
            self.base_mut().look_at(target_pos);
        }
    }
}