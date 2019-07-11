use std::borrow::BorrowMut;

pub struct GenerationalIndex {
    index : usize,
    generation: u64
}

impl GenerationalIndex {

    pub fn index(&self) -> usize {
        self.index
    }
}

struct ArrayEntry<T> {
    value : T,
    generation : u64
}

pub struct GenerationalIndexArray<T>(Vec<Option<ArrayEntry<T>>>);

impl<T> GenerationalIndexArray<T> {

    pub fn new() -> GenerationalIndexArray<T> {

        let array = GenerationalIndexArray {
            0: Vec::new()
        };

        array
    }

    pub fn set(&mut self, index : GenerationalIndex, value : T) {


    }

    pub fn get(&self, index : GenerationalIndex) -> Option<&T> {
        None
    }

    pub fn get_mut(&mut self, index : GenerationalIndex) -> Option<&mut T> {
        None
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
            entries: Vec::new(),
            free : Vec::new()
        };

        allocator
    }

    pub fn allocate(&mut self) -> GenerationalIndex {

        let mut index : usize = 0;
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