struct RingBuffer<T>{
    buf: Vec<Option<T>>,
    size: usize,
    write_index: usize,
    read_index: usize,
}

impl<T:Clone> RingBuffer<T>{
    fn new(size: usize) -> Self {
        RingBuffer {
            buf: vec![None; size],
            size,
            write_index: 0,
            read_index: 0,
        }
    }

    fn put(&mut self, item: T) {
        if self.is_full() {
            panic!("ringbuffer is full");
        }

        self.buf[self.write_index] = Some(item);
        self.write_index = (self.write_index + 1) % self.size;
    }

    fn get(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let value = self.buf[self.read_index].take();
        self.read_index = (self.read_index + 1) % self.size;
        value
    }

    fn is_full(&self) -> bool {
        (self.write_index + 1) % self.size == self.read_index
    }

    fn is_empty(&self) -> bool {
        self.write_index == self.read_index && self.buf[self.read_index].is_none()
    }
}

fn main() {
    let mut rb = RingBuffer::<usize>::new(10);

    for i in 1..=9 {
        rb.put(i);
    }

    println!("rb content is: ");

    while let Some(value) = rb.get() {
        println!("{}", value);
    }
}
