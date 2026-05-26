use godot::prelude::*;
use godot::classes::{CharacterBody3D, ICharacterBody3D, AnimationPlayer};

#[derive(GodotClass)]
#[class(init, base=CharacterBody3D)]
pub struct SceneGDT {
    #[export]
    anim_player: OnEditor<Gd<AnimationPlayer>>,
    base: Base<CharacterBody3D>,
}
#[godot_api]
impl ICharacterBody3D for SceneGDT {
    fn ready(&mut self) {
        self.anim_player.play_ex()
            .name("gdt_idle")
            .done();
    }
}