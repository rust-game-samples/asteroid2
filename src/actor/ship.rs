use super::actor::Actor;
use super::component::{Component, ComponentBase};
use super::game::Game;
use super::laser::Laser;
use super::sprite_component::SpriteComponent;
use super::texture::TextureManager;
use super::vector2::Vector2;
use std::f32::consts::PI;

pub struct Ship {
    base: ComponentBase,
    laser_cooldown: f32,
    laser_cooldown_timer: f32,
}

impl Ship {
    pub fn new(texture_manager: &mut TextureManager) -> Self {
        let mut ship = Self {
            base: ComponentBase::new(),
            laser_cooldown: 0.5, // レーザーの発射間隔
            laser_cooldown_timer: 0.0,
        };

        // スプライトコンポーネントを追加
        let sprite = SpriteComponent::new("Ship.png", 100, texture_manager);
        ship.add_component(Box::new(sprite));

        ship
    }

    pub fn shoot_laser(&mut self, game: &mut Game) {
        if self.laser_cooldown_timer <= 0.0 {
            if let Some(actor) = self.owner_mut() {
                let pos = actor.position();
                let rot = actor.rotation();
                game.create_laser(pos, rot);
                self.laser_cooldown_timer = self.laser_cooldown;
            }
        }
    }

    pub fn add_component(&mut self, component: Box<dyn Component>) {
        if let Some(actor) = self.owner_mut() {
            actor.add_component(component);
        }
    }
}

impl Component for Ship {
    fn update(&mut self, delta_time: f32) {
        // クールダウンタイマーを更新
        if self.laser_cooldown_timer > 0.0 {
            self.laser_cooldown_timer -= delta_time;
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
