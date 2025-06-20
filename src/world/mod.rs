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

/// Builder để tạo một entity với các component một cách tiện lợi.
pub struct EntityCommands<'a> {
    entity: Entity,
    components: Vec<(TypeId, Box<dyn Any + Send + Sync>)>, 
    commands: &'a mut Commands,
}

impl<'a> EntityCommands<'a> {
    pub fn with<C: Component>(mut self, component: C) -> Self {
        self.components.push((TypeId::of::<C>(), Box::new(component)));
        self
    }

    pub fn id(&self) -> Entity {
        self.entity
    }
}

impl<'a> Drop for EntityCommands<'a> {
    fn drop(&mut self) {
        // Khi builder bị drop, lệnh spawn sẽ được thêm vào hàng đợi.
        let components = std::mem::take(&mut self.components);
        self.commands.0.push(Command::Spawn(components));
    }
}


/// Resource để xếp hàng các lệnh thay đổi cấu trúc World.
#[derive(Default)]
pub struct Commands(Vec<Command>);

impl Commands {
    pub fn spawn(&mut self, world: &mut World) -> EntityCommands {
        let entity = world.create_entity();
        EntityCommands {
            entity,
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

    /// Tạo một entity mới với các component đã cho.
    pub fn spawn(&mut self) -> EntityCommands {
        let entity = self.create_entity();
        // Lấy mutable reference đến Commands thông qua RefCell::get_mut
        let commands_cell = self.resources.get_mut(&TypeId::of::<Commands>()).unwrap();
        let commands = commands_cell.get_mut().downcast_mut::<Commands>().unwrap();
        EntityCommands {
            entity,
            components: Vec::new(),
            commands,
        }
    }

    /// Hủy một entity.
    pub fn despawn(&mut self, entity: Entity) {
        if let Some(location) = self.entities[entity.id()].take() {
            let archetype = &mut self.archetypes[location.archetype_id];
            let (moved_entity, _) = archetype.remove(location.index);
            // Cập nhật vị trí của entity đã bị di chuyển đến chỗ trống.
            self.entities[moved_entity.id()] = Some(Location { archetype_id: location.archetype_id, index: location.index });
        }
    }

    /// Áp dụng các lệnh đã được xếp hàng.
    pub fn apply_commands(&mut self) {
        let mut commands = self.get_mut::<Commands>().unwrap();
        let command_list = std::mem::take(&mut commands.0);
        drop(commands); // Giải phóng RefMut trước khi mượn lại world

        for command in command_list {
            match command {
                Command::Spawn(components) => {
                    let _entity = self.create_entity();
                    let _components = components;
                    // TODO: Thêm logic add component cho entity này
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
}
