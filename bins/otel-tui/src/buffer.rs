#[derive(Debug)]
pub(crate) struct RingBuffer<T> {
    elements: Vec<T>,
    capacity: usize,
    index: usize,
}

impl<T> RingBuffer<T> {
    pub(crate) fn new(capacity: usize) -> Self {
        Self {
            elements: Vec::with_capacity(capacity),
            capacity,
            index: 0,
        }
    }

    pub(crate) fn push(&mut self, element: T) {
        if self.elements.len() < self.capacity {
            self.elements.push(element);
            self.index += 1;
        } else {
            self.index = 0;
            self.elements[self.index] = element;
        }
    }

    pub(crate) fn iter(&self) {
        if self.elements.len() <= self.capacity {
            // self.elements.iter().c
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.elements.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut buffer = RingBuffer::<u8>::new(2);
        assert_eq!(buffer.len(), 0);

        buffer.push(1);
        buffer.push(2);
        buffer.push(4);
    }
}
