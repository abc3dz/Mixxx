use godot::prelude::*;
use godot::classes::{Area3D, IArea3D};

#[derive(GodotClass)]
#[class(init, base=Area3D)]
struct FloatingItem {
    #[init(val = 0.5)]
    amplitude: f32,
    #[init(val = 2.0)]
    frequency: f32,
    #[init(val = 0.0)]
    time_passed: f32,
    #[init(val = 0.0)]
    initial_y: f32,

    base: Base<Area3D>,
}

#[godot_api]
impl IArea3D for FloatingItem {
    fn ready(&mut self) {
        self.initial_y = self.base().get_position().y;
    }

    fn process(&mut self, delta: f64) {
        self.time_passed += delta as f32;

        let new_y = self.initial_y + (self.time_passed * self.frequency).sin() * self.amplitude;

        let mut pos = self.base().get_position();
        pos.y = new_y;
        
        self.base_mut().set_position(pos);

        self.base_mut().rotate_y(delta as f32 * 1.5);
    }
}
#[godot_api]
impl FloatingItem {
    #[func]
    fn on_body_entered(&mut self, body: Gd<Node3D>) {
        if body.is_in_group("player") {
            self.base_mut().queue_free();
        }
    }
}