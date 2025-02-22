use super::actor::Actor;
use super::component::{Component, ComponentBase};
use super::vector2::Vector2;

pub struct CircleComponent {
    base: ComponentBase,
    radius: f32,
}

impl CircleComponent {
    pub fn new(radius: f32) -> Self {
        Self {
            base: ComponentBase::new(),
            radius,
        }
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn set_radius(&mut self, radius: f32) {
        self.radius = radius;
    }

    pub fn intersect(&mut self, other: &mut CircleComponent) -> bool {
        if let (Some(owner), Some(other_owner)) = (self.owner_mut(), other.owner_mut()) {
            let pos1 = owner.position();
            let pos2 = other_owner.position();
            let diff = pos1 - pos2;
            let dist_sq = diff.length_squared();
            let radii = self.radius + other.radius;
            dist_sq <= radii * radii
        } else {
            false
        }
    }
}

impl Component for CircleComponent {
    fn update(&mut self, _delta_time: f32) {}

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
