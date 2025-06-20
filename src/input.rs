//! Quản lý input từ bàn phím.

use std::collections::HashMap;
use dioxus::events::Key;

#[derive(Debug, Clone, Copy, Default)]
pub struct KeyState {
    pub pressed: bool,
    pub just_pressed: bool,
    pub just_released: bool,
}

#[derive(Default)]
pub struct Input {
    pub keys: HashMap<String, KeyState>,
}

impl Input {
    /// Cập nhật trạng thái phím dựa trên KeyboardData truyền vào (không queue event).
    pub fn push(&mut self, data: dioxus::prelude::KeyboardData) {
        let code = data.code(); // code là &str
        let state = self.keys.entry(code.to_string()).or_default();
        if matches!(data.key(), Key::ArrowUp | Key::ArrowDown | Key::ArrowLeft | Key::ArrowRight) {
            if !state.pressed {
                state.just_pressed = true;
            }
            state.pressed = true;
        } else {
            if state.pressed {
                state.just_released = true;
            }
            state.pressed = false;
        }
    }

    /// Reset trạng thái just_pressed/just_released cho tất cả phím (gọi mỗi frame).
    pub fn update(&mut self) {
        for state in self.keys.values_mut() {
            state.just_pressed = false;
            state.just_released = false;
        }
    }

    pub fn is_key_pressed(&self, code: &str) -> bool {
        self.keys.get(code).is_some_and(|s| s.pressed)
    }

    pub fn is_key_just_pressed(&self, code: &str) -> bool {
        self.keys.get(code).is_some_and(|s| s.just_pressed)
    }
}
