use gdnative::{
    nativescript::init::property::{EnumHint, FloatHint, RangeHint, StringHint, Usage},
    object::Ref,
    prelude::{
        godot_print, methods, user_data::MutexData, GodotString, Input, KinematicBody2D, NodePath,
        Sprite, Vector2,
    },
};

const BASE_SPEED: f32 = 200.0;

pub struct Paddle {
    side: String,
    speed: f32,
    target_movement: f32,
    linear_velocity: Vector2,
    sprite: Option<Ref<Sprite, gdnative::thread_access::Unique>>,
}

unsafe impl Send for Paddle {}

impl gdnative::prelude::NativeClass for Paddle {
    type Base = KinematicBody2D;
    type UserData = MutexData<Paddle>;

    fn class_name() -> &'static str {
        "Paddle"
    }

    fn init(owner: &Self::Base) -> Self {
        Self::new(owner)
    }

    fn register_properties(builder: &gdnative::nativescript::init::ClassBuilder<Self>) {
        builder
            .add_property("Side")
            .with_default(GodotString::from_str("none"))
            .with_hint(StringHint::Enum(EnumHint::new(vec![
                "right".into(),
                "left".into(),
                "none".into(),
            ])))
            .with_getter(|this: &Paddle, _owner: &KinematicBody2D| {
                GodotString::from_str(this.side.clone())
            })
            .with_setter(|this: &mut Paddle, _owner: &KinematicBody2D, v| this.side = v.to_string())
            .with_usage(Usage::DEFAULT)
            .done();

        builder
            .add_property("Speed")
            .with_default(BASE_SPEED)
            .with_hint(FloatHint::from(RangeHint::new(0.0, 500.0).with_step(1.0)))
            .with_getter(|this: &Paddle, _owner: &KinematicBody2D| this.speed)
            .with_setter(|this: &mut Paddle, _owner: &KinematicBody2D, v| this.speed = v)
            .with_usage(Usage::DEFAULT)
            .done();
    }
}

#[methods]
impl Paddle {
    fn new(_owner: &KinematicBody2D) -> Self {
        Paddle {
            side: "none".to_string(),
            speed: BASE_SPEED,
            target_movement: 0.0,
            linear_velocity: Vector2::new(0.0, 0.0),
            sprite: None,
        }
    }

    #[export]
    unsafe fn _ready(&mut self, owner: &KinematicBody2D) {
        self.sprite = Some(
            owner
                .get_node(NodePath::from_str("Sprite"))
                .unwrap()
                .assume_unique()
                .cast::<Sprite>()
                .unwrap(),
        );

        godot_print!("Paddle created!");
    }

    #[export]
    unsafe fn _physics_process(&mut self, owner: &KinematicBody2D, _delta: f64) {
        let initial_position: Vector2 = owner.global_position();
        self.target_movement *= self.speed;
        self.linear_velocity.y = self.target_movement;
        self.linear_velocity =
            owner.move_and_slide(self.linear_velocity, Vector2::zero(), false, 4, 0.7, true);
        let new_position: Vector2 = owner.global_position();
        // Godot can't lock objects to a specific axis, so we do that manually
        owner.set_global_position(Vector2::new(initial_position.x, new_position.y));
    }

    #[export]
    unsafe fn _process(&mut self, _owner: &KinematicBody2D, _delta: f64) {
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
                    self.sprite.as_ref().unwrap().set_visible(false);
                } else {
                    self.sprite.as_ref().unwrap().set_visible(true);
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
