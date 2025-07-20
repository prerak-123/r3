mod incremental_id_generator;

use incremental_id_generator::IncremantalIDGenerator;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
enum AllocatedID<T> {
    New(T),
    Repeat(T),
}

impl<T> AllocatedID<T> {
    pub fn is_new(&self) -> bool {
        match self {
            Self::New(_) => true,
            Self::Repeat(_) => false,
        }
    }

    pub fn is_repeat(&self) -> bool {
        !self.is_new()
    }
}

impl<T> AllocatedID<T>
where
    T: Copy,
{
    pub fn id(&self) -> T {
        match self {
            Self::New(id) => *id,
            Self::Repeat(id) => *id,
        }
    }
}

#[derive(Debug)]
pub struct IDAllocator<K, G>
where
    K: Hash + Eq,
    G: IncremantalIDGenerator,
{
    id_table: HashMap<K, G::ID>,
    generator: G,
}

impl<K, G> IDAllocator<K, G>
where
    K: Hash + Eq,
    G: Copy + IncremantalIDGenerator,
{
    pub fn new() -> Self {
        IDAllocator {
            id_table: HashMap::new(),
            generator: G::init(),
        }
    }

    pub fn num_allocated(&self) -> usize {
        self.id_table.len()
    }

    pub fn allocate(&mut self, val: K) -> AllocatedID<G::ID> {
        match self.id_table.get(&val) {
            Some(id) => AllocatedID::Repeat(*id),
            None => {
                let new_id = self.generator.get_and_increment();
                self.id_table.insert(val, new_id);
                AllocatedID::New(new_id)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type K = str;
    type Id = u32;

    #[test]
    fn test_create() {
        let _: IDAllocator<&K, Id> = IDAllocator::new();
    }

    #[test]
    fn test_allocate_rettype() {
        let name1 = String::from("hello");
        let name2 = String::from("world!");
        let name1_copy = name1.clone();

        let mut allocator: IDAllocator<&K, Id> = IDAllocator::new();

        let is_id1_new = allocator.allocate(name1.as_str()).is_new();
        assert!(is_id1_new);

        let is_id2_new = allocator.allocate(name2.as_str()).is_new();
        assert!(is_id2_new);

        let is_id1_copy_repeat = allocator.allocate(name1_copy.as_str()).is_repeat();
        assert!(is_id1_copy_repeat);
    }

    #[test]
    fn test_allocate_uniqueid() {
        let name1 = String::from("hello");
        let name2 = String::from("world!");
        let name1_copy = name1.clone();

        let mut allocator: IDAllocator<&K, Id> = IDAllocator::new();

        let id1 = allocator.allocate(name1.as_str()).id();
        let id2 = allocator.allocate(name2.as_str()).id();
        assert_ne!(id1, id2);

        let id1_copy = allocator.allocate(name1_copy.as_str()).id();
        assert_eq!(id1, id1_copy);
    }
}
