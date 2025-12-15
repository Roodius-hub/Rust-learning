#[derive(Debug)]
struct MinHeap {
    data: Vec<i32>,
}

impl MinHeap {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    // Insert (push)
    fn push(&mut self, value: i32) {
        self.data.push(value);
        self.bubble_up(self.data.len() - 1);
    }

    // Remove smallest (pop)
    fn pop(&mut self) -> Option<i32> {
        if self.data.is_empty() {
            return None;
        }

        let last_index = self.data.len() - 1;
        self.data.swap(0, last_index);
        let min_value = self.data.pop();

        self.bubble_down(0);

        min_value
    }

    fn bubble_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let parent = (idx - 1) / 2;
            if self.data[idx] < self.data[parent] {
                self.data.swap(idx, parent);
                idx = parent;
            } else {
                break;
            }
        }
    }

    fn bubble_down(&mut self, mut idx: usize) {
        let len = self.data.len();

        loop {
            let left = 2 * idx + 1;
            let right = 2 * idx + 2;
            let mut smallest = idx;

            if left < len && self.data[left] < self.data[smallest] {
                smallest = left;
            }
            if right < len && self.data[right] < self.data[smallest] {
                smallest = right;
            }

            if smallest != idx {
                self.data.swap(idx, smallest);
                idx = smallest;
            } else {
                break;
            }
        }
    }
}

//---------------MAX HEAP --------------------------
#[derive(Debug)]
struct MaxHeap {
    data: Vec<i32>,
}

impl MaxHeap {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn push(&mut self, value: i32) {
        self.data.push(value);
        self.bubble_up(self.data.len() - 1);
    }

    fn pop(&mut self) -> Option<i32> {
        if self.data.is_empty() {
            return None;
        }

        let last = self.data.len() - 1;
        self.data.swap(0, last);
        let max = self.data.pop();

        self.bubble_down(0);

        max
    }

    fn bubble_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let parent = (idx - 1) / 2;

            if self.data[idx] > self.data[parent] {  // only change
                self.data.swap(idx, parent);
                idx = parent;
            } else {
                break;
            }
        }
    }

    fn bubble_down(&mut self, mut idx: usize) {
        let len = self.data.len();

        loop {
            let left = 2 * idx + 1;
            let right = 2 * idx + 2;
            let mut largest = idx;

            if left < len && self.data[left] > self.data[largest] {
                largest = left;
            }
            if right < len && self.data[right] > self.data[largest] {
                largest = right;
            }

            if largest != idx {
                self.data.swap(idx, largest);
                idx = largest;
            } else {
                break;
            }
        }
    }
}


fn main(){
    let min_heap = MinHeap::new();
}