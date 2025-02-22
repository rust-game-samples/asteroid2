use super::component::Component;
use super::input_component::InputComponent;
use super::vector2::Vector2;

/// アクターの基本構造体
pub struct Actor {
    /// アクターの位置
    position: Vector2,
    /// アクターの回転角度（ラジアン）
    rotation: f32,
    /// アクターのスケール
    scale: Vector2,
    /// アクターの状態（アクティブかどうか）
    active: bool,
    /// アクターに付属するコンポーネントのリスト
    components: Vec<Box<dyn Component>>,
}

impl Actor {
    /// 新しいアクターを作成
    pub fn new() -> Self {
        Self {
            position: Vector2::zero(),
            rotation: 0.0,
            scale: Vector2::one(),
            active: true,
            components: Vec::new(),
        }
    }

    /// アクターを更新
    pub fn update(&mut self, delta_time: f32) {
        if !self.active {
            return;
        }

        // 全てのコンポーネントを更新
        for component in &mut self.components {
            component.update(delta_time);
        }
    }

    /// コンポーネントを追加
    pub fn add_component(&mut self, component: Box<dyn Component>) {
        self.components.push(component);
    }

    // Getters and Setters
    pub fn position(&mut self) -> Vector2 {
        self.position
    }

    pub fn set_position(&mut self, pos: Vector2) {
        self.position = pos;
    }

    pub fn rotation(&mut self) -> f32 {
        self.rotation
    }

    pub fn set_rotation(&mut self, rot: f32) {
        self.rotation = rot;
    }

    pub fn scale(&self) -> Vector2 {
        self.scale
    }

    pub fn set_scale(&mut self, scale: Vector2) {
        self.scale = scale;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn get_component<T: 'static>(&self) -> Option<&T> {
        for component in self.components.iter() {
            if let Some(c) = component.as_ref().as_any().downcast_ref::<T>() {
                return Some(c);
            }
        }
        None
    }

    pub fn get_component_mut<T: 'static>(&mut self) -> Option<&mut T> {
        for component in self.components.iter_mut() {
            if let Some(c) = component.as_mut().as_any_mut().downcast_mut::<T>() {
                return Some(c);
            }
        }
        None
    }
}
