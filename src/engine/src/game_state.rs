use crate::components::{ColorComponent, PositionComponent, TimerComponent};
use crate::generational_index::generational_index::*;
use crate::renderer::renderer_component::{RenderComponent};
use std::time::{Instant};
use crate::renderer::shaders::shader::Shader;
use crate::renderer::shaders::shader_program::ShaderProgram;
use std::ffi::{CString};
use std::borrow::BorrowMut;
use anymap::AnyMap;

type Entity = GenerationalIndex;
type EntityMap<T> = GenerationalIndexArray<T>;

/// GameState object stores all entities and components within itself. If handles the streaming of
/// components into different systems.

pub struct GameState {

    pub components : AnyMap,
    pub allocator : GenerationalIndexAllocator,
    pub entities : Vec<Entity>
}

/// should store all components and entity IDs when actual gameobjects and players are added to the game.
/// TODO: populate GameState with relevant variables.

impl GameState {

    pub fn create_initial_state() -> GameState {

        let mut state = GameState {
            components : AnyMap::new(),
            allocator : GenerationalIndexAllocator::new(),
            entities : Vec::new()
        };

        state
    }

    pub fn register_component<T : 'static>(&mut self, component : T, index : &GenerationalIndex) {

        let map = self.components.get_mut::<EntityMap<T>>().unwrap();

        GameState::sync_registry(&self.entities, map);

        map.set(index, component);
    }

    pub fn register_map<T : 'static>(&mut self, component : GenerationalIndexArray<T>) {

        self.components.insert(component);
    }

    pub fn create_entity(entities : &mut Vec<Entity>, allocator : &mut GenerationalIndexAllocator) -> GenerationalIndex {

        let entity = allocator.allocate();

        let idx = entity.index();

        if idx < entities.len() {

            entities[idx] = entity;

        } else {

            entities.push(entity);
        }

        entities[idx].clone()
    }

    pub fn get_map_mut<T : 'static>(&mut self) -> &mut EntityMap<T> {

        self.components.get_mut::<EntityMap<T>>().unwrap()
    }

    pub fn get_map<T : 'static>(&self) -> &EntityMap<T> {

        self.components.get::<EntityMap<T>>().unwrap()
    }

    pub fn sync_registry<T>(entities : &Vec<Entity>, array : &mut GenerationalIndexArray<T>) {

        let entities = entities.len();

        if array.entries.len() < entities {

            for _entity in array.entries.len()..entities {

                array.set_empty();
            }
        }
    }

    pub fn init_test_state(state : &mut GameState) {

        let render_comps : EntityMap<RenderComponent> = EntityMap::new();
        let pos_comps : EntityMap<PositionComponent> = EntityMap::new();
        let color_comps : EntityMap<ColorComponent> = EntityMap::new();
        let timer_comps : EntityMap<TimerComponent> = EntityMap::new();

        state.register_map(render_comps);
        state.register_map(pos_comps);
        state.register_map(color_comps);
        state.register_map(timer_comps);

        // RIGHT

        let first_comp = GameState::create_entity(&mut state.entities, &mut state.allocator);

        state.register_component(RenderComponent { shader_program: triangle_render!() }, &first_comp);

        state.register_component(PositionComponent { position : (0.5, 0.0, 0.0), reversed : false }, &first_comp);

        state.register_component(ColorComponent { color : (0.0, 0.0, 0.0, 0.0), use_vertex_colors : false, use_position : true}, &first_comp);

        // LEFT

        let second_comp = GameState::create_entity(&mut state.entities, &mut state.allocator);

        state.register_component(RenderComponent { shader_program: triangle_render!() }, &second_comp);

        state.register_component(PositionComponent { position : (-0.5, 0.0, 0.0), reversed : false }, &second_comp);

        state.register_component(ColorComponent { color : (0.0, 0.0, 0.0, 0.0), use_vertex_colors : true, use_position : false}, &second_comp);

        // CENTER

        let third_comp = GameState::create_entity(&mut state.entities, &mut state.allocator);

        state.register_component(RenderComponent { shader_program: triangle_render!() }, &third_comp);

        state.register_component(PositionComponent { position : (0.0, 0.0, 0.0), reversed : true }, &third_comp);

        state.register_component(ColorComponent { color : (0.0, 0.0, 0.0, 0.0), use_vertex_colors : false, use_position : false}, &third_comp);

        state.register_component(TimerComponent { now : Instant::now()}, &third_comp);
    }

}