use super::actor::Actor;
use std::any::Any;

/// コンポーネントの基本トレイト
pub trait Component: Any {
    /// コンポーネントの更新処理
    fn update(&mut self, delta_time: f32);

    /// コンポーネントの初期化処理
    fn start(&mut self) {}

    /// 所有者のアクターを設定
    fn set_owner(&mut self, owner: &mut Actor);

    /// 所有者のアクターを取得
    fn owner(&self) -> Option<&Actor>;

    /// 所有者のアクターを可変参照で取得
    fn owner_mut(&mut self) -> Option<&mut Actor>;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// コンポーネントの基本実装のためのベース構造体
pub struct ComponentBase {
    owner: Option<*mut Actor>,
}

impl ComponentBase {
    pub fn new() -> Self {
        Self { owner: None }
    }

    pub fn set_owner(&mut self, owner: &mut Actor) {
        self.owner = Some(owner as *mut Actor);
    }

    pub fn owner(&self) -> Option<&Actor> {
        unsafe { self.owner.map(|ptr| &*ptr) }
    }

    pub fn owner_mut(&mut self) -> Option<&mut Actor> {
        unsafe { self.owner.map(|ptr| &mut *ptr) }
    }
}
