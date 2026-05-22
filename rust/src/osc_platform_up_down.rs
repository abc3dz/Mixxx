use godot::prelude::*;
use godot::classes::{Node3D, MeshInstance3D};
use godot::classes::tween::TransitionType;
use godot::classes::tween::EaseType;

#[derive(GodotClass)]
#[class(init, base=Node3D)]
pub struct PlatformUpDown {
    #[export]
    platform: OnEditor<Gd<MeshInstance3D>>,
    #[export] 
    moving_up: bool,
    #[export]
    updown_speed: f32,
    base: Base<Node3D>,
}
#[godot_api]
impl PlatformUpDown {
    #[func]
    fn on_up_body_entered(&mut self, body: Gd<Node3D>) {
        if body.is_in_group("player") {
            let platform = self.platform.clone();

            let mut tween = self.base_mut().create_tween();

            tween.tween_property(
                    &platform,
                    "position:y",
                    &1.25.to_variant(),
                    2.0,
                )
                .set_trans(TransitionType::SINE)
                .set_ease(EaseType::IN_OUT);
        }
    }
    #[func]
    fn on_down_body_entered(&mut self, body: Gd<Node3D>) {
        if body.is_in_group("player") {
            let platform = self.platform.clone();

            let mut tween = self.base_mut().create_tween();

            tween.tween_property(
                    &platform,
                    "position:y",
                    &0.0.to_variant(),
                    2.0,
                )
                .set_trans(TransitionType::SINE)
                .set_ease(EaseType::IN_OUT);
        }
    }
}