//! Điểm khởi chạy chính của ứng dụng.
//! Trách nhiệm duy nhất của nó là tạo `App` và thêm `Plugin` của game vào.
//! Sự đơn giản của file này là thước đo thành công của kiến trúc framework.

// Khai báo tất cả các module của dự án.
mod app;
mod camera;
mod components;
mod engine;
mod input;
mod plugin;
mod renderer;
mod schedule;
mod snake_plugin;
mod systems;
mod view;
mod world;

use app::App;
use snake_plugin::SnakePlugin;

fn main() {
    // Thiết lập logger để xem các thông báo từ wgpu và engine
    env_logger::init();

    // Xây dựng và chạy ứng dụng theo kiến trúc plugin.
    // Hoàn toàn tách biệt engine và game.
    App::new().plugin(SnakePlugin).run();
}
