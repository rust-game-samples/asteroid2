use super::vector2::Vector2;
use rand::{thread_rng, Rng};
use std::f32::consts::PI;

/// ランダム値生成ユーティリティ
pub struct Random;

impl Random {
    /// 指定範囲の浮動小数点数をランダムに生成
    pub fn float_range(min: f32, max: f32) -> f32 {
        thread_rng().gen_range(min..=max)
    }

    /// 指定範囲の整数をランダムに生成
    pub fn int_range(min: i32, max: i32) -> i32 {
        thread_rng().gen_range(min..=max)
    }

    /// -1.0から1.0の範囲でランダムな値を生成
    pub fn float() -> f32 {
        Self::float_range(-1.0, 1.0)
    }

    /// ランダムな方向のベクトルを生成
    pub fn vector() -> Vector2 {
        let angle = Self::float_range(0.0, PI * 2.0);
        Vector2::new(angle.cos(), angle.sin())
    }

    /// 指定された長さのランダムな方向のベクトルを生成
    pub fn vector_with_length(length: f32) -> Vector2 {
        let v = Self::vector();
        v * length
    }

    /// 指定された範囲内のランダムなベクトルを生成
    pub fn vector_in_rect(min_x: f32, max_x: f32, min_y: f32, max_y: f32) -> Vector2 {
        Vector2::new(
            Self::float_range(min_x, max_x),
            Self::float_range(min_y, max_y),
        )
    }
}
