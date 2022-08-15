pub struct Table<T> {
    values: Vec<T>,
    references: Vec<usize>,
    open_reference_indices: Vec<usize> 
}

impl<T> Table<T> {
    pub fn add(&mut self, value: T) -> usize {
        self.values.push(value);
        let return_index;
        if self.open_reference_indices.len() > 0 {
            return_index = *self.open_reference_indices.last().unwrap();
            self.references[return_index] = self.values.len() - 1;
            self.open_reference_indices.pop();
        }
        else {
            self.references.push(self.values.len() - 1);
            return_index = self.references.len() - 1;
        }

        return return_index;
    }

    pub fn remove(&mut self, ref_index: usize) {
        let index_to_value = self.references[ref_index];
        if index_to_value != self.values.len() - 1 {
            self.values[index_to_value] = *self.values.last().unwrap();
            for i in 0..self.references.len() - 1 {
                if self.references[i] == self.references.len() - 1 {
                    self.references[i] = index_to_value;
                }
            }
        }

        self.values.pop();
        self.open_reference_indices.push(ref_index);
    }

    pub fn at(&mut self, ref_index: usize) -> &mut T {
        return &mut self.values[self.references[ref_index]];
    }

    pub fn size(&self) -> usize {
        return self.values.len();
    }

    pub fn clear(&mut self) {
        self.values.clear();
        self.references.clear();
        self.open_reference_indices.clear();
    }
}


