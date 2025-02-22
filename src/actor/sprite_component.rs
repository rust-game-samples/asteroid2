use super::actor::Actor;
use super::component::{Component, ComponentBase};
use super::texture::Texture;
use super::texture::TextureManager;
use std::sync::Arc;
use wgpu;

pub struct SpriteComponent {
    base: ComponentBase,
    texture_height: i32,
    texture_width: i32,
    draw_order: i32,
    texture_name: String,
    texture: Option<Arc<Texture>>,
}

impl SpriteComponent {
    pub fn new(texture_name: &str, draw_order: i32, texture_manager: &mut TextureManager) -> Self {
        texture_manager.load_texture(texture_name);
        let texture = texture_manager.get_texture(texture_name);

        Self {
            base: ComponentBase::new(),
            texture_height: 0,
            texture_width: 0,
            draw_order,
            texture_name: texture_name.to_string(),
            texture,
        }
    }

    pub fn texture_height(&self) -> i32 {
        self.texture_height
    }

    pub fn texture_width(&self) -> i32 {
        self.texture_width
    }

    pub fn draw_order(&self) -> i32 {
        self.draw_order
    }

    // テクスチャの設定メソッド
    // 注: 実際のテクスチャ管理は別途実装が必要
    pub fn set_texture(&mut self, width: i32, height: i32) {
        self.texture_width = width;
        self.texture_height = height;
    }

    // 描画メソッド
    // 注: 実際の描画処理は別途実装が必要
    pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        if let (Some(actor), Some(texture)) = (self.owner(), &self.texture) {
            render_pass.set_bind_group(0, &texture.bind_group, &[]);
            render_pass.draw(0..6, 0..1); // 6頂点を描画
        }
    }
}

impl Component for SpriteComponent {
    fn update(&mut self, _delta_time: f32) {
        // スプライトの更新処理が必要な場合はここに実装
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
