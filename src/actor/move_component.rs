use super::actor::Actor;
use super::component::{Component, ComponentBase};
use super::vector2::Vector2;

pub struct MoveComponent {
    base: ComponentBase,
    angular_speed: f32,
    forward_speed: f32,
}

impl MoveComponent {
    pub fn new(angular_speed: f32, forward_speed: f32) -> Self {
        Self {
            base: ComponentBase::new(),
            angular_speed,
            forward_speed,
        }
    }

    pub fn forward_speed(&self) -> f32 {
        self.forward_speed
    }

    pub fn set_forward_speed(&mut self, speed: f32) {
        self.forward_speed = speed;
    }

    pub fn angular_speed(&self) -> f32 {
        self.angular_speed
    }

    pub fn set_angular_speed(&mut self, speed: f32) {
        self.angular_speed = speed;
    }
}

impl Component for MoveComponent {
    fn update(&mut self, delta_time: f32) {
        let (angular_speed, forward_speed) = (self.angular_speed, self.forward_speed);
        if let Some(actor) = self.owner_mut() {
            let rot = actor.rotation();
            actor.set_rotation(rot + angular_speed * delta_time);

            let forward = Vector2::new(rot.cos(), rot.sin());
            let pos = actor.position();
            actor.set_position(pos + forward * forward_speed * delta_time);
        }
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
