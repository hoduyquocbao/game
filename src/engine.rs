//! Các trait và kiểu dữ liệu cốt lõi của engine.

use crate::world::World;

// --- Core Traits & Structs ---

/// Trait đánh dấu một struct là một component, một mẩu dữ liệu gắn với một entity.
pub trait Component: Send + Sync + 'static {}
impl<T: Send + Sync + 'static> Component for T {}

/// Trait đánh dấu một struct là một resource, một tài nguyên toàn cục.
pub trait Resource: Send + Sync + 'static {}
impl<T: Send + Sync + 'static> Resource for T {}

/// Một đơn vị logic thực thi trong game.
pub trait System: 'static + Send + Sync {
    /// Khai báo quyền truy cập dữ liệu của hệ thống.
    fn access(&self) -> SystemAccess;
    /// Chạy logic của hệ thống.
    fn run(&mut self, world: &mut World);
}

// --- State & Events ---

pub struct State<T: Send + Sync + 'static>(pub T);

impl<T: Send + Sync + 'static> State<T> {
    /// Tạo một State mới với giá trị ban đầu.
    pub fn new(value: T) -> Self {
        Self(value)
    }
    /// Lấy reference tới giá trị bên trong State.
    pub fn get(&self) -> &T {
        &self.0
    }
}

pub struct Events<T: Event> {
    a: Vec<T>,
    b: Vec<T>,
}

impl<T: Event> Default for Events<T> {
    fn default() -> Self {
        Self { a: Vec::new(), b: Vec::new() }
    }
}

impl<T: Event> Events<T> {
    /// Gửi một sự kiện.
    pub fn send(&mut self, event: T) {
        self.a.push(event);
    }

    /// Đọc tất cả các sự kiện đã được gửi từ lần cập nhật trước.
    pub fn read(&self) -> impl Iterator<Item = &T> {
        self.b.iter()
    }

    /// Cập nhật các buffer, chuẩn bị cho tick tiếp theo.
    /// Các sự kiện được gửi trong tick này sẽ có thể đọc được ở tick sau.
    pub fn update(&mut self) {
        self.b.clear();
        std::mem::swap(&mut self.a, &mut self.b);
    }
}

pub trait Event: Send + Sync + 'static + Clone {}
impl<T: Send + Sync + 'static + Clone> Event for T {}

// --- Entity & World Placeholders ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Entity(u64);

impl Entity {
    pub fn new(id: usize) -> Self {
        Entity(id as u64)
    }
    pub fn id(&self) -> usize {
        self.0 as usize
    }
}

pub struct SystemAccess;
