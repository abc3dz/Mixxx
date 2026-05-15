use godot::prelude::*;
use godot::classes::{Area3D, IArea3D};

#[derive(GodotClass)]
#[class(init, base=Area3D)]
struct PodiumMix {

    base: Base<Area3D>,
}
#[godot_api]
impl IArea3D for PodiumMix {
}