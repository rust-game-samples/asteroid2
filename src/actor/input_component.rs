use super::actor::Actor;
use super::component::{Component, ComponentBase};
use super::move_component::MoveComponent;
use winit::event::VirtualKeyCode;

pub struct InputComponent {
    base: ComponentBase,
    max_forward_speed: f32,
    max_angular_speed: f32,
    move_component: Option<*mut MoveComponent>,
}

impl InputComponent {
    pub fn new(max_forward_speed: f32, max_angular_speed: f32) -> Self {
        Self {
            base: ComponentBase::new(),
            max_forward_speed,
            max_angular_speed,
            move_component: None,
        }
    }

    pub fn set_move_component(&mut self, move_comp: &mut MoveComponent) {
        self.move_component = Some(move_comp as *mut MoveComponent);
    }

    pub fn process_input(&mut self, keys: &[VirtualKeyCode]) {
        println!("InputComponent processing keys: {:?}", keys);
        if let Some(move_ptr) = self.move_component {
            let move_comp = unsafe { &mut *move_ptr };

            // 前進/後退の処理
            if keys.contains(&VirtualKeyCode::W) {
                println!("W key pressed - moving forward");
                move_comp.set_forward_speed(self.max_forward_speed);
            } else if keys.contains(&VirtualKeyCode::S) {
                println!("S key pressed - moving backward");
                move_comp.set_forward_speed(-self.max_forward_speed);
            } else {
                move_comp.set_forward_speed(0.0);
            }

            // 回転の処理
            if keys.contains(&VirtualKeyCode::D) {
                println!("D key pressed - rotating right");
                move_comp.set_angular_speed(self.max_angular_speed);
            } else if keys.contains(&VirtualKeyCode::A) {
                println!("A key pressed - rotating left");
                move_comp.set_angular_speed(-self.max_angular_speed);
            } else {
                move_comp.set_angular_speed(0.0);
            }
        }
    }
}

impl Component for InputComponent {
    fn update(&mut self, _delta_time: f32) {
        // キー入力の処理は別途実装が必要
    }

    fn set_owner(&mut self, owner: &mut Actor) {
        self.base.set_owner(owner);
    }

    fn owner(&self) -> Option<&Actor> {
        self.base.owner()
    }

    fn owner_mut(&mut self) -> Option<&mut Actor> {
        self.base.owner_mut()
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
