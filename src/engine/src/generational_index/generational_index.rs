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
            0: vec![]
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

pub struct AllocatorEntry {

    live : bool,
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

    pub fn allocate(&mut self) -> AllocatorEntry {

        let entry = AllocatorEntry {
            live: false,
            generation : 0
        };

        entry

    }

    pub fn deallocate(&mut self, index : GenerationalIndex) -> bool {

        false
    }

    pub fn is_live(&self, index : GenerationalIndex) -> bool {

        false
    }
}