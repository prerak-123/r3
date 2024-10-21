mod incremental_id;

use incremental_id::IncremantalID;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct IDAllocator<'a, K, Id>
where
    K: Hash + Eq + ?Sized,
    Id: Copy + IncremantalID,
{
    id_table: HashMap<&'a K, Id>,
    next_id: Id,
}

impl<'a, K, Id> IDAllocator<'a, K, Id>
where
    K: Hash + Eq + ?Sized,
    Id: Copy + IncremantalID,
{
    pub fn new() -> Self {
        IDAllocator {
            id_table: HashMap::new(),
            next_id: Id::init(),
        }
    }

    pub fn allocate(&mut self, val: &'a K) -> Id {
        match self.id_table.get(val) {
            Some(id) => id.clone(),
            None => {
                let new_id = self.next_id.get_and_increment();
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
        let _: IDAllocator<'_, K, Id> = IDAllocator::new();
    }

    #[test]
    fn test_allocate() {
        let name1 = String::from("hello");
        let name2 = String::from("world!");
        let name1_copy = String::from("hello");

        let mut allocator: IDAllocator<'_, K, Id> = IDAllocator::new();

        let id1 = allocator.allocate(name1.as_str());
        let id2 = allocator.allocate(name2.as_str());
        assert_ne!(id1, id2);

        let id1_copy = allocator.allocate(name1_copy.as_str());
        assert_eq!(id1, id1_copy);
    }
}
