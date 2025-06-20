//! Xây dựng và cấu hình ứng dụng thông qua builder pattern, quản lý vòng lặp chính.


use crate::{
    components::*,
    input::Input,
    plugin::Plugin,
    schedule::{Schedule, Schedules},
    world::World,
    engine::Resource,
    view::{GameView, GameOverView},
};
use dioxus::prelude::*;

/// Builder chính của ứng dụng, cung cấp một API chuỗi để cấu hình.
///
/// ## Ví dụ
/// ```rust,no_run
/// // use my_project::app::App;
/// // App::new()
/// //     .resource(MyResource)
/// //     .plugin(MyPlugin)
/// //     .run();
/// ```
pub struct App {
    pub world: World,
    schedules: Schedules,
}

impl Default for App {
    fn default() -> Self {
        let mut world = World::default();
        world.insert(Input::default());

        let mut schedules = Schedules::default();
        schedules.add(GameState::Playing, Schedule::new());
        schedules.add(GameState::GameOver, Schedule::new());

        Self {
            world,
            schedules,
        }
    }
}

impl App {
    /// Tạo một ứng dụng mới với các thành phần cốt lõi được khởi tạo.
    pub fn new() -> Self {
        Self::default()
    }

    /// Thêm một plugin vào ứng dụng.
    pub fn plugin(mut self, plugin: impl Plugin) -> Self {
        plugin.build(&mut self);
        self
    }

    /// Thêm một resource vào `World`.
    pub fn resource(&mut self, resource: impl Resource) {
        self.world.insert(resource);
    }

    /// Lấy một schedule để thêm các system vào.
    pub fn schedule(&mut self, state: GameState) -> &mut Schedule {
        self.schedules.get_mut(&state).unwrap()
    }

    /// Khởi chạy ứng dụng, mở cửa sổ và bắt đầu vòng lặp game.
    pub fn run(self) {
        launch(Root);
    }
}

/// Component gốc của Dioxus, quản lý vòng lặp game.
#[allow(non_snake_case)]
fn Root() -> Element {
    // TODO: Lấy state và score từ world thực tế (hiện tại chỉ mock)
    let state = crate::components::GameState::GameOver; // TODO: wiring thực tế
    let score = 42u32; // TODO: wiring thực tế
    match state {
        crate::components::GameState::GameOver => GameOverView(crate::view::GameOverViewProps { score }),
        _ => GameView(),
    }
}
