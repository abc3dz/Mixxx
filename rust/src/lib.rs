use godot::prelude::*;

struct SquashTheCreeps;

#[gdextension]
unsafe impl ExtensionLibrary for SquashTheCreeps {}

mod main_scene;
mod player;
mod camera_follow;
mod floating;
mod podium2mix;
mod scene_gdr;
mod scene_gdt;