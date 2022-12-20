use std::ops::Index;

pub trait Commands<T, C>: From<T> + Into<T> + Clone + Index<usize, Output = C> {
    fn create_from(value: &T) -> Self;
    fn create(&self) -> T where T: Default {
        let mut result = T::default();
        self.update(&mut result);
        result
    }
    fn update(&self, target: &mut T);
    fn add(&mut self, command: C) -> &mut Self;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn combine_last(&mut self) -> &mut Self;
    fn truncate(&mut self, index: usize) -> &mut Self;
    fn _replace_last_two_with(&mut self, command: C) {
        self.truncate(self.len()-2);
        self.add(command);
    }    
}