use godot::prelude::*;
use godot::classes::{CharacterBody3D};

#[derive(GodotClass)]
#[class(init, base=CharacterBody3D)]
pub struct SceneGDT {
    base: Base<CharacterBody3D>,
}