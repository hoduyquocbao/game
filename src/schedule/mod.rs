//! Tổ chức và thực thi các hệ thống (systems).

use crate::components::GameState;
use crate::engine::System;
use crate::world::World;
use std::collections::HashMap;

// pub mod executor; // Sẽ được triển khai trong tương lai

/// Một tập hợp các hệ thống sẽ được thực thi trong một giai đoạn.
/// Các hệ thống trong một `Stage` có thể chạy song song.
#[derive(Default)]
pub struct Stage {
    systems: Vec<Box<dyn System>>,
}

impl Stage {
    /// Tạo một Stage mới.
    pub fn new() -> Self {
        Self::default()
    }

    /// Thêm một hệ thống vào Stage.
    pub fn add(&mut self, system: impl System + 'static) {
        self.systems.push(Box::new(system));
    }

    /// Chạy tất cả các hệ thống trong Stage.
    pub fn run(&mut self, world: &mut World) {
        // Tạm thời chạy tuần tự. Executor song song sẽ được thêm sau.
        for system in self.systems.iter_mut() {
            system.run(world);
        }
    }
}

/// Chứa một chuỗi các `Stage` để thực thi tuần tự.
#[derive(Default)]
pub struct Schedule {
    stages: Vec<Stage>,
}

impl Schedule {
    /// Tạo một Schedule mới.
    pub fn new() -> Self {
        Self::default()
    }

    /// Thêm một hệ thống vào stage cuối cùng của schedule.
    /// Nếu chưa có stage nào, một stage mới sẽ được tạo.
    pub fn add(&mut self, system: impl System + 'static) {
        if self.stages.is_empty() {
            self.stages.push(Stage::new());
        }
        self.stages.last_mut().unwrap().add(system);
    }

    /// Chạy tất cả các stage trong Schedule.
    pub fn run(&mut self, world: &mut World) {
        for stage in self.stages.iter_mut() {
            stage.run(world);
        }
    }
}

/// Lưu trữ tất cả các lịch trình cho các trạng thái game khác nhau.
#[derive(Default)]
pub struct Schedules(pub HashMap<GameState, Schedule>);

impl Schedules {
    /// Tạo một đối tượng Schedules mới.
    pub fn new() -> Self {
        Self::default()
    }

    /// Lấy hoặc chèn một Schedule cho một GameState cụ thể.
    pub fn entry(&mut self, state: GameState) -> &mut Schedule {
        self.0.entry(state).or_default()
    }

    /// Chạy schedule tương ứng với `GameState` hiện tại trong `World`.
    pub fn run(&mut self, world: &mut World) {
        // Lấy trạng thái game hiện tại từ World.
        let state = world.get::<crate::engine::State<crate::components::GameState>>().unwrap();
        let state_val = state.0;
        drop(state);
        if let Some(schedule) = self.0.get_mut(&state_val) {
            schedule.run(world);
        }
    }

    pub fn add(&mut self, state: GameState, schedule: Schedule) {
        self.0.insert(state, schedule);
    }

    pub fn get_mut(&mut self, state: &GameState) -> Option<&mut Schedule> {
        self.0.get_mut(state)
    }
}
