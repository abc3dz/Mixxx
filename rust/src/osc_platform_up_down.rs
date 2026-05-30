use godot::prelude::*;
use godot::classes::{Node3D, MeshInstance3D, AudioStreamPlayer3D};
use godot::classes::tween::TransitionType;
use godot::classes::tween::EaseType;

#[derive(GodotClass)]
#[class(init, base=Node3D)]
pub struct PlatformUpDown {
    #[export]
    platform: OnEditor<Gd<MeshInstance3D>>,
    #[export]
    up_sound: OnEditor<Gd<AudioStreamPlayer3D>>,
    #[export]
    down_sound: OnEditor<Gd<AudioStreamPlayer3D>>,
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
            let mut up_sound = self.up_sound.clone();
            up_sound.play();
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
            let mut down_sound = self.down_sound.clone();
            down_sound.play();
        }
    }
}