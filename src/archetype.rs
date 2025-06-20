//! Định nghĩa Archetype và các cấu trúc liên quan.

use crate::engine::Component;
use dioxus::prelude::Entity;
use std::any::{Any, TypeId};
use std::collections::HashMap;

/// Một kho lưu trữ cho một tập hợp các component có cùng kiểu.
pub struct Storage {
    // Sử dụng Box<dyn Any> để lưu trữ Vec<T> của bất kỳ kiểu T nào.
    data: Box<dyn Any + Send + Sync>,
}

impl Storage {
    pub fn new<T: Component>() -> Self {
        Self { data: Box::new(Vec::<T>::new()) }
    }

    pub fn as_slice<T: Component>(&self) -> &[T] {
        self.data.downcast_ref::<Vec<T>>().unwrap()
    }

    pub fn as_mut_slice<T: Component>(&mut self) -> &mut [T] {
        self.data.downcast_mut::<Vec<T>>().unwrap()
    }

    // ... các hàm khác để thêm, xóa component ...
}

/// Một Archetype đại diện cho một tập hợp các thực thể có cùng một bộ component.
#[derive(Default)]
pub struct Archetype {
    // Map từ TypeId của component đến vị trí của nó trong `storages`.
    types: HashMap<TypeId, usize>,
    storages: Vec<Storage>,
    entities: Vec<Entity>,
}

impl Archetype {
    pub fn new(types: Vec<TypeId>) -> Self {
        let mut archetype = Self::default();
        // Logic để tạo storages dựa trên types
        archetype
    }

    pub fn entities(&self) -> &[Entity] {
        &self.entities
    }

    // ... các hàm khác để quản lý entity và component trong archetype ...
}
