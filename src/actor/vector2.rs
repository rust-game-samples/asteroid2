use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// 2次元ベクトルを表す構造体
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    /// 新しいVector2を作成
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// ゼロベクトルを返す
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// (1, 1)のベクトルを返す
    pub fn one() -> Self {
        Self { x: 1.0, y: 1.0 }
    }

    /// ベクトルの長さを計算
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// ベクトルの長さの二乗を計算
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    /// ベクトルを正規化
    pub fn normalize(&mut self) {
        let length = self.length();
        if length > 0.0 {
            self.x /= length;
            self.y /= length;
        }
    }

    /// 正規化されたベクトルを返す
    pub fn normalized(&self) -> Self {
        let mut vec = *self;
        vec.normalize();
        vec
    }

    /// 内積を計算
    pub fn dot(&self, other: &Vector2) -> f32 {
        self.x * other.x + self.y * other.y
    }
}

// 加算の実装
impl Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

// 減算の実装
impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

// スカラー乗算の実装
impl Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

// スカラー除算の実装
impl Div<f32> for Vector2 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl DivAssign<f32> for Vector2 {
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
    }
}
