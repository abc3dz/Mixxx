use godot::classes::{Area3D, INode3D, Label, Node3D, Texture2D, TextureRect};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node3D)]
pub struct MainScene {
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
    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for MainScene {
    fn ready(&mut self) {
        self.how2.set_text("");
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
        }
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
            self.how2.set_text("Congratulations! You've collected all items and entered the podium mix area!");
        }
        if body.is_in_group("player") && self.collected < 3 {
            self.how2.set_text("You need to collect all items before entering the podium mix area!");
        }
    }
     #[func]
    fn on_podium2mix_body_exited(&mut self, body: Gd<Node3D>) {
        if body.is_in_group("player") {
            self.how2.set_text("");
        }
        
    }
}