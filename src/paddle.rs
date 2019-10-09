use godot::{
    init::{Property, PropertyHint, PropertyUsage},
    user_data::MutexData,
    GodotString, Input, KinematicBody2D, NodePath, Sprite, Vector2,
};

use crate::utils::{clamp, lerp};

const BASE_SPEED: f32 = 100.0;
const MAX_VELOCITY: f32 = 500.0;
const MIN_VELOCITY: f32 = -500.0;

pub struct Paddle {
    side: String,
    speed: f32,
    target_movement: f32,
    linear_velocity: Vector2,
    sprite: Sprite,
}

unsafe impl Send for Paddle {}

impl godot::NativeClass for Paddle {
    type Base = KinematicBody2D;
    type UserData = MutexData<Paddle>;

    fn class_name() -> &'static str {
        "Paddle"
    }

    fn init(_owner: Self::Base) -> Self {
        Self::_init()
    }

    fn register_properties(_builder: &godot::init::ClassBuilder<Self>) {
        _builder.add_property(Property {
            name: "Side",
            default: GodotString::from_str("none"),
            hint: PropertyHint::Enum {
                values: &["right", "left", "none"],
            },
            getter: |this: &Paddle| GodotString::from_str(this.side.clone()),
            setter: |this: &mut Paddle, v: GodotString| this.side = v.to_string(),
            usage: PropertyUsage::DEFAULT,
        });

        _builder.add_property(Property {
            name: "Speed",
            default: BASE_SPEED,
            hint: PropertyHint::Range {
                range: 0.0..500.0,
                step: 1.0,
                slider: true,
            },
            getter: |this: &Paddle| this.speed,
            setter: |this: &mut Paddle, v| this.speed = v,
            usage: PropertyUsage::DEFAULT,
        });
    }
}

#[methods]
impl Paddle {
    fn _init() -> Self {
        Paddle {
            side: "none".to_string(),
            speed: BASE_SPEED,
            target_movement: 0.0,
            linear_velocity: Vector2::new(0.0, 0.0),
            sprite: Sprite::new(),
        }
    }

    #[export]
    unsafe fn _ready(&mut self, mut owner: KinematicBody2D) {
        owner.set_physics_process(true);
        // TODO scary cast to KinematicBody2D that probably works
        // self.kb2d = mem::transmute::<Node, godot::KinematicBody2D>(
        //     owner
        //         .get_node(NodePath::from_str("KinematicBody2D"))
        //         .expect("Missing KinematicBody2D"),
        // );

        self.sprite = owner
            .get_node(NodePath::from_str("Sprite"))
            .expect("Missing Sprite node")
            .cast::<Sprite>()
            .expect("Unable to cast to Sprite");

        godot_print!("Paddle created!");
    }

    #[export]
    unsafe fn _physics_process(&mut self, mut owner: KinematicBody2D, delta: f64) {
        self.target_movement *= self.speed;
        self.linear_velocity.y = self.target_movement;
        self.linear_velocity =
            owner.move_and_slide(self.linear_velocity, Vector2::zero(), false, 4, 0.7, true);
    }

    #[export]
    unsafe fn _process(&mut self, mut owner: KinematicBody2D, delta: f64) {
        let input = Input::godot_singleton();
        self.target_movement = 0.0;
        match self.side.as_ref() {
            "left" => {
                if Input::is_action_pressed(&input, GodotString::from_str("paddle_left_up")) {
                    self.target_movement -= 1.0;
                }
                if Input::is_action_pressed(&input, GodotString::from_str("paddle_left_down")) {
                    self.target_movement += 1.0;
                }
                // TODO just to demonstrate that I can manipulate child nodes
                // Remove later, maybe
                if Input::is_action_pressed(&input, GodotString::from_str("ui_accept")) {
                    self.sprite.set_visible(false);
                } else {
                    self.sprite.set_visible(true);
                }
            }
            "right" => {
                if Input::is_action_pressed(&input, GodotString::from_str("paddle_right_up")) {
                    self.target_movement -= 1.0;
                }
                if Input::is_action_pressed(&input, GodotString::from_str("paddle_right_down")) {
                    self.target_movement += 1.0;
                }
            }
            "none" => godot_print!("[ERROR] Paddle side not assigned!"),
            _ => godot_print!("[ERROR] Paddle side is not expected, somehow!"),
        }
    }
}
