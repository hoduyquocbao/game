//! Quản lý logic di chuyển, va chạm và phát triển của rắn.

use crate::components::*;
use crate::engine::{System, SystemAccess};
use crate::world::World;
use crate::components::GRID_SIZE;
use std::collections::HashMap;
use crate::engine::Entity;

/// Hệ thống di chuyển rắn (hai pha: cập nhật vị trí head, thu thập vị trí, cập nhật body)
pub struct Movement;
impl System for Movement {
    fn access(&self) -> SystemAccess {
        SystemAccess {}
    }
    fn run(&mut self, world: &mut World) {
        // PHA 1: Thu thập vị trí cũ của tất cả entity có Position
        let mut old_pos: HashMap<Entity, Position> = HashMap::new();
        for entity in world.query::<Position>() {
            // TODO: Khi World hỗ trợ query component thực sự, lấy đúng Position từng entity
            if let Some(pos) = world.get::<Position>() {
                old_pos.insert(entity, *pos);
            }
        }
        // PHA 2: Di chuyển Head
        for entity in world.query::<Head>() {
            // Lấy direction hiện tại
            if let (Some(head), Some(mut pos)) = (world.get_mut::<Head>(), world.get_mut::<Position>()) {
                let dir = head.direction;
                let mut new_pos = *pos;
                match dir {
                    Direction::Up => new_pos.y = (new_pos.y - 1).rem_euclid(GRID_SIZE),
                    Direction::Down => new_pos.y = (new_pos.y + 1).rem_euclid(GRID_SIZE),
                    Direction::Left => new_pos.x = (new_pos.x - 1).rem_euclid(GRID_SIZE),
                    Direction::Right => new_pos.x = (new_pos.x + 1).rem_euclid(GRID_SIZE),
                }
                *pos = new_pos;
                // Cập nhật lại trong old_pos để các Follow có thể lấy vị trí head cũ
                old_pos.insert(entity, *pos);
            }
        }
        // PHA 3: Cập nhật các entity có Follow
        for _entity in world.query::<Follow>() {
            if let (Some(follow), Some(mut pos)) = (world.get::<Follow>(), world.get_mut::<Position>()) {
                let target = follow.0;
                if let Some(target_pos) = old_pos.get(&target) {
                    *pos = *target_pos;
                }
            }
        }
    }
}

/// Hệ thống phát triển rắn khi ăn mồi
pub struct Growth;
impl System for Growth {
    fn access(&self) -> SystemAccess {
        SystemAccess {}
    }
    fn run(&mut self, _world: &mut World) {
        // TODO: Lắng nghe event Eaten, tìm đuôi, spawn entity mới với Body, Position, Follow
    }
}

/// Hệ thống kiểm tra va chạm
pub struct Collision;
impl System for Collision {
    fn access(&self) -> SystemAccess {
        SystemAccess {}
    }
    fn run(&mut self, _world: &mut World) {
        // TODO: Lấy Position của Head, so với tất cả Body để phát hiện va chạm
    }
}
