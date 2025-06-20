//! Container trung tâm chứa tất cả entities, components, và resources.

pub mod archetype;

use archetype::Archetype;
use crate::engine::{Component, Entity, Resource};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::cell::{Ref, RefMut, RefCell};

/// Một map từ TypeId đến một đối tượng Any được đóng gói trong RefCell.
/// Dùng để lưu trữ các Resource.
type TypeIdMap<T> = HashMap<TypeId, RefCell<T>>;

/// Lưu trữ vị trí của một Entity trong World.
#[derive(Clone, Copy, Debug)]
pub struct Location {
    archetype_id: usize,
    index: usize,
}

/// Các lệnh được xếp hàng để thực thi trên World một cách an toàn.
pub enum Command {
    Spawn(Vec<(TypeId, Box<dyn Any + Send + Sync>)>),
    Despawn(Entity),
    AddComponent(Entity, TypeId, Box<dyn Any + Send + Sync>),
    RemoveComponent(Entity, TypeId),
}

/// Builder để tạo một entity với các component một cách tiện lợi (API mới, không cần World).
pub struct EntityBuilder<'a> {
    components: Vec<(TypeId, Box<dyn Any + Send + Sync>)>,
    commands: &'a mut Commands,
}

impl<'a> EntityBuilder<'a> {
    pub fn with<C: Component>(mut self, component: C) -> Self {
        self.components.push((TypeId::of::<C>(), Box::new(component)));
        self
    }
}

impl<'a> Drop for EntityBuilder<'a> {
    fn drop(&mut self) {
        let components = std::mem::take(&mut self.components);
        self.commands.0.push(Command::Spawn(components));
    }
}

/// Resource để xếp hàng các lệnh thay đổi cấu trúc World.
#[derive(Default)]
pub struct Commands(Vec<Command>);

impl Commands {
    /// API mới: chỉ buffer ý định spawn, không cần World
    pub fn spawn(&mut self) -> EntityBuilder {
        EntityBuilder {
            components: Vec::new(),
            commands: self,
        }
    }

    pub fn despawn(&mut self, entity: Entity) {
        self.0.push(Command::Despawn(entity));
    }
}

/// World là container trung tâm cho tất cả dữ liệu trong game.
#[derive(Default)]
pub struct World {
    entities: Vec<Option<Location>>,
    archetypes: Vec<Archetype>,
    // Map từ một tập hợp các component (đại diện bằng HashSet<TypeId>) đến ID của archetype.
    archetype_map: HashMap<Vec<TypeId>, usize>,
    resources: TypeIdMap<Box<dyn Any + Send + Sync>>,
    pub tick_count: u64, // Để kích hoạt re-render
}

impl World {
    /// Tạo một World mới.
    pub fn new() -> Self {
        let mut world = Self::default();
        // Mọi World đều có resource Commands.
        world.insert(Commands::default());
        world
    }

    /// Tạo một entity mới và trả về ID của nó.
    fn create_entity(&mut self) -> Entity {
        let id = self.entities.len();
        self.entities.push(None);
        Entity::new(id)
    }

    /// Chèn một resource vào World.
    pub fn insert<R: Resource>(&mut self, resource: R) {
        self.resources.insert(TypeId::of::<R>(), RefCell::new(Box::new(resource)));
    }

    /// Lấy một tham chiếu không đổi đến một resource.
    pub fn get<R: Resource>(&self) -> Option<Ref<R>> {
        self.resources.get(&TypeId::of::<R>()).map(|cell| {
            Ref::map(cell.borrow(), |b| b.downcast_ref::<R>().unwrap())
        })
    }

    /// Lấy một tham chiếu có thể thay đổi đến một resource.
    pub fn get_mut<R: Resource>(&self) -> Option<RefMut<R>> {
        self.resources.get(&TypeId::of::<R>()).map(|cell| {
            RefMut::map(cell.borrow_mut(), |b| b.downcast_mut::<R>().unwrap())
        })
    }

    /// Áp dụng các lệnh đã được xếp hàng.
    pub fn apply_commands(&mut self) {
        let mut commands = self.get_mut::<Commands>().unwrap();
        let command_list = std::mem::take(&mut commands.0);
        drop(commands); // Giải phóng RefMut trước khi mượn lại world

        for command in command_list {
            match command {
                Command::Spawn(components) => {
                    // Tạo entity mới và add các component vào archetype đúng
                    let entity = self.create_entity();
                    // Xác định archetype phù hợp
                    let type_ids: Vec<TypeId> = components.iter().map(|(t, _)| *t).collect();
                    let archetype_id = if let Some(&id) = self.archetype_map.get(&type_ids) {
                        id
                    } else {
                        let id = self.archetypes.len();
                        self.archetypes.push(Archetype::new(&type_ids));
                        self.archetype_map.insert(type_ids.clone(), id);
                        id
                    };
                    let archetype = &mut self.archetypes[archetype_id];
                    let index = archetype.add(entity, components);
                    self.entities[entity.id()] = Some(Location { archetype_id, index });
                }
                Command::Despawn(entity) => {
                    let _entity = entity;
                    // TODO: Thực hiện despawn entity
                }
                // TODO: Implement AddComponent/RemoveComponent
                _ => {}
            }
        }
    }

    /// Trả về iterator các entity có component kiểu T
    pub fn query<T: Component>(&self) -> impl Iterator<Item = Entity> + '_ {
        self.entities.iter().enumerate().filter_map(move |(i, loc)| {
            if loc.is_some() {
                // TODO: kiểm tra entity này có component T không (giả lập: trả về tất cả entity)
                Some(Entity::new(i))
            } else {
                None
            }
        })
    }

    /// Trả về Vec<(Entity, &T)> cho tất cả entity có component T
    pub fn query_component<T: Component>(&self) -> Vec<(Entity, &T)> {
        let mut result = Vec::new();
        for archetype in &self.archetypes {
            let type_id = std::any::TypeId::of::<T>();
            if let Some(storage) = archetype.get_component_storage(type_id) {
                let slice = storage.as_slice::<T>();
                for (i, entity) in archetype.entities().iter().enumerate() {
                    result.push((*entity, &slice[i]));
                }
            }
        }
        result
    }

    /// Trả về Vec<(Entity, usize)> cho tất cả entity có component T (dùng index để mutate)
    pub fn query_component_mut<T: Component>(&mut self) -> Vec<(Entity, usize)> {
        let mut result = Vec::new();
        for archetype in &mut self.archetypes {
            let type_id = std::any::TypeId::of::<T>();
            if let Some(storage) = archetype.get_component_storage_mut(type_id) {
                let len = storage.as_mut_slice::<T>().len();
                for (i, entity) in archetype.entities().iter().enumerate().take(len) {
                    result.push((*entity, i));
                }
            }
        }
        result
    }

    pub fn archetypes_mut(&mut self) -> &mut Vec<crate::world::archetype::Archetype> {
        &mut self.archetypes
    }
}
