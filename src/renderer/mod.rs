//! Trừu tượng hóa hệ thống render.
pub mod gpu;
use crate::world::World;


/// Trait chung cho tất cả các backend render.
pub trait Renderer {
    /// Khởi tạo renderer với một window handle.
    fn new(_: ()) -> Self where Self: Sized;
    /// Vẽ trạng thái hiện tại của `World`.
    fn draw(&mut self, world: &World);
}

/// Hàm khởi tạo renderer mặc định (WGPU).
pub fn new(_: ()) -> Box<dyn Renderer> {
    Box::new(crate::renderer::gpu::Gpu::new(()))
}
