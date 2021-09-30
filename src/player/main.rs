use gdnative::{api::ProjectSettings, prelude::*};

/// The HelloWorld "class"
#[derive(NativeClass)]
#[inherit(KinematicBody)]
pub struct Player {
    #[property(path = "speed", default = 5.0)]
    speed: f32,
    gravity: f32,
    velocity: Vector3,
}

// You may add any number of ordinary `impl` blocks as you want. However, ...
impl Player {
    /// The "constructor" of the class.
    fn new(_owner: &KinematicBody) -> Self {
        Player {
            speed: 5.0,
            // TODO: Inheritance or something? as this can be common across many types
            gravity: -ProjectSettings::get_setting(
                ProjectSettings::godot_singleton(),
                "physics/3d/default_gravity",
            )
            .to_f64() as f32,
            velocity: Vector3::zero(),
        }
    }
}

// Only __one__ `impl` block can have the `#[methods]` attribute, which
// will generate code to automatically bind any exported methods to Godot.
#[methods]
impl Player {
    // To make a method known to Godot, use the #[export] attribute.
    // In Godot, script "classes" do not actually inherit the parent class.
    // Instead, they are "attached" to the parent object, called the "owner".
    //
    // In order to enable access to the owner, it is passed as the second
    // argument to every single exposed method. As a result, all exposed
    // methods MUST have `owner: &BaseClass` as their second arguments,
    // before all other arguments in the signature.
    #[export]
    fn _ready(&self, _owner: &KinematicBody) {
        // The `godot_print!` macro works like `println!` but prints to the Godot-editor
        // output tab as well.
        godot_print!("{}", self.gravity);
    }

    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody, delta: f32) {
        let grounded = owner.is_on_floor();
        // apply gravity.
        if grounded {
            self.velocity = Vector3::zero();
        } else {
            self.velocity.y += delta * self.gravity;
        }

        // directional move
        if grounded {
            let mut dir = Vector3::zero();

            // Ray cast to detect if movement is block

            dir.x = Input::get_action_strength(Input::godot_singleton(), "ui_right") as f32
                - Input::get_action_strength(Input::godot_singleton(), "ui_left") as f32;
            dir.z = Input::get_action_strength(Input::godot_singleton(), "ui_down") as f32
                - Input::get_action_strength(Input::godot_singleton(), "ui_up") as f32;
            // limit the input to a length of 1. length_squared is faster to check.
            if dir.square_length() > 1.0 {
                dir /= dir.length()
            }

            // apply speed
            dir *= self.speed;

            self.velocity.x = dir.x;
            self.velocity.z = dir.z;
        }

        // Move it
        self.velocity = owner.move_and_slide(
            self.velocity,
            Vector3::new(0.0, 1.0, 0.0),
            true,
            1,
            0.0,
            false,
        );
    }
}
