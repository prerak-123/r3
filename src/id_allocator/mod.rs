mod incremental_id_generator;

use incremental_id_generator::IncremantalIDGenerator;
use std::collections::HashMap;
use std::hash::Hash;

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

    pub fn allocate(&mut self, val: K) -> G::ID {
        match self.id_table.get(&val) {
            Some(id) => *id,
            None => {
                let new_id = self.generator.get_and_increment();
                self.id_table.insert(val, new_id);
                new_id
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
    fn test_allocate() {
        let name1 = String::from("hello");
        let name2 = String::from("world!");
        let name1_copy = name1.clone();

        let mut allocator: IDAllocator<&K, Id> = IDAllocator::new();

        let id1 = allocator.allocate(name1.as_str());
        let id2 = allocator.allocate(name2.as_str());
        assert_ne!(id1, id2);

        let id1_copy = allocator.allocate(name1_copy.as_str());
        assert_eq!(id1, id1_copy);
    }
}
