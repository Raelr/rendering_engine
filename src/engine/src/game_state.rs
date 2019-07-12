use crate::components::{ColorComponent, PositionComponent, TimerComponent};
use crate::generational_index::generational_index::*;
use crate::renderer::renderer_component::{RenderComponent};

type Entity = GenerationalIndex;
type EntityMap<T> = GenerationalIndexArray<T>;

/// GameState object stores all entities and components within itself. If handles the streaming of
/// components into different systems.

pub struct GameState {
    pub render_components : EntityMap<RenderComponent>,
    pub color_components : EntityMap<ColorComponent>,
    pub position_components : EntityMap<PositionComponent>,
    pub timer_components : EntityMap<TimerComponent>,
    pub allocator : GenerationalIndexAllocator,
    pub entities : Vec<Entity>
}

/// should store all components and entity IDs when actual gameobjects and players are added to the game.
/// TODO: populate GameState with relevant variables.

impl GameState {

    pub fn create_initial_state() -> GameState {
        let state = GameState {
            render_components : EntityMap::new(),
            color_components : EntityMap::new(),
            position_components : EntityMap::new(),
            timer_components : EntityMap::new(),
            allocator : GenerationalIndexAllocator::new(),
            entities : Vec::new()
        };

        state
    }

    pub fn create_entity(&mut self) -> Entity {

        let entity = self.allocator.allocate();

        entity
    }

    pub fn register_renderer(&mut self, entity : &Entity, value : RenderComponent) {

        self.render_components.set(&entity, value);

        self.sync_registries();
    }

    pub fn register_position(&mut self, entity : &Entity, value : PositionComponent) {

        self.position_components.set(&entity, value);

        self.sync_registries();
    }

    pub fn register_color(&mut self, entity : &Entity, value : ColorComponent) {

        self.color_components.set(&entity, value);

        self.sync_registries();
    }

    pub fn register_timer(&mut self, entity : &Entity, value : TimerComponent) {

        self.timer_components.set(&entity, value);

        self.sync_registries();
    }

    pub fn sync_registries(&mut self) {

        let entities = self.entities.len();

        if self.render_components.entries.len() < entities {

            for entity in self.render_components.entries.len()..entities {

                self.render_components.set_empty();
            }
        }

        if self.position_components.entries.len() < entities {

            for entity in self.position_components.entries.len()..entities {

                self.position_components.set_empty();
            }
        }

        if self.color_components.entries.len() < entities {

            for entity in self.color_components.entries.len()..entities {

                self.color_components.set_empty();
            }
        }

        if self.timer_components.entries.len() < entities {

            for entity in self.timer_components.entries.len()..entities {

                self.timer_components.set_empty();
            }
        }
    }

    pub fn register_entity(&mut self, entity : Entity) {

        if entity.index() < (self.entities.len()) {

            let idx = entity.index();
            self.entities[idx] = entity;

        } else {

            self.entities.push(entity);
        }
    }
}