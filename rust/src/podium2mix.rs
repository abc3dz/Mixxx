use godot::prelude::*;
use godot::classes::{Area3D, AudioStreamPlayer3D};

use crate::main_scene::MainScene;

#[derive(GodotClass)]
#[class(init, base=Area3D)]
struct PodiumMix {
    #[export]
    notss_sound: OnEditor<Gd<AudioStreamPlayer3D>>,
    #[export]
    congrats_sound: OnEditor<Gd<AudioStreamPlayer3D>>,
    #[export]
    main_scene: OnEditor<Gd<MainScene>>,
    base: Base<Area3D>,
}
#[godot_api]
impl PodiumMix {
    #[func]
    fn on_notss_body_entered(&mut self, body: Gd<Node3D>) {
        if body.is_in_group("player") && self.main_scene.bind().collected >= 3 {            
            let mut congrats_sound = self.congrats_sound.clone();
            congrats_sound.play();
        } else {
            let mut notss_sound = self.notss_sound.clone();
            notss_sound.play();
        }
    }
}