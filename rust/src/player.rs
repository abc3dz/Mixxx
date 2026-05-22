use godot::classes::{CharacterBody3D, ICharacterBody3D, Input, AnimationTree};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=CharacterBody3D)]
pub struct Player {
    #[init(val = 5.0)]
    speed: f32,

    #[init(val = 6.0)]
    jump_impulse: f32,

    #[init(val = 9.8)]
    gravity: f32, 

    base: Base<CharacterBody3D>,
}

#[godot_api]
impl ICharacterBody3D for Player {
    fn physics_process(&mut self, delta: f64) {
        let speed = self.speed;
        let jump_impulse = self.jump_impulse;
        let gravity = self.gravity;
        let mut anim_tree = self.base().get_node_as::<AnimationTree>("AnimationTree");

        let mut base = self.base_mut();

        let mut velocity = base.get_velocity();

        // Gravity
        if !base.is_on_floor() {
            velocity.y -= gravity * delta as f32;
        }

        // Jump
        if Input::singleton().is_action_just_pressed("ui_select")
            && base.is_on_floor()
        {
            velocity.y = jump_impulse;
            anim_tree.set(
                "parameters/jump_os/request",
                &Variant::from(1.0_f32),
            );
        }

        // Movement input
        let input = Input::singleton().get_vector(
            "ui_left",
            "ui_right",
            "ui_up",
            "ui_down",
        );

        let direction = Vector3::new(input.x, 0.0, input.y);

        if direction.length() > 0.1 {
            velocity.x = direction.x * speed;
            velocity.z = direction.z * speed;

            let angle = f32::atan2(direction.x, direction.z);
            base.set_rotation(Vector3::new(0.0, angle, 0.0));

            anim_tree.set(
                "parameters/idle_walk/blend_amount",
                &Variant::from(1.0_f32),
            );
        } else {
            velocity.x = 0.0;
            velocity.z = 0.0;
            
            anim_tree.set(
                "parameters/idle_walk/blend_amount",
                &Variant::from(0.0_f32),
            );
        }

        base.set_velocity(velocity);

        base.move_and_slide();
    }
}