use super::actor::Actor;
use super::component::{Component, ComponentBase};
use super::game::Game;
use super::sprite_component::SpriteComponent;
use super::texture::TextureManager;
use super::vector2::Vector2;
use std::f32::consts::PI;

pub struct Laser {
    base: ComponentBase,
    death_timer: f32,
    forward_speed: f32,
}

impl Laser {
    pub fn new(texture_manager: &mut TextureManager) -> Self {
        let mut laser = Self {
            base: ComponentBase::new(),
            death_timer: 1.0,     // レーザーの生存時間
            forward_speed: 800.0, // レーザーの速度
        };

        // スプライトコンポーネントを追加
        let sprite = SpriteComponent::new("Laser.png", 100, texture_manager);
        laser.add_component(Box::new(sprite));

        laser
    }

    pub fn add_component(&mut self, component: Box<dyn Component>) {
        if let Some(actor) = self.owner_mut() {
            actor.add_component(component);
        }
    }
}

impl Component for Laser {
    fn update(&mut self, delta_time: f32) {
        let forward_speed = self.forward_speed;
        let should_deactivate = {
            let death_timer = self.death_timer;
            self.death_timer = death_timer - delta_time;
            self.death_timer <= 0.0
        };

        if let Some(actor) = self.owner_mut() {
            let rotation = actor.rotation();
            let forward = Vector2::new(rotation.cos(), rotation.sin());
            let pos = actor.position();
            actor.set_position(pos + forward * forward_speed * delta_time);

            if should_deactivate {
                actor.set_active(false);
            }
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
