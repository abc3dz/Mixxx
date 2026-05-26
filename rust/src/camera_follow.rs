use godot::prelude::*;
use godot::classes::{Camera3D, ICamera3D, Node3D};
use crate::main_scene::MainScene;
#[derive(GodotClass)]
#[class(init, base=Camera3D)]
pub struct CameraFollow {
    #[export]
    target_path: NodePath,
    #[export]
    smooth: f32,
    #[export]
    offset: Vector3,
    #[export]
    main_scene: OnEditor<Gd<MainScene>>,
    base: Base<Camera3D>,
}

#[godot_api]
impl ICamera3D for CameraFollow {
    fn process(&mut self, delta: f64) {
        
        if self.main_scene.bind().collected_podium {
            self.base_mut().set_position(Vector3 { x: -11.0, y: (3.0), z: (-16.5) });
            return;
        }

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