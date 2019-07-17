use crate::components::{ColorComponent, PositionComponent, TimerComponent, RenderComponent};
use crate::generational_index::generational_index::*;
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

impl GameState {

    /// Basic constructor for the game state.
    pub fn create_initial_state() -> GameState {

        let mut state = GameState {
            components : AnyMap::new(),
            allocator : GenerationalIndexAllocator::new(),
            entities : Vec::new()
        };

        state
    }

    /// Takes in a generic component and attempts to map it to a type in the component anymap.

    pub fn register_component<T : 'static>(&mut self, component : T, index : &GenerationalIndex) {

        if let Some(m) = self.components.get_mut::<EntityMap<T>>() {

            GameState::sync_registry(&self.entities, m);

            m.set(index, component);

        } else {
            eprintln!("The component does not exist!");
        }
    }

    /// used to register a component array to the anymap

    pub fn register_map<T : 'static>(&mut self, component : GenerationalIndexArray<T>) {

        self.components.insert(component);
    }

    /// Allocates a generational index and adds it to the entity vector

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

    /// Returns a mutable reference of the map

    pub fn get_map_mut<T : 'static>(&mut self) -> &mut EntityMap<T> {

        self.components.get_mut::<EntityMap<T>>().unwrap()
    }

    /// Returns an immutable reference of the component map

    pub fn get_map<T : 'static>(&self) -> &EntityMap<T> {

        self.components.get::<EntityMap<T>>().unwrap()
    }

    /// Ensures that the inputted index array is the same size as the number of entities
    /// (Each entity can have ONE of each component)

    pub fn sync_registry<T>(entities : &Vec<Entity>, array : &mut GenerationalIndexArray<T>) {

        let entities = entities.len();

        if array.entries.len() < entities {

            for _entity in array.entries.len()..entities {

                array.set_empty();
            }
        }
    }

    /// A sandbox for experimenting with component creation. The goal is to have entity creation be
    /// reduced to one or two lines of code.

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