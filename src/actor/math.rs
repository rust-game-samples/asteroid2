use crate::actor::Vector2;
use std::f32::consts::{PI, TAU};
/// 数学ユーティリティ関数を提供する構造体
pub struct Math;

impl Math {
    /// 度数からラジアンに変換
    pub fn to_rad(degrees: f32) -> f32 {
        degrees * PI / 180.0
    }

    /// ラジアンから度数に変換
    pub fn to_deg(radians: f32) -> f32 {
        radians * 180.0 / PI
    }

    /// 角度を-πからπの範囲に正規化
    pub fn normalize_angle(angle: f32) -> f32 {
        let mut result = angle;
        while result <= -PI {
            result += TAU;
        }
        while result > PI {
            result -= TAU;
        }
        result
    }

    /// 2つの値の間を線形補間
    pub fn lerp(a: f32, b: f32, f: f32) -> f32 {
        a + f * (b - a)
    }

    /// 値を指定範囲内に制限
    pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
        if value < min {
            min
        } else if value > max {
            max
        } else {
            value
        }
    }

    /// 2つのベクトル間の角度を計算（ラジアン）
    pub fn angle_between(v1: Vector2, v2: Vector2) -> f32 {
        (v2.y.atan2(v2.x) - v1.y.atan2(v1.x)).abs()
    }
}
