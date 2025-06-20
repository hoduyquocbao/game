//! Tập hợp và export tất cả các module hệ thống con.

pub mod food;
pub mod input;
pub mod snake;

pub struct Transition;
impl crate::engine::System for Transition {
    fn access(&self) -> crate::engine::SystemAccess {
        crate::engine::SystemAccess {}
    }
    fn run(&mut self, world: &mut crate::world::World) {
        // Lắng nghe event Dead
        let has_dead = {
            if let Some(events) = world.get::<crate::engine::Events<crate::components::Dead>>() {
                events.read().next().is_some()
            } else {
                false
            }
        };
        if has_dead {
            if let Some(mut state) = world.get_mut::<crate::engine::State<crate::components::GameState>>() {
                *state = crate::engine::State(crate::components::GameState::GameOver);
            }
        }
    }
}

pub struct InputGameOver;
impl crate::engine::System for InputGameOver {
    fn access(&self) -> crate::engine::SystemAccess {
        crate::engine::SystemAccess {}
    }
    fn run(&mut self, world: &mut crate::world::World) {
        // Lắng nghe phím 'R' từ resource Input
        if let Some(input) = world.get::<crate::input::Input>() {
            if input.is_key_just_pressed("KeyR") || input.is_key_just_pressed("r") {
                if let Some(mut events) = world.get_mut::<crate::engine::Events<crate::components::Reset>>() {
                    events.send(crate::components::Reset);
                }
            }
        }
    }
}

pub struct ResetSystem;
impl crate::engine::System for ResetSystem {
    fn access(&self) -> crate::engine::SystemAccess {
        crate::engine::SystemAccess {}
    }
    fn run(&mut self, world: &mut crate::world::World) {
        // Lắng nghe event Reset
        let has_reset = {
            if let Some(events) = world.get::<crate::engine::Events<crate::components::Reset>>() {
                events.read().next().is_some()
            } else {
                false
            }
        };
        if !has_reset {
            return;
        }
        // Despawn toàn bộ Head, Body, Food
        let mut to_despawn = Vec::new();
        for (entity, _) in world.query_component::<(crate::components::Head,)>() {
            to_despawn.push(entity);
        }
        for (entity, _) in world.query_component::<(crate::components::Body,)>() {
            to_despawn.push(entity);
        }
        for (entity, _) in world.query_component::<(crate::components::Food,)>() {
            to_despawn.push(entity);
        }
        if let Some(mut commands) = world.get_mut::<crate::world::Commands>() {
            for entity in to_despawn {
                commands.despawn(entity);
            }
        }
        // Reset Score
        if let Some(mut score) = world.get_mut::<crate::components::Score>() {
            score.0 = 0;
        }
        // Spawn lại rắn chỉ có Head ở giữa màn hình
        let center = crate::components::Position { x: crate::components::GRID_SIZE / 2, y: crate::components::GRID_SIZE / 2 };
        if let Some(mut commands) = world.get_mut::<crate::world::Commands>() {
            commands.spawn()
                .with(crate::components::Head { direction: crate::components::Direction::Right })
                .with(center);
        }
        // Đặt state về Playing
        if let Some(mut state) = world.get_mut::<crate::engine::State<crate::components::GameState>>() {
            *state = crate::engine::State(crate::components::GameState::Playing);
        }
    }
}
