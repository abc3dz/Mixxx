use crate::scene_gdr;
use crate::scene_gdt;

use godot::classes::{CharacterBody3D, Area3D, INode3D, Label, Node3D, Texture2D, TextureRect, AnimationTree, Timer};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node3D)]
pub struct MainScene {
    #[export]
    player: OnEditor<Gd<CharacterBody3D>>,
    #[export]
    base_mesh: OnEditor<Gd<Area3D>>,
    #[export]
    godot_toon: OnEditor<Gd<Area3D>>,
    #[export]
    rust_toon: OnEditor<Gd<Area3D>>,
    #[init(node = "HBoxContainer/BaseMeshTR")]
    base_mesh_tr: OnReady<Gd<TextureRect>>,
    #[init(node = "HBoxContainer/GodotToonTR")]
    godot_toon_tr: OnReady<Gd<TextureRect>>,
    #[init(node = "HBoxContainer/RustToonTR")]
    rust_toon_tr: OnReady<Gd<TextureRect>>,
    #[init(val = 0)]
    collected: i32,
    #[export]
    how2: OnEditor<Gd<Label>>,
    #[export]
    scene_gdt: OnEditor<Gd<PackedScene>>,
    #[export]
    scene_gdr: OnEditor<Gd<PackedScene>>,
    
    #[export]
    timer_yeah: OnEditor<Gd<Timer>>,
    #[export]
    fall_area: OnEditor<Gd<Area3D>>,
    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for MainScene {
    fn ready(&mut self) {
        self.how2.set_text("");
        //self.player.set_position(Vector3 { x: -10.0, y: 0.0, z: -21.0 });
    }
}
#[godot_api]
impl MainScene {
    #[func]
    fn on_rust_toon_body_entered(&mut self, body: Gd<Node3D>) {
        if body.is_in_group("player") {
            let texture = load::<Texture2D>("res://Imgs/RustToon100.webp");
            self.rust_toon_tr.set_texture(&texture);
            self.collected += 1;
        }
    }
    #[func]
    fn on_godot_toon_body_entered(&mut self, body: Gd<Node3D>) {
        if body.is_in_group("player") {
            let texture = load::<Texture2D>("res://Imgs/GodotToon100.webp");
            self.godot_toon_tr.set_texture(&texture);
            self.collected += 1;
            let mut player_anim_tree = self.player.get_node_as::<AnimationTree>("AnimationTree");
            player_anim_tree.set(
                "parameters/yeah_blend/blend_amount",
                &Variant::from(5.0_f32),
            );
        }
        self.timer_yeah.start();
    }
    #[func]
    fn on_base_mesh_body_entered(&mut self, body: Gd<Node3D>) {
        if body.is_in_group("player") {
            let texture = load::<Texture2D>("res://Imgs/BaseMesh100.webp");
            self.base_mesh_tr.set_texture(&texture);
            self.collected += 1;
        }
    }
    #[func]
    fn on_podium2mix_body_entered(&mut self, body: Gd<Node3D>) {
        if body.is_in_group("player") && self.collected >= 3 {
            self.how2.set_text("Congratulations!");
            
            let mut sc_gdr = self.scene_gdr.instantiate_as::<scene_gdr::SceneGDR>();
            sc_gdr.set_position(Vector3 { x: 1.0, y: 0.0, z: 0.0 });
            self.base_mut().add_child(&sc_gdr);

            let mut sc_gdt = self.scene_gdt.instantiate_as::<scene_gdt::SceneGDT>();
            sc_gdt.set_position(Vector3 { x: -1.0, y: 0.0, z: 0.0 });
            self.base_mut().add_child(&sc_gdt);

            self.player.set_physics_process(false);
        }
        if body.is_in_group("player") && self.collected < 3 {
            self.how2.set_text("You need to collect 3 items before entering the podium mix area!");
        }
    }
     #[func]
    fn on_podium2mix_body_exited(&mut self, body: Gd<Node3D>) {
        if body.is_in_group("player") {
            self.how2.set_text("");
        }
        
    }
    #[func]
    fn on_timer_timeout(&mut self) {
        let mut player_anim_tree = self.player.get_node_as::<AnimationTree>("AnimationTree");
        player_anim_tree.set(
            "parameters/yeah_blend/blend_amount",
            &Variant::from(0.0_f32),
        );
    }
    #[func]
    fn on_fall_area_body_entered(&mut self, body: Gd<Node3D>) {
        if body.is_in_group("player") {
            self.player.set_position(Vector3 { x: 0.0, y: 1.0, z: 0.0 });
        }
    }
}