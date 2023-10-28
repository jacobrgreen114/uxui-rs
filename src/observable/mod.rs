struct ObservableVec<T> {
    vec: Vec<T>,
}

impl<T> ObservableVec<T> {
    pub fn new() -> Self {
        Self { vec: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.vec.push(item);
    }

    pub fn remove(&mut self, index: usize) -> T {
        self.vec.remove(index)
    }
}
