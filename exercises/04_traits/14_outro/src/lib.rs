// TODO: Define a new `SaturatingU16` type.
// TODO: SaturatingU16 타입을 정의하세요.
// u16 타입의 값을 가집니다.
// u16, u8, &u16, &u8로부터 변환을 제공합니다.
// SaturatingU16, u16, &u16, &SaturatingU16 타입의
// right-hand side와 덧셈을 지원합니다.
// u16의 최댓값에 도달하면 saturate되어야 합니다.
// 다른 SaturatingU16 또는 u16과 비교할 수 있습니다.
// 디버그 표현을 출력할 수 있습니다.
//
//
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::ops::{Add,Deref};
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct SaturatingU16 {
    value: u16,
}

impl SaturatingU16 {

}

// u16, u8, &u16, &u8로부터 변환을 제공
impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        SaturatingU16 { value }
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        SaturatingU16 { value: value as u16 }
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        SaturatingU16 { value: *value }
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        SaturatingU16 { value: *value as u16 }
    }
}

// SaturatingU16, u16, &u16, &SaturatingU16 타입의
// right-hand side와 덧셈을 지원
impl Add for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: Self) -> Self::Output {
        SaturatingU16 { value: self.value.saturating_add(rhs.value) }
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: u16) -> Self::Output {
        SaturatingU16 { value: self.value.saturating_add(rhs) }
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: &u16) -> Self::Output {
        SaturatingU16 { value: self.value.saturating_add(*rhs) }
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: &SaturatingU16) -> Self::Output {
        SaturatingU16 { value: self.value.saturating_add(rhs.value) }
    }
}

// 다른 SaturatingU16 또는 u16과 비교
// impl Deref for SaturatingU16 {
//     type Target = u16;
//     fn deref(&self) -> &Self::Target {
//         &self.value
//     }
// }

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}