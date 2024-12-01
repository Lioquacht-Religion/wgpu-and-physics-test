//vector with generational indices
//


pub struct Key{
    index: usize,
    generation: usize,
}

pub struct GenVec<T>{
    data: Vec<GenEntry<T>>,
    free_head: usize,
    len: usize,
}

struct GenEntry<T>{
    value: Entry<T>,
    generation: usize,
}

enum Entry<T>{
    Free{next_free: usize},
    Occupied{value: T}
}

impl<T> GenVec<T> {
    pub fn new() -> GenVec<T>{
        Self{
            data: Vec::new(),
            free_head: 0,
            len: 0,
        }
    }

    pub fn insert(&mut self, to_insert: T) -> Key{
        let key = if let Some(GenEntry{value: entry, generation }) = self.data.get_mut(self.free_head){
            if let Entry::Free { next_free } = entry{
                let index = self.free_head;
                let generation = *generation+1;
                self.free_head = *next_free;
                Key{
                    index,
                    generation,
                }
            }
            else{
                panic!("corrupted list; free head should not be occupied")
            }

        }
        else {
            self.data.push(
                GenEntry {
                    value: Entry::Occupied { value: to_insert },
                    generation: 0
                }
            );
            self.free_head += self.data.len()+1;
            Key{
                index: self.data.len(),
                generation: 0,
            }
        };
        self.len += 1;

        key
    }

    pub fn get(&self, key: &Key) -> Option<&T>{
        if let Some(GenEntry { value, generation }) = self.data.get(key.index){
            if let Entry::Occupied { value } = value{
                if *generation==key.generation{
                    return Some(value);
                }
            }
        }
        None
    }

    pub fn get_mut(&mut self, key: &Key) -> Option<&mut T>{
        if let Some(GenEntry { value, generation }) = self.data.get_mut(key.index){
            if let Entry::Occupied { value } = value{
                if *generation==key.generation{
                    return Some(value);
                }
            }
        }
        None
    }



    pub fn remove(&mut self, key: &Key){
        let GenEntry { value, generation } = &self.data[key.index];
        if let Entry::Occupied { .. } = value{
            if *generation!=key.generation{
                return;
            }

            self.data[key.index] =
                GenEntry{
                    value: Entry::Free { next_free: self.free_head },
                    generation : generation+1,
                };
            self.free_head = key.index;
            self.len -= 1;
        }
    }

    pub fn len(&self) -> usize{
        self.data.len()
    }

    pub fn iter(&self) -> impl Iterator<Item=&T>{
        self.data.iter().filter_map(|e| {
                if let Entry::Occupied { value } = &e.value{
                    return Some(value)
                }
            None
        } )
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut T>{
        self.data.iter_mut().filter_map(|e| {
                if let Entry::Occupied { value } = &mut e.value{
                    return Some(value)
                }
            None
        } )
    }




}


