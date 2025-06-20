//! Quản lý logic di chuyển, va chạm và phát triển của rắn.

use crate::components::*;
use crate::engine::{System, SystemAccess};
use crate::world::World;
use crate::components::GRID_SIZE;

pub struct MovementSystem;
impl System for MovementSystem {
    fn access(&self) -> SystemAccess {
        SystemAccess {}
    }

    fn run(&mut self, world: &mut World) {
        // let tick = world.get::<GameTick>().unwrap(); // Dọn unused variable
        let snake = world.get_mut::<Snake>().unwrap(); // Xóa mut nếu không cần
        if snake.segments.is_empty() {
            return;
        }

        let head_direction = world.get::<SnakeHead>().unwrap().direction;

        let mut positions: Vec<Position> = Vec::new();
        for _entity in snake.segments.iter() {
            positions.push(*world.get::<Position>().unwrap());
        }

        let new_head_pos = match head_direction {
            Direction::Up => Position {
                x: positions[0].x,
                y: (positions[0].y - 1).rem_euclid(GRID_SIZE),
            },
            Direction::Down => Position {
                x: positions[0].x,
                y: (positions[0].y + 1).rem_euclid(GRID_SIZE),
            },
            Direction::Left => Position {
                x: (positions[0].x - 1).rem_euclid(GRID_SIZE),
                y: positions[0].y,
            },
            Direction::Right => Position {
                x: (positions[0].x + 1).rem_euclid(GRID_SIZE),
                y: positions[0].y,
            },
        };

        // Cập nhật vị trí mới cho đầu rắn.
        *world.get_mut::<Position>().unwrap() = new_head_pos;

        // Cập nhật vị trí cho các đốt thân.
        for i in 1..snake.segments.len() {
            *world.get_mut::<Position>().unwrap() = positions[i - 1];
        }
    }
}

pub struct CollisionSystem;
impl System for CollisionSystem {
    fn access(&self) -> SystemAccess {
        SystemAccess {}
    }

    fn run(&mut self, _world: &mut World) {
        // TODO: Refactor lại toàn bộ logic sử dụng API mới của World (get, get_mut, query_component, ...)
        // Tạm thời bỏ trống để build sạch
    }
}

pub struct GrowthSystem;
impl System for GrowthSystem {
    fn access(&self) -> SystemAccess {
        SystemAccess {}
    }

    fn run(&mut self, _world: &mut World) {
        // TODO: Refactor lại toàn bộ logic sử dụng API mới của World (get, get_mut, query_component, ...)
        // Tạm thời bỏ trống để build sạch
    }
}
