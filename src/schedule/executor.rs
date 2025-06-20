//! Chứa logic thực thi các hệ thống trong một `Schedule` song song.

use crate::engine::{System, World};

/// Phân tích đồ thị phụ thuộc và thực thi các hệ thống song song bằng `rayon`.
///
/// # Safety
/// Việc sử dụng `unsafe` để có được `&mut World` từ con trỏ là an toàn vì
/// thuật toán xây dựng lô đã đảm bảo không có data race.
pub fn parallel(world: &mut World, systems: &mut [Box<dyn System>]) {
    // TODO: Implement the actual parallel execution logic.
    // For now, run systems sequentially.
    for system in systems {
        system.run(world);
    }
}
