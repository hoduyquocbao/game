//! Cung cấp backend render hiệu suất cao sử dụng WGPU.

use crate::{
    world::World,
    renderer::Renderer,
};
use glam::Mat4;


const MAX: usize = 10_000; // Số lượng instance tối đa

/// Dữ liệu cho mỗi instance được gửi lên GPU.
#[repr(C)]
struct Instance {
    model: Mat4,
    color: [f32; 4],
}

/// Dữ liệu uniform cho camera.
#[repr(C)]
struct Uniform {
    view_proj: Mat4,
}

/// Backend render sử dụng WGPU.
pub struct Gpu<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

impl<'a> Renderer for Gpu<'a> {
    fn new(_: ()) -> Self {
        // ... (Implementation of WGPU initialization)
        unimplemented!();
    }

    fn draw(&mut self, _world: &World) {
        // TODO: Thêm logic vẽ
    }
}
