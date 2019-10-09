use godot::{
    init::{Property, PropertyHint, PropertyUsage},
    user_data::MutexData,
    GodotString, KinematicBody2D, KinematicCollision2D, Node2D, NodePath, Sprite, Vector2,
};

use rand::{
    distributions::{Distribution, Uniform},
    Rng,
};

use crate::utils::clamp;

const BASE_SPEED: f32 = 5.0;
const MAX_VELOCITY_X: f32 = 10.0;
const MIN_VELOCITY_X: f32 = -10.0;
const MAX_VELOCITY_Y: f32 = 10.0;
const MIN_VELOCITY_Y: f32 = -10.0;

pub struct Ball {
    speed: f32,
    target_movement: Vector2,
    linear_velocity: Vector2,
    sprite: Sprite,
    rng: rand::prelude::ThreadRng,
    coin: Uniform<i32>,
}

unsafe impl Send for Ball {}

impl godot::NativeClass for Ball {
    type Base = KinematicBody2D;
    type UserData = MutexData<Ball>;

    fn class_name() -> &'static str {
        "Ball"
    }

    fn init(_owner: Self::Base) -> Self {
        Self::_init()
    }

    fn register_properties(_builder: &godot::init::ClassBuilder<Self>) {
        _builder.add_property(Property {
            name: "Speed",
            default: BASE_SPEED,
            hint: PropertyHint::Range {
                range: 0.0..500.0,
                step: 1.0,
                slider: true,
            },
            getter: |this: &Ball| this.speed,
            setter: |this: &mut Ball, v| this.speed = v,
            usage: PropertyUsage::DEFAULT,
        });
    }
}

#[methods]
impl Ball {
    fn _init() -> Self {
        Ball {
            speed: BASE_SPEED,
            target_movement: Vector2::zero(),
            linear_velocity: Vector2::zero(),
            sprite: Sprite::new(),
            rng: rand::thread_rng(),
            coin: Uniform::from(0..2),
        }
    }

    #[export]
    unsafe fn _ready(&mut self, mut owner: KinematicBody2D) {
        owner.set_physics_process(true);

        self.sprite = owner
            .get_node(NodePath::from_str("Sprite"))
            .expect("Missing Sprite node")
            .cast::<Sprite>()
            .expect("Unable to cast to Sprite");

        match self.coin.sample(&mut self.rng) {
            0 => self.target_movement.x = -1.0,
            1 => self.target_movement.x = 1.0,
            _ => godot_print!("[ERROR] Did we flip the coin onto its side?"),
        }

        godot_print!("Ball created!");
    }

    #[export]
    unsafe fn _physics_process(&mut self, mut owner: KinematicBody2D, delta: f64) {
        let actual_movement: Vector2 = Vector2::new(
            self.target_movement.x * self.speed,
            self.target_movement.y * self.speed,
        );
        // self.target_movement *= self.speed;
        self.linear_velocity = actual_movement;
        match owner.move_and_collide(self.linear_velocity, true, true, false) {
            Some(collision_data) => {
                let collider = collision_data
                    .get_collider()
                    .expect("[ERROR] No collider in collision data")
                    .cast::<Node2D>()
                    .expect("[ERROR] Collided with something that is not a Node2D");
                if collider.is_in_group(GodotString::from_str("Paddles")) {
                    let random_x = self.rng.gen_range(-1.0, 1.0);
                    let random_y = self.rng.gen_range(-1.0, 1.0);
                    self.target_movement.x = -self.target_movement.x;
                    self.target_movement.y = -self.target_movement.y;
                    match self.coin.sample(&mut self.rng) {
                        0 => {
                            self.target_movement.x = clamp(
                                self.target_movement.x + random_x,
                                MIN_VELOCITY_X,
                                MAX_VELOCITY_X,
                            );
                        }
                        1 => {
                            self.target_movement.y = clamp(
                                self.target_movement.y + random_y,
                                MIN_VELOCITY_Y,
                                MAX_VELOCITY_Y,
                            );
                        }
                        _ => godot_print!("[ERROR] How did we mess up a coin flip?"),
                    }
                }
                if collider.is_in_group(GodotString::from_str("Walls")) {
                    self.target_movement.y = -self.target_movement.y;
                }
            }
            _ => (),
        }
    }
}
