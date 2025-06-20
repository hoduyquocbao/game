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
        // --- PHA 1: TÍNH TOÁN VỊ TRÍ MỚI CHO HEAD ---
        let mut next_positions: HashMap<Entity, Position> = HashMap::new();
        for (entity, (head, pos)) in world.query_component::<(Head, Position)>() {
            let mut new_pos = *pos;
            match head.direction {
                Direction::Up => new_pos.y = (new_pos.y - 1).rem_euclid(GRID_SIZE),
                Direction::Down => new_pos.y = (new_pos.y + 1).rem_euclid(GRID_SIZE),
                Direction::Left => new_pos.x = (new_pos.x - 1).rem_euclid(GRID_SIZE),
                Direction::Right => new_pos.x = (new_pos.x + 1).rem_euclid(GRID_SIZE),
            }
            next_positions.insert(entity, new_pos);
        }
        // --- PHA 2: TÍNH TOÁN VỊ TRÍ MỚI CHO BODY ---
        for (entity, (follow, _body)) in world.query_component::<(Follow, Body)>() {
            // Lấy vị trí hiện tại của entity mà đốt này follow
            if let Some((_, target_pos)) = world.query_component::<(Position,)>().into_iter().find(|(e, _)| *e == follow.0) {
                next_positions.insert(entity, target_pos.0);
            }
        }
        // --- PHA 3: ÁP DỤNG TOÀN BỘ THAY ĐỔI ---
        for (entity, new_pos) in next_positions {
            for (e, idx) in world.query_component_mut::<Position>() {
                if e == entity {
                    // Tìm archetype chứa entity này
                    if let Some(archetype) = world.archetypes_mut().iter_mut().find(|a| a.entities().contains(&entity)) {
                        let type_id = std::any::TypeId::of::<Position>();
                        if let Some(storage) = archetype.get_component_storage_mut(type_id) {
                            let slice = storage.as_mut_slice::<Position>();
                            if idx < slice.len() {
                                slice[idx] = new_pos;
                            }
                        }
                    }
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
