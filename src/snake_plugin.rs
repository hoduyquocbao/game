//! Đóng gói và khởi tạo tất cả resource, system, và state cần thiết cho game Rắn Săn Mồi.

use crate::{
    app::App, components::*, engine, plugin::Plugin, systems::{food::*, input::*, snake::*}
};

/// Plugin chính của game, tuân thủ quy tắc đặt tên đơn từ.
/// Tên gốc: `SnakeGamePlugin`
pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.resource(engine::State::new(GameState::Playing));
        app.resource(Score::default());
        app.resource(GameTick {
            timer: std::time::Instant::now(),
            rate: std::time::Duration::from_millis(150),
        });
        app.resource(engine::Events::<Eaten>::default());
        app.resource(engine::Events::<Dead>::default());
        let playing_schedule = app.schedule(GameState::Playing);
        playing_schedule.add(InputSystem);
        playing_schedule.add(Movement);
        playing_schedule.add(Collision);
        playing_schedule.add(Growth);
        playing_schedule.add(FoodSpawnerSystem);
        playing_schedule.add(LifetimeSystem);
        // TODO: Thiết lập hệ thống cho trạng thái GameOver
    }
}