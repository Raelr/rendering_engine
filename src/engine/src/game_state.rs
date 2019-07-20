use crate::components::{ColorComponent, PositionComponent, TimerComponent, RenderComponent, Component};
use crate::generational_index::generational_index::*;
use std::time::{Instant};
use crate::renderer::shaders::shader::Shader;
use crate::renderer::shaders::shader_program::ShaderProgram;
use std::ffi::{CString};
use anymap::AnyMap;

/// Types for the generational indices and arrays.

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

        let state = GameState {
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

    pub fn create_entity(state : &mut GameState) -> EntityBuilder {

        let entity = state.allocator.allocate();

        let idx = entity.index();

        if idx < state.entities.len() {

            state.entities[idx] = entity;

        } else {

            state.entities.push(entity);
        }

        EntityBuilder::new(state.entities[idx].clone(), state)
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

        let _first_comp = GameState::create_entity(state)
            .with(RenderComponent {shader_program : triangle_render!()})
            .with(PositionComponent {position : (0.0, 0.0, 0.0), reversed : true })
            .with(ColorComponent {color : (1.0, 1.0, 1.0, 0.0), use_vertex_colors : false, use_position : false})
            .build();

//        // LEFT
//
//        let _second_comp = GameState::create_entity(state)
//            .with(RenderComponent {shader_program : triangle_render!()})
//            .with(PositionComponent {position : (-0.5, 0.0, 0.0), reversed : false })
//            .with(ColorComponent {color : (0.0, 0.0, 0.0, 0.0), use_vertex_colors : true, use_position : false})
//            .build();
//
//        //CENTRE
//
//        let _third_comp = GameState::create_entity(state)
//            .with(RenderComponent {shader_program : triangle_render!()})
//            .with(PositionComponent {position : (0.0, 0.0, 0.0), reversed : true })
//            .with(ColorComponent {color : (0.0, 0.0, 0.0, 0.0), use_vertex_colors : false, use_position : false})
//            .with(TimerComponent { now : Instant::now()})
//            .build();
    }
}

/// Struct for the EntityBuilder. The struct allows the user to easily build and configure entities
/// within the the game.
pub struct EntityBuilder<'a>{

    id : GenerationalIndex,
    state : &'a mut GameState
}

/// Temporary container for building and configuring entities.
impl<'a> EntityBuilder<'a> {

    /// Basic Constructor.
    pub fn new(id : GenerationalIndex, state : &'a mut GameState) -> EntityBuilder {

        EntityBuilder { id, state}
    }

    /// Function used for adding new components to the entity.
    pub fn with<T : Component >(self, component : T) -> Self {

        self.state.register_component(component, &self.id);

        self
    }

    /// Simply returns the ID of the entity.
    pub fn build(self) -> Entity{
        self.id
    }
}