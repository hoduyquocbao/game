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
