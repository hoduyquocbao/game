//! Quản lý logic tạo và hủy mồi.

use crate::{components::*, engine::{System, SystemAccess}, world::World};
use rand::Rng;
use crate::components::GRID_SIZE;

pub struct FoodSpawnerSystem;
impl System for FoodSpawnerSystem {
    fn access(&self) -> SystemAccess {
        SystemAccess {}
    }

    fn run(&mut self, _world: &mut World) {
        let mut rng = rand::thread_rng();
        let _pos = Position {
            x: rng.gen_range(0..GRID_SIZE),
            y: rng.gen_range(0..GRID_SIZE),
        };
        // TODO: Thêm logic spawn entity vào world
    }
}

pub struct LifetimeSystem;
impl System for LifetimeSystem {
    fn access(&self) -> SystemAccess {
        SystemAccess {}
    }

    fn run(&mut self, _world: &mut World) {
        // TODO: Thay thế bằng query đúng nếu World chưa có API query
        // Không thực hiện despawn để tránh lỗi
    }
}
