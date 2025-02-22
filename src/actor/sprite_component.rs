use super::actor::Actor;
use super::component::{Component, ComponentBase};
use super::texture::Texture;
use super::texture::TextureManager;
use wgpu;

pub struct SpriteComponent {
    base: ComponentBase,
    texture_height: i32,
    texture_width: i32,
    draw_order: i32,
    texture_name: String,
}

impl SpriteComponent {
    pub fn new(texture_name: &str, draw_order: i32, texture_manager: &mut TextureManager) -> Self {
        texture_manager.load_texture(texture_name); // テクスチャをロードのみ
        Self {
            base: ComponentBase::new(),
            texture_height: 0,
            texture_width: 0,
            draw_order,
            texture_name: texture_name.to_string(),
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
    pub fn draw(&self, render_pass: &mut wgpu::RenderPass) {
        if let Some(actor) = self.owner() {
            // TODO: 実際の描画処理を実装
            // スプライトの位置、回転、スケールを使用して描画
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
}
