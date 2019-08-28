use crate::generational_index::generational_index::EntryValue::Empty;

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
    // Generation   // PackedIndex  //EntityIndex
    Full((u64,      usize,          usize)),
    Empty
}

// TODO: FIND WAY TO SORT COMPONENTS INSTEAD OF STORING GENERATIONAL INDEX FOR EACH ONE.
#[derive(Clone, Copy)]
pub struct ArrayEntry<T> {
    pub value : T,
    pub owned_entity: GenerationalIndex
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

        //println!("Setting empty...");
        &mut self.unpacked_entries.push(EntryValue::Empty);
    }

    pub fn set(&mut self, index : &GenerationalIndex, value : T) {

        let saved_index : (u64, usize, usize);

        if let Some(idx) = self.get_unpacked_index(index) {

            self.entries[idx.1] = Some(ArrayEntry {value, owned_entity: index.clone()} );
            saved_index = idx;

        } else {
            //println!("Pushing back value at index: {}", index.index);
            self.entries.push(Some(ArrayEntry {value, owned_entity: index.clone()}));
            //println!("length: {}", self.entries.len());
            saved_index = ( index.generation, self.entries.len() - 1, index.index.clone());
        }

        if index.index < self.unpacked_entries.len() {
            //println!("Adding to unpacked...");
            self.unpacked_entries[index.index] = EntryValue::Full((index.generation.clone(), saved_index.1, index.index.clone()))
        }
    }

    pub fn get_unpacked_index(&self, index : &GenerationalIndex) -> Option<(u64, usize, usize)>{

        let mut success = None;

        if index.index() < self.unpacked_entries.len() {
            match self.unpacked_entries[index.index()] {
                EntryValue::Full(v) => { success = Some(v) },
                EntryValue::Empty => ()
            }
        }
        success
    }

    pub fn get(&self, index : &GenerationalIndex) -> Option<&T> {

        let mut value : Option<&T> = None;

        if self.contains(index) {

            //println!("fetching for index: {} generation: {}", index.index, index.generation);

            //println!("{}", index.index);

            let entry = self.entries[self.get_unpacked_index(index).unwrap().1].as_ref().unwrap();

                if index.generation == entry.owned_entity.generation {
                    value = Some(&entry.value);
                }
            }
        value
    }

    pub fn contains(&self, index : &GenerationalIndex) -> bool {

        let mut success = false;

        if index.index() < self.unpacked_entries.len() {
            match &self.unpacked_entries[index.index()] {
                EntryValue::Full(gen) => if gen.0 == index.generation { success = true },
                EntryValue::Empty => success = false
            }
        }
        success
    }

    pub fn remove(&mut self, index : &GenerationalIndex) {

        if self.contains(index) {
            let unpacked_index = self.get_unpacked_index(index).unwrap().1;
            self.entries.remove(unpacked_index);
            self.unpacked_entries[index.index()] = Empty;
            self.update_entries();
        }
    }

    pub fn update_entries(&mut self) {

        let mut idx : usize = 0;

        let mut coordinates : Vec<(GenerationalIndex, usize)> = Vec::new();

        self.entries.iter().for_each(|entry| {

            coordinates.push((entry.as_ref().unwrap().owned_entity.clone(), idx));
            idx+= 1;

        });

        coordinates.iter().for_each(|coordinate| {
           let entry = &self.unpacked_entries[coordinate.0.index];
              match entry {
                  EntryValue::Full(mut coord) => {
                      coord.1 = coordinate.1;
                  },
                  EntryValue::Empty => ()
           };
        });
    }

    pub fn get_mut(&mut self, index : &GenerationalIndex) -> Option<&mut T> {

        let mut value = None;

        if self.contains(index) {

            let idx = self.get_unpacked_index(index).as_ref().unwrap().1;
            let entry = &mut self.entries[idx];

                if let Some(val) = entry.as_mut() {
                    if index.generation == val.owned_entity.generation {
                        value = Some(&mut val.value);
                    }
                }
            }
        value
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