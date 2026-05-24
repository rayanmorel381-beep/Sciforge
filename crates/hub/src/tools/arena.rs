//! Bump-allocator arena for temporary buffers: slices, matrices, and scratch pools.
//!
//! Provides [`Arena`], [`ArenaSlice`], [`ArenaMatrix`], and [`ScratchPool`]
//! to reuse memory without repeated dynamic allocation.

use std::cell::RefCell;

/// Contiguous `f64` bump allocator.
///
/// Pre-allocates a fixed-size vector and hands out slices
/// (`ArenaSlice`) by advancing an internal cursor.
pub struct Arena {
    buf: RefCell<Vec<f64>>,
    capacity: usize,
    offset: RefCell<usize>,
}

impl Arena {
    /// Creates an arena with room for `capacity` `f64` elements.
    pub fn new(capacity: usize) -> Self {
        Self {
            buf: RefCell::new(vec![0.0; capacity]),
            capacity,
            offset: RefCell::new(0),
        }
    }

    /// Allocates a slice of `n` elements, returns `None` if capacity is insufficient.
    pub fn alloc_slice(&self, n: usize) -> Option<ArenaSlice> {
        let mut off = self.offset.borrow_mut();
        if *off + n > self.capacity {
            return None;
        }
        let start = *off;
        *off += n;
        Some(ArenaSlice { start, len: n })
    }

    /// Writes `data` into the given slice (truncated if `data` exceeds the slice length).
    pub fn write(&self, slice: &ArenaSlice, data: &[f64]) {
        let mut buf = self.buf.borrow_mut();
        let end = slice.start + data.len().min(slice.len);
        buf[slice.start..end].copy_from_slice(&data[..end - slice.start]);
    }

    /// Reads the slice contents into a new `Vec<f64>`.
    pub fn read(&self, slice: &ArenaSlice) -> Vec<f64> {
        let buf = self.buf.borrow();
        buf[slice.start..slice.start + slice.len].to_vec()
    }

    /// Returns the element at `index` within the slice, or `None` if out of bounds.
    pub fn get(&self, slice: &ArenaSlice, index: usize) -> Option<f64> {
        if index >= slice.len {
            return None;
        }
        Some(self.buf.borrow()[slice.start + index])
    }

    /// Sets `value` at `index` within the slice.
    pub fn set(&self, slice: &ArenaSlice, index: usize, value: f64) {
        if index < slice.len {
            self.buf.borrow_mut()[slice.start + index] = value;
        }
    }

    /// Resets the cursor to zero, making all memory available for reuse.
    pub fn reset(&self) {
        *self.offset.borrow_mut() = 0;
    }

    /// Number of currently allocated elements.
    pub fn used(&self) -> usize {
        *self.offset.borrow()
    }

    /// Number of elements still available.
    pub fn remaining(&self) -> usize {
        self.capacity - self.used()
    }

    /// Total capacity of the arena.
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

/// Handle to a contiguous slice within the arena.
#[derive(Debug, Clone, Copy)]
pub struct ArenaSlice {
    start: usize,
    len: usize,
}

impl ArenaSlice {
    /// Length of the slice in number of elements.
    pub fn len(&self) -> usize {
        self.len
    }
    /// Returns `true` if the slice is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

/// 2D matrix stored contiguously within an arena.
pub struct ArenaMatrix {
    slice: ArenaSlice,
    /// Number of rows.
    pub rows: usize,
    /// Number of columns.
    pub cols: usize,
}

impl ArenaMatrix {
    /// Allocates a `rows × cols` matrix in `arena`.
    pub fn new(arena: &Arena, rows: usize, cols: usize) -> Option<Self> {
        let slice = arena.alloc_slice(rows * cols)?;
        Some(Self { slice, rows, cols })
    }

    /// Returns the element at `(r, c)`, or `None` if out of bounds.
    pub fn get(&self, arena: &Arena, r: usize, c: usize) -> Option<f64> {
        if r >= self.rows || c >= self.cols {
            return None;
        }
        arena.get(&self.slice, r * self.cols + c)
    }

    /// Sets `value` at `(r, c)`.
    pub fn set(&self, arena: &Arena, r: usize, c: usize, value: f64) {
        if r < self.rows && c < self.cols {
            arena.set(&self.slice, r * self.cols + c, value);
        }
    }

    /// Converts the matrix to `Vec<Vec<f64>>` (rows × columns).
    pub fn to_vec(&self, arena: &Arena) -> Vec<Vec<f64>> {
        let flat = arena.read(&self.slice);
        flat.chunks(self.cols).map(|row| row.to_vec()).collect()
    }
}

/// Pool of fixed-size scratch buffers stored in an arena.
pub struct ScratchPool {
    arena: Arena,
    scratch_size: usize,
    count: usize,
}

impl ScratchPool {
    /// Creates a pool of `count` buffers with `scratch_size` elements each.
    pub fn new(scratch_size: usize, count: usize) -> Self {
        Self {
            arena: Arena::new(scratch_size * count),
            scratch_size,
            count,
        }
    }

    /// Returns the slice for buffer `index`, or `None` if out of bounds.
    pub fn get(&self, index: usize) -> Option<ArenaSlice> {
        if index >= self.count {
            return None;
        }
        Some(ArenaSlice {
            start: index * self.scratch_size,
            len: self.scratch_size,
        })
    }

    /// Writes `data` into buffer `index`.
    pub fn write(&self, index: usize, data: &[f64]) {
        if let Some(slice) = self.get(index) {
            self.arena.write(&slice, data);
        }
    }

    /// Reads the contents of buffer `index`.
    pub fn read(&self, index: usize) -> Vec<f64> {
        self.get(index)
            .map(|s| self.arena.read(&s))
            .unwrap_or_default()
    }

    /// Size of each scratch buffer.
    pub fn scratch_size(&self) -> usize {
        self.scratch_size
    }
    /// Number of buffers in the pool.
    pub fn count(&self) -> usize {
        self.count
    }
    /// Reference to the underlying arena.
    pub fn arena(&self) -> &Arena {
        &self.arena
    }
}
