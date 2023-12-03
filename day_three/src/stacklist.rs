struct StackList<T, const N: usize> {
    data: [T; N],
    count: usize,
}

impl<T, const N: usize> StackList<T, N> {
    fn new() -> Self {
        StackList {
            data: [Default::default(); N],
            count: 0,
        }
    }

    fn push(&mut self, element: T) {
        if self.count < N {
            self.data[self.count] = element;
            self.count += 1;
        } else {
            // Handle the case where the list is full
            println!("Error: List is full, cannot push more elements.");
        }
    }

    fn len(&self) -> usize {
        self.count
    }

    fn elements(&self) -> &[T; N] {
        &self.data
    }
}