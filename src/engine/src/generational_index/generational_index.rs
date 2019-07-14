pub struct GenerationalIndex {
    pub index : usize,
    pub generation: u64
}

impl GenerationalIndex {

    pub fn index(&self) -> usize {
        self.index
    }
}

pub struct ArrayEntry<T> {
    pub value : T,
    pub generation : u64
}

pub struct GenerationalIndexArray<T> {

    pub entries : Vec<Option<ArrayEntry<T>>>,
}

impl<T> GenerationalIndexArray<T> {

    pub fn new() -> GenerationalIndexArray<T> {

        let array = GenerationalIndexArray {
            entries: Vec::with_capacity(1024)
        };

        array
    }

    pub fn set_empty(&mut self) {

        &mut self.entries.push(None);
    }

    pub fn set(&mut self, index : &GenerationalIndex, value : T) {

        if index.index < self.entries.len() {

            let mut entry = self.entries[index.index()].as_mut();

            entry = Some(ArrayEntry {value, generation : index.generation} ).as_mut();

        } else {

            //println!("Placing value in index: {} and generation: {}", index.index, index.generation);
            self.entries.push(Some(ArrayEntry {value, generation : index.generation}));
        }
    }

    pub fn get(&self, index : &GenerationalIndex) -> Option<&T> {

        let entry = self.entries[index.index()].as_ref();

        if let Some(i) = entry {

            if index.generation == i.generation {
                return Some(&i.value);
            }
        }
            None
    }

    pub fn get_mut(&mut self, index : &GenerationalIndex) -> Option<&mut T> {

        let entry = self.entries[index.index()].as_mut();

        if let Some(i) = entry {

            if index.generation == i.generation {
                return Some(&mut i.value);
            }
        }
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