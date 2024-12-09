struct Memory {
    inner_mem: Vec<Option<u32>>,
}

#[derive(Clone, Debug)]
struct Chunk {
    size: u8,
    f_id: Option<u64>,
}

struct ChunkedMemory {
    memory: Vec<Chunk>,
}

impl From<String> for Memory {
    fn from(value: String) -> Self {
        let filtered = value.replace("\n", "");
        let chars: Vec<char> = filtered.chars().filter(|c| *c != ' ').collect();
        let mut mem: Vec<Option<u32>> = vec![];
        let mut f_id: u32 = 0;
        for (i, c) in chars.iter().enumerate() {
            let num: u8 = (*c as u8) - ('0' as u8);
            let is_free = i % 2 == 1;
            for _ in 0..num {
                if is_free {
                    mem.push(None);
                } else {
                    mem.push(Some(f_id))
                }
            }
            if !is_free {
                f_id += 1;
            }
        }

        Memory { inner_mem: mem }
    }
}

impl From<String> for ChunkedMemory {
    fn from(value: String) -> Self {
        let filtered = value.replace("\n", "");
        let chars: Vec<char> = filtered.chars().filter(|c| *c != ' ').collect();
        let mut mem: Vec<Chunk> = vec![];
        let mut f_id: u64 = 0;
        for (i, c) in chars.iter().enumerate() {
            let num: u8 = (*c as u8) - ('0' as u8);
            let is_free = i % 2 == 1;

            if is_free {
                mem.push(Chunk {
                    size: num,
                    f_id: None,
                });
            } else {
                mem.push(Chunk {
                    size: num,
                    f_id: Some(f_id),
                });
                f_id += 1;
            }
        }

        ChunkedMemory { memory: mem }
    }
}

impl Memory {
    pub fn task1(&mut self) -> u64 {
        self.sort_memory();
        self.calculate_check_sum()
    }

    fn calculate_check_sum(&mut self) -> u64 {
        let mut sum: u64 = 0;
        for (i, chunk) in self.inner_mem.iter().enumerate() {
            if let Some(val) = chunk {
                sum += i as u64 * (*val) as u64;
            }
        }
        sum
    }

    fn sort_memory(&mut self) {
        for i in 0..self.inner_mem.len() {
            let chunk = self.inner_mem.get(i).unwrap().clone();
            if chunk.is_none() {
                let last_chunk_index = self.get_index_of_last_filled_chunk();
                if last_chunk_index > i {
                    let last_chunk = self.inner_mem.get(last_chunk_index).unwrap().clone();
                    let chunk_mut = self.inner_mem.get_mut(i).unwrap();
                    *chunk_mut = last_chunk;
                    let last_chunk_mut = self.inner_mem.get_mut(last_chunk_index).unwrap();
                    *last_chunk_mut = None;
                } else {
                    break;
                }
            }
        }
    }

    fn get_index_of_last_filled_chunk(&mut self) -> usize {
        for i in (0..self.inner_mem.len()).rev() {
            if self.inner_mem.get(i).unwrap().is_some() {
                return i;
            }
        }
        0
    }
}

impl ChunkedMemory {
    pub fn task2(&mut self) -> u64 {
        self.sort_memory();
        self.calculate_check_sum()
    }

    fn calculate_check_sum(&mut self) -> u64 {
        let mut sum: u64 = 0;
        let mut current_index = 0;
        for chunk in self.memory.iter() {
            if let Some(f_id) = chunk.f_id {
                for _ in 0..chunk.size {
                    sum += current_index * f_id;
                    current_index += 1;
                }
            } else {
                current_index += chunk.size as u64;
            }
        }
        sum
    }

    fn sort_memory(&mut self) {
        while !self.is_memory_sorted() {
            for i in (0..self.memory.len()).rev() {
                let chunk = self.memory.get(i).unwrap().clone();
                if chunk.f_id.is_some() {
                    if let Some(mem_addr) = self.has_free_chunk_with_size(chunk.size, i) {
                        let free_chunk = self.memory.get_mut(mem_addr).unwrap();
                        let left_over_size = free_chunk.size - chunk.size;

                        *free_chunk = chunk.clone();

                        let mut index_inc: usize = 0;

                        if left_over_size > 0 {
                            let right_chunk = self.memory.get_mut(mem_addr + 1).unwrap();
                            if right_chunk.f_id.is_none() {
                                right_chunk.size += left_over_size;
                            } else {
                                self.memory.insert(
                                    mem_addr + 1,
                                    Chunk {
                                        size: left_over_size,
                                        f_id: None,
                                    },
                                );
                                index_inc = 1;
                            }
                        }
                        /*if i + 1 >= self.memory.len() {
                            let new_free = self.memory.get_mut(i).unwrap();
                            *new_free = Chunk {
                                size: chunk.size,
                                f_id: None,
                            };
                            break;
                        }

                        let right_to = self.memory.get_mut(i + 1).unwrap();
                        if right_to.f_id.is_none() {
                            right_to.size += chunk.size;
                            break;
                        }

                        if i == 0 {
                            let new_free = self.memory.get_mut(i).unwrap();
                            *new_free = Chunk {
                                size: chunk.size,
                                f_id: None,
                            };
                            break;
                        }

                        let left_to = self.memory.get_mut(i - 1).unwrap();
                        if left_to.f_id.is_none() {
                            left_to.size += chunk.size;
                            break;
                        }*/

                        let new_free = self.memory.get_mut(i + index_inc).unwrap();
                        *new_free = Chunk {
                            size: chunk.size,
                            f_id: None,
                        };

                        break;
                    }
                }
            }
        }
    }

    fn is_memory_sorted(&mut self) -> bool {
        for (i, chunk) in self.memory.clone().iter().enumerate().rev() {
            if chunk.f_id.is_some() && self.has_free_chunk_with_size(chunk.size, i).is_some() {
                return false;
            }
        }
        true
    }

    fn has_free_chunk_with_size(&mut self, size: u8, until: usize) -> Option<usize> {
        for i in 0..self.memory.len() {
            if i >= until {
                return None;
            }
            let val = self.memory.get(i).unwrap();
            if val.size >= size && val.f_id == None {
                return Some(i);
            }
        }
        return None;
    }
}

pub fn run(contents: String) {
    let mut memory = Memory::from(contents.clone());
    println!("Task 1: {}", memory.task1());

    let mut chunked_memory = ChunkedMemory::from(contents);
    println!("Task 2: {}", chunked_memory.task2());
}
