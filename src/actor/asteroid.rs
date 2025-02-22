use super::actor::Actor;
use super::component::{Component, ComponentBase};
use super::game::Game;
use super::sprite_component::SpriteComponent;
use super::vector2::Vector2;
use std::f32::consts::PI;

pub struct Asteroid {
    base: ComponentBase,
    rotation_speed: f32,
}

impl Asteroid {
    pub fn new(game: &mut Game) -> Self {
        let mut asteroid = Self {
            base: ComponentBase::new(),
            rotation_speed: rand::random::<f32>() * PI - PI / 2.0,
        };

        // スプライトコンポーネントを追加
        let sprite = SpriteComponent::new("Asteroid.png", 100, &mut game.texture_manager);
        asteroid.add_component(Box::new(sprite));

        asteroid
    }

    pub fn add_component(&mut self, component: Box<dyn Component>) {
        if let Some(actor) = self.owner_mut() {
            actor.add_component(component);
        }
    }
}

impl Component for Asteroid {
    fn update(&mut self, delta_time: f32) {
        let rotation_speed = self.rotation_speed; // 先に値を取得
        if let Some(actor) = self.owner_mut() {
            let rotation = actor.rotation();
            actor.set_rotation(rotation + rotation_speed * delta_time);

            let forward = Vector2::new(rotation.cos(), rotation.sin());
            let pos = actor.position();
            actor.set_position(pos + forward * 150.0 * delta_time);
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
}
