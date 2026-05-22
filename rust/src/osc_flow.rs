use godot::prelude::*;
use godot::classes::{MeshInstance3D, Node3D};

#[derive(GodotClass)]
#[class(init, base=Node3D)]
pub struct Flow {
    #[export]
    platform: OnEditor<Gd<MeshInstance3D>>,
    #[export] 
    moving_up: bool,
    #[export] 
    updown_speed: f32,
    base: Base<Node3D>,
}
#[godot_api]
impl INode3D for Flow{
    fn process(&mut self, delta: f64) {
        let mut platform = self.platform.clone();
        let current_pos = platform.get_position();
        let mut new_y = current_pos.y;

        if self.moving_up {
            new_y += self.updown_speed * delta as f32;
            if new_y >= 1.3 {
                new_y = 1.3;
                self.moving_up = false; 
            }
        } else {
            new_y -= self.updown_speed * delta as f32;
            if new_y <= 0.0 {
                new_y = 0.0;
                self.moving_up = true;
            }
        }

        platform.set_position(Vector3::new(
            current_pos.x,
            new_y,
            current_pos.z,
        ));
    }

}
