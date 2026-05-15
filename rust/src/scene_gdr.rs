use godot::prelude::*;
use godot::classes::{CharacterBody3D, ICharacterBody3D};

#[derive(GodotClass)]
#[class(init, base=CharacterBody3D)]
pub struct SceneGDR {
    base: Base<CharacterBody3D>,
}
#[godot_api]
impl ICharacterBody3D for SceneGDR {
}
#[godot_api]
impl SceneGDR {
}