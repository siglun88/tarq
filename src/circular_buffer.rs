// File: circular_buffer.rs

use std::collections::VecDeque;

pub struct CircularBuffer<T> {
    buffer: VecDeque<T>,
    capacity: usize,
}

impl<T> CircularBuffer<T> {
    /// Creates a new CircularBuffer with the specified capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    /// Adds a new element to the circular buffer. If the buffer is full,
    /// the oldest element is removed to make room for the new one.
    pub fn push(&mut self, value: T) {
        if self.buffer.len() == self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(value);
    }

    /// Returns a contiguous slice of the buffer's first segment.
    /// Note that VecDeque may internally wrap around, so this returns the
    /// first contiguous segment only.
    pub fn as_slice(&self) -> &[T] {
        self.buffer.as_slices().0
    }

    /// Returns `true` if the buffer is full.
    pub fn is_full(&self) -> bool {
        self.buffer.len() == self.capacity
    }

    /// Returns a reference to the oldest element in the buffer.
    pub fn front(&self) -> Option<&T> {
        self.buffer.front()
    }
}
