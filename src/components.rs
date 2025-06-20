//! Module này chứa tất cả các định nghĩa về cấu trúc dữ liệu và enum của game.
//! Đây là những "thành phần" (components) và "tài nguyên" (resources) theo nghĩa đen,
//! không chứa logic, chỉ chứa dữ liệu.

use std::collections::VecDeque;
use std::time::{Duration, Instant};
use crate::engine::Entity;

// --- Các hằng số của Game ---
pub const MAX_FOOD_COUNT: usize = 1000;
pub const FOOD_LIFETIME_SECS: u64 = 60;
pub const GRID_SIZE: i32 = 40;

// --- Các Enum Trạng thái ---
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    Playing,
    GameOver,
}

#[derive( Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

// --- Các Component Dữ liệu ---
#[derive( Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub struct SnakeHead {
    pub direction: Direction,
}

pub struct SnakeBody;

pub struct Food;

pub struct Lifetime {
    pub spawn_time: Instant,
    pub duration: Duration,
}

// --- Các Sự kiện (Events) ---
// Đây là các struct rỗng dùng làm tín hiệu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Eaten(pub Entity);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dead;


// --- Các Resource Quản lý Trạng thái ---
pub struct Snake {
    // Sử dụng VecDeque để lưu trữ thứ tự các đốt thân một cách hiệu quả.
    // Phần tử đầu tiên là đầu, cuối cùng là đuôi.
    pub segments: VecDeque<Entity>,
}


pub struct GameTick {
    pub timer: Instant,
    pub rate: Duration,
}

#[derive( Default, Debug)]
pub struct Score(pub u32);
