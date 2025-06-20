//! Xử lý các sự kiện bàn phím thô.

use crate::components::{Direction, Head};
use crate::engine::{System, SystemAccess};
use crate::input::Input;
use crate::world::World;

pub struct InputSystem;
impl System for InputSystem {
    fn access(&self) -> SystemAccess {
        SystemAccess {}
    }

    fn run(&mut self, world: &mut World) {
        let input = world.get::<Input>().unwrap();
        let new_dir = if input.is_key_just_pressed("ArrowUp") {
            Some(Direction::Up)
        } else if input.is_key_just_pressed("ArrowDown") {
            Some(Direction::Down)
        } else if input.is_key_just_pressed("ArrowLeft") {
            Some(Direction::Left)
        } else if input.is_key_just_pressed("ArrowRight") {
            Some(Direction::Right)
        } else {
            None
        };
        if let Some(new_dir) = new_dir {
            // TODO: Thay thế bằng query component đúng nếu World chưa có API query
            // Giả sử chỉ có 1 Head, lấy mutable reference đầu tiên
            if let Some(mut head) = world.get_mut::<Head>() {
                if new_dir != head.direction.opposite() {
                    head.direction = new_dir;
                }
            }
        }
    }
}
