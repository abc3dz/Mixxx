use godot::prelude::*;
use godot::classes::{CharacterBody3D};

#[derive(GodotClass)]
#[class(init, base=CharacterBody3D)]
pub struct SceneGDR {
    base: Base<CharacterBody3D>,
}