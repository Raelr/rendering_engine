use crate::generational_index::generational_index::EntryValue::Empty;
use anymap::Entry;

#[derive(Clone, Copy)]
pub struct GenerationalIndex {
    pub index : usize,
    pub generation: u64
}

impl GenerationalIndex {

    pub fn index(&self) -> usize {
        self.index
    }
}

pub enum EntryValue {
    Full(u64),
    Empty
}

#[derive(Clone, Copy)]
pub struct ArrayEntry<T> {
    pub value : T,
    pub generation : u64
}

pub struct GenerationalIndexArray<T> {

    pub unpacked_entries : Vec<EntryValue>,
    pub entries : Vec<Option<ArrayEntry<T>>>
}

impl<T> GenerationalIndexArray<T> {

    pub fn new() -> GenerationalIndexArray<T> {

        let array = GenerationalIndexArray {
            unpacked_entries: Vec::with_capacity(1024),
            entries: Vec::with_capacity(1024)
        };

        array
    }

    pub fn set_empty(&mut self) {

        println!("Setting empty...");
        &mut self.unpacked_entries.push(EntryValue::Empty);
    }

    pub fn set(&mut self, index : &GenerationalIndex, value : T) {

        

        if index.index < self.entries.len() {
            //println!("Entry exists, placing value in index: {}", index.index());
            //println!("Setting value");
            self.entries[index.index()] = Some(ArrayEntry {value, generation : index.generation} );

        } else {
            //println!("pushing value");
            //println!("Placing value in index: {} and generation: {}", index.index, index.generation);
            self.entries.push(Some(ArrayEntry {value, generation : index.generation}));
        }

        if index.index < self.unpacked_entries.len() {

            self.unpacked_entries[index.index()] = EntryValue::Full(index.generation.clone());
        }
    }

    pub fn get(&self, index : &GenerationalIndex) -> Option<&T> {

        let mut value : Option<&T> = None;

        if self.contains(index) {

        let entry = self.entries[index.index()].as_ref().unwrap();

            if index.generation == entry.generation {
                value =  Some(&entry.value);
            }
        }
        value
    }

    pub fn contains(&self, index : &GenerationalIndex) -> bool{

        let mut success = false;

        if index.index() < self.unpacked_entries.len() {
            match &self.unpacked_entries[index.index()] {
                EntryValue::Full(gen) => if *gen == index.generation { success = true },
                EntryValue::Empty => success = false
            }
        }
        //println!("Success: {}", success);
        success
    }

    pub fn remove(&mut self, index : &GenerationalIndex) {

        self.entries[index.index()] = None;
        self.unpacked_entries[index.index()] = Empty;
    }

    pub fn get_mut(&mut self, index : &GenerationalIndex) -> Option<&mut T> {

        let mut value = None;

        if self.contains(index) {

            let entry = self.entries[index.index()].as_mut();

            if let Some(i) = entry {

                if index.generation == i.generation {
                    value = Some(&mut i.value);
                }
            }
        }
        value
    }

    pub fn get_non_empty_components(&self) -> Vec<&ArrayEntry<T>> {

        let mut relevant_values = Vec::new();

        for option in &self.entries {
            if option.is_some() {
                let new_val = option.clone();
                relevant_values.push(new_val.as_ref().unwrap());
            }
        }
        relevant_values
    }
}

/// Used to create the generational indices which will be stored in the generational index array.

pub struct AllocatorEntry {
    pub live : bool,
    generation : u64
}

pub struct GenerationalIndexAllocator {
    entries : Vec<AllocatorEntry>,
    free : Vec<usize>
}

impl GenerationalIndexAllocator {

    pub fn new() -> GenerationalIndexAllocator {

        let allocator = GenerationalIndexAllocator {
            entries: Vec::with_capacity(1024),
            free : Vec::with_capacity(1024)
        };

        allocator
    }

    pub fn allocate(&mut self) -> GenerationalIndex {

        let index : usize;
        let mut generation : u64 = 0;

        if !self.free.is_empty() {

            let free_index = self.free[0];

            let mut entry = &mut self.entries[free_index];

            entry.generation += 1;
            entry.live = true;

            index = self.free.pop().unwrap();
            generation = entry.generation;

        } else {

            self.entries.push(AllocatorEntry { live : true, generation : 0});
            index = self.entries.len() - 1;
        }

        //println!("Generated index with index: {}, and generation: {}", index, generation);
        GenerationalIndex {index, generation }
    }

    pub fn deallocate(&mut self, index : &GenerationalIndex) -> bool {

        self.free.push(index.index());
        self.entries[index.index].live = false;

        true
    }

    pub fn is_live(&self, index : &GenerationalIndex) -> bool {

        //println!("Index is live: {}", self.entries[index.index].live);
        self.entries[index.index].live
    }
}