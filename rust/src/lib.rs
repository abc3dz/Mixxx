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
mod osc_wall_door_hold;
mod osc_flow;
mod osc_platform_up_down;
mod osc_platform_left_right;