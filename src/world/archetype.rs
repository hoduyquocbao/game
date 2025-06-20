//! Định nghĩa Archetype, một tập hợp các thực thể có cùng một bộ component.

use crate::engine::Entity;
use std::any::{Any, TypeId};
use std::collections::HashMap;

/// Lưu trữ dữ liệu cho một loại component cụ thể dưới dạng một vector các byte.
/// Sử dụng `Box<dyn Any>` để có thể lưu trữ bất kỳ kiểu dữ liệu nào.
pub struct ComponentVec(Vec<Box<dyn Any + Send + Sync>>);

impl ComponentVec {
    /// Tạo một ComponentVec mới.
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Đẩy một component vào vector.
    /// `component` phải là `Box<dyn Any + Send + Sync>`.
    pub fn push(&mut self, component: Box<dyn Any + Send + Sync>) {
        self.0.push(component);
    }

    /// Xóa một component tại một chỉ mục và trả về nó.
    pub fn swap_remove(&mut self, index: usize) -> Box<dyn Any + Send + Sync> {
        self.0.swap_remove(index)
    }

    /// Lấy một tham chiếu không đổi đến một component tại một chỉ mục.
    pub fn get(&self, index: usize) -> Option<&(dyn Any + Send + Sync)> {
        self.0.get(index).map(|b| b.as_ref())
    }

    /// Lấy một tham chiếu có thể thay đổi đến một component tại một chỉ mục.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut (dyn Any + Send + Sync)> {
        self.0.get_mut(index).map(|b| b.as_mut())
    }
}

/// Một Archetype đại diện cho một tập hợp các thực thể có cùng một bộ component.
/// Dữ liệu được lưu trữ theo cấu trúc SoA (Struct of Arrays) để tối ưu truy cập.
pub struct Archetype {
    /// Map từ TypeId của component đến vector chứa dữ liệu của component đó.
    components: HashMap<TypeId, ComponentVec>,
    /// Danh sách các thực thể thuộc về archetype này.
    entities: Vec<Entity>,
}

impl Archetype {
    /// Tạo một Archetype mới với một tập hợp các TypeId của component.
    pub fn new(component_types: &[TypeId]) -> Self {
        let mut components = HashMap::new();
        for type_id in component_types {
            components.insert(*type_id, ComponentVec::new());
        }
        Self { components, entities: Vec::new() }
    }

    /// Thêm một thực thể và dữ liệu component của nó vào archetype.
    pub fn add(&mut self, entity: Entity, components: Vec<(TypeId, Box<dyn Any + Send + Sync>)>) -> usize {
        self.entities.push(entity);
        let index = self.entities.len() - 1;
        for (type_id, component_data) in components {
            self.components.get_mut(&type_id).unwrap().push(component_data);
        }
        index
    }

    /// Xóa một thực thể khỏi archetype tại một chỉ mục.
    /// Trả về thực thể đã bị di chuyển đến vị trí của thực thể bị xóa (do swap_remove).
    pub fn remove(&mut self, index: usize) -> (Entity, Vec<(TypeId, Box<dyn Any + Send + Sync>)>) {
        let removed_entity = self.entities.swap_remove(index);
        let mut removed_components = Vec::new();
        for (type_id, component_vec) in self.components.iter_mut() {
            removed_components.push((*type_id, component_vec.swap_remove(index)));
        }
        (removed_entity, removed_components)
    }

    /// Lấy danh sách các thực thể trong archetype.
    pub fn entities(&self) -> &[Entity] {
        &self.entities
    }

    /// Lấy dữ liệu của một component cho một thực thể tại một chỉ mục.
    pub fn get_component_storage(&self, type_id: TypeId) -> Option<&ComponentVec> {
        self.components.get(&type_id)
    }

    /// Lấy dữ liệu có thể thay đổi của một component cho một thực thể tại một chỉ mục.
    pub fn get_component_storage_mut(&mut self, type_id: TypeId) -> Option<&mut ComponentVec> {
        self.components.get_mut(&type_id)
    }
}
