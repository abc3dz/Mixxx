use godot::prelude::*;
use godot::classes::{MeshInstance3D, Node3D, Timer};

#[derive(GodotClass)]
#[class(init, base=Node3D)]
pub struct Flow {
    #[export]
    platform: OnEditor<Gd<MeshInstance3D>>,
    #[export]
    timer: OnEditor<Gd<Timer>>,
    base: Base<Node3D>,
}
#[godot_api]
impl Flow{
    #[func]
    fn on_body_entered(&mut self, body: Gd<Node3D>) {
        if body.is_in_group("player") {
            let mut platform = self.platform.clone();
            platform.set_scale(Vector3::new(1.0, 1.0, 1.0));
        }
        self.timer.clone().start();
    }       
    #[func]
    fn on_timer_timeout(&mut self) {
        let mut platform = self.platform.clone();
        platform.set_scale(Vector3::new(0.3, 0.3, 0.3));
    }
}
