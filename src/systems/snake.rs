//! Quản lý logic di chuyển, va chạm và phát triển của rắn.

use crate::components::*;
use crate::engine::{System, SystemAccess};
use crate::world::World;
use crate::components::GRID_SIZE;
use std::collections::HashMap;
use crate::engine::Entity;
use std::collections::HashSet;

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
    fn run(&mut self, world: &mut World) {
        // Lắng nghe event Eaten
        let eaten_targets: Vec<Entity> = {
            let mut result = Vec::new();
            {
                if let Some(events) = world.get::<crate::engine::Events<Eaten>>() {
                    for e in events.read() {
                        result.push(e.0);
                    }
                }
            }
            result
        };
        if eaten_targets.is_empty() {
            return;
        }
        // Tìm đuôi: Body không bị Follow
        let mut followed: HashSet<Entity> = HashSet::new();
        for (_entity, follow) in world.query_component::<(Follow,)>() {
            followed.insert(follow.0.0);
        }
        let mut tail_entity = None;
        let mut tail_pos = None;
        for (entity, (_body, pos)) in world.query_component::<(Body, Position)>() {
            if !followed.contains(&entity) {
                tail_entity = Some(entity);
                tail_pos = Some(*pos);
                break;
            }
        }
        if let (Some(tail_entity), Some(tail_pos)) = (tail_entity, tail_pos) {
            let mut commands = world.get_mut::<crate::world::Commands>().unwrap();
            commands.spawn(world)
                .with(Body)
                .with(tail_pos)
                .with(Follow(tail_entity));
        }
    }
}

/// Hệ thống kiểm tra va chạm
pub struct Collision;
impl System for Collision {
    fn access(&self) -> SystemAccess {
        SystemAccess {}
    }
    fn run(&mut self, world: &mut World) {
        // Lấy Position của Head
        let mut head_pos: Option<Position> = None;
        for (_entity, (_head, pos)) in world.query_component::<(Head, Position)>() {
            head_pos = Some(*pos);
            break;
        }
        if head_pos.is_none() {
            return;
        }
        let head_pos = head_pos.unwrap();
        // So với tất cả Body
        for (_entity, (_body, pos)) in world.query_component::<(Body, Position)>() {
            if *pos == head_pos {
                // Gửi event Dead
                if let Some(mut events) = world.get_mut::<crate::engine::Events<Dead>>() {
                    events.send(Dead);
                }
                break;
            }
        }
    }
}
