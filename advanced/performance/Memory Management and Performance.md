# Memory Management and Performance in Rust

## Overview

Rust's memory management system provides both safety and performance. This guide covers advanced memory management techniques, performance optimization strategies, and low-level memory operations in Rust.

---

## Memory Layout and Allocation

### Stack vs Heap Allocation

```rust
use std::alloc::{alloc, dealloc, Layout};

pub struct MemoryProfiler {
    stack_allocations: Vec<usize>,
    heap_allocations: Vec<usize>,
    total_memory: usize,
}

impl MemoryProfiler {
    pub fn new() -> Self {
        MemoryProfiler {
            stack_allocations: Vec::new(),
            heap_allocations: Vec::new(),
            total_memory: 0,
        }
    }
    
    pub fn profile_stack_allocation<T>(&mut self, value: T) -> T {
        let size = std::mem::size_of::<T>();
        self.stack_allocations.push(size);
        self.total_memory += size;
        
        println!("Stack allocation: {} bytes", size);
        value
    }
    
    pub fn profile_heap_allocation<T>(&mut self, value: T) -> Box<T> {
        let size = std::mem::size_of::<T>();
        self.heap_allocations.push(size);
        self.total_memory += size;
        
        println!("Heap allocation: {} bytes", size);
        Box::new(value)
    }
    
    pub fn get_memory_stats(&self) -> MemoryStats {
        MemoryStats {
            stack_total: self.stack_allocations.iter().sum(),
            heap_total: self.heap_allocations.iter().sum(),
            stack_count: self.stack_allocations.len(),
            heap_count: self.heap_allocations.len(),
            total_memory: self.total_memory,
        }
    }
}

#[derive(Debug)]
pub struct MemoryStats {
    pub stack_total: usize,
    pub heap_total: usize,
    pub stack_count: usize,
    pub heap_count: usize,
    pub total_memory: usize,
}

// Custom allocator
pub struct CustomAllocator {
    allocated_blocks: HashMap<*mut u8, (usize, Layout)>,
    total_allocated: usize,
}

impl CustomAllocator {
    pub fn new() -> Self {
        CustomAllocator {
            allocated_blocks: HashMap::new(),
            total_allocated: 0,
        }
    }
    
    pub fn allocate(&mut self, size: usize) -> *mut u8 {
        let layout = Layout::from_size_align(size, 8)
            .expect("Invalid layout");
        
        unsafe {
            let ptr = alloc(layout);
            if !ptr.is_null() {
                self.allocated_blocks.insert(ptr, (size, layout));
                self.total_allocated += size;
                println!("Custom allocator: allocated {} bytes at {:?}", size, ptr);
            }
            ptr
        }
    }
    
    pub fn deallocate(&mut self, ptr: *mut u8) {
        unsafe {
            if let Some((size, layout)) = self.allocated_blocks.remove(&ptr) {
                dealloc(ptr, layout);
                self.total_allocated -= size;
                println!("Custom allocator: deallocated {} bytes at {:?}", size, ptr);
            }
        }
    }
    
    pub fn get_allocated_bytes(&self) -> usize {
        self.total_allocated
    }
}

impl Drop for CustomAllocator {
    fn drop(&mut self) {
        println!("Custom allocator dropped with {} bytes still allocated", self.total_allocated);
    }
}
```

### Memory Pool Management

```rust
use std::sync::Mutex;

pub struct MemoryPool<T> {
    pool: Vec<T>,
    available: Vec<usize>,
    next_index: usize,
    capacity: usize,
}

impl<T: Default + Clone> MemoryPool<T> {
    pub fn new(capacity: usize) -> Self {
        let pool = (0..capacity).map(|_| T::default()).collect();
        let available = (0..capacity).collect();
        
        MemoryPool {
            pool,
            available,
            next_index: 0,
            capacity,
        }
    }
    
    pub fn allocate(&mut self) -> Option<usize> {
        if let Some(index) = self.available.pop() {
            Some(index)
        } else if self.next_index < self.capacity {
            let index = self.next_index;
            self.next_index += 1;
            Some(index)
        } else {
            None
        }
    }
    
    pub fn deallocate(&mut self, index: usize) {
        if index < self.capacity && !self.available.contains(&index) {
            self.available.push(index);
        }
    }
    
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.capacity {
            Some(&self.pool[index])
        } else {
            None
        }
    }
    
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.capacity {
            Some(&mut self.pool[index])
        } else {
            None
        }
    }
    
    pub fn available_count(&self) -> usize {
        self.available.len()
    }
    
    pub fn used_count(&self) -> usize {
        self.capacity - self.available.len()
    }
}

// Thread-safe memory pool
pub struct ThreadSafeMemoryPool<T> {
    pool: Mutex<MemoryPool<T>>,
}

impl<T: Default + Clone> ThreadSafeMemoryPool<T> {
    pub fn new(capacity: usize) -> Self {
        ThreadSafeMemoryPool {
            pool: Mutex::new(MemoryPool::new(capacity)),
        }
    }
    
    pub fn allocate(&self) -> Option<usize> {
        self.pool.lock().unwrap().allocate()
    }
    
    pub fn deallocate(&self, index: usize) {
        self.pool.lock().unwrap().deallocate(index);
    }
    
    pub fn get(&self, index: usize) -> Option<T> {
        self.pool.lock().unwrap().get(index).cloned()
    }
    
    pub fn set(&self, index: usize, value: T) -> bool {
        if let Some(slot) = self.pool.lock().unwrap().get_mut(index) {
            *slot = value;
            true
        } else {
            false
        }
    }
}
```

---

## Performance Optimization

### Cache-Friendly Data Structures

```rust
use std::mem;

pub struct CacheFriendlyArray<T> {
    data: Vec<T>,
    stride: usize,
}

impl<T> CacheFriendlyArray<T> {
    pub fn new(capacity: usize) -> Self {
        CacheFriendlyArray {
            data: Vec::with_capacity(capacity),
            stride: 1,
        }
    }
    
    pub fn with_stride(capacity: usize, stride: usize) -> Self {
        CacheFriendlyArray {
            data: Vec::with_capacity(capacity * stride),
            stride,
        }
    }
    
    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }
    
    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index * self.stride)
    }
    
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.data.get_mut(index * self.stride)
    }
    
    pub fn len(&self) -> usize {
        self.data.len() / self.stride
    }
    
    pub fn memory_usage(&self) -> usize {
        self.data.len() * mem::size_of::<T>()
    }
}

// SOA (Structure of Arrays) vs AOS (Array of Structures)
#[derive(Debug, Clone)]
pub struct ParticleAOS {
    pub position: [f32; 3],
    pub velocity: [f32; 3],
    pub mass: f32,
    pub radius: f32,
}

pub struct ParticleSOA {
    pub positions: Vec<[f32; 3]>,
    pub velocities: Vec<[f32; 3]>,
    pub masses: Vec<f32>,
    pub radii: Vec<f32>,
}

impl ParticleSOA {
    pub fn new(capacity: usize) -> Self {
        ParticleSOA {
            positions: Vec::with_capacity(capacity),
            velocities: Vec::with_capacity(capacity),
            masses: Vec::with_capacity(capacity),
            radii: Vec::with_capacity(capacity),
        }
    }
    
    pub fn add_particle(&mut self, position: [f32; 3], velocity: [f32; 3], mass: f32, radius: f32) {
        self.positions.push(position);
        self.velocities.push(velocity);
        self.masses.push(mass);
        self.radii.push(radius);
    }
    
    pub fn update_positions(&mut self, dt: f32) {
        for (pos, vel) in self.positions.iter_mut().zip(self.velocities.iter()) {
            pos[0] += vel[0] * dt;
            pos[1] += vel[1] * dt;
            pos[2] += vel[2] * dt;
        }
    }
    
    pub fn memory_usage(&self) -> usize {
        (self.positions.len() * mem::size_of::<[f32; 3]>() +
         self.velocities.len() * mem::size_of::<[f32; 3]>() +
         self.masses.len() * mem::size_of::<f32>() +
         self.radii.len() * mem::size_of::<f32>())
    }
}

// Performance benchmarking
pub struct PerformanceBenchmark {
    name: String,
    iterations: usize,
}

impl PerformanceBenchmark {
    pub fn new(name: String, iterations: usize) -> Self {
        PerformanceBenchmark {
            name,
            iterations,
        }
    }
    
    pub fn benchmark<F>(&self, operation: F) -> BenchmarkResult
    where
        F: Fn() -> (),
    {
        let start = std::time::Instant::now();
        
        for _ in 0..self.iterations {
            operation();
        }
        
        let duration = start.elapsed();
        
        BenchmarkResult {
            name: self.name.clone(),
            iterations: self.iterations,
            total_time: duration,
            avg_time: duration / self.iterations as u32,
            ops_per_second: self.iterations as f64 / duration.as_secs_f64(),
        }
    }
    
    pub fn benchmark_with_setup<F, S>(&self, setup: S, operation: F) -> BenchmarkResult
    where
        F: Fn() -> (),
        S: Fn() -> (),
    {
        let start = std::time::Instant::now();
        
        for _ in 0..self.iterations {
            setup();
            operation();
        }
        
        let duration = start.elapsed();
        
        BenchmarkResult {
            name: self.name.clone(),
            iterations: self.iterations,
            total_time: duration,
            avg_time: duration / self.iterations as u32,
            ops_per_second: self.iterations as f64 / duration.as_secs_f64(),
        }
    }
}

#[derive(Debug)]
pub struct BenchmarkResult {
    pub name: String,
    pub iterations: usize,
    pub total_time: std::time::Duration,
    pub avg_time: std::time::Duration,
    pub ops_per_second: f64,
}
```

### Zero-Copy Operations

```rust
use std::borrow::Cow;
use std::ops::Deref;

pub struct ZeroCopyBuffer<'a> {
    data: Cow<'a, [u8]>,
}

impl<'a> ZeroCopyBuffer<'a> {
    pub fn from_borrowed(data: &'a [u8]) -> Self {
        ZeroCopyBuffer {
            data: Cow::Borrowed(data),
        }
    }
    
    pub fn from_owned(data: Vec<u8>) -> Self {
        ZeroCopyBuffer {
            data: Cow::Owned(data),
        }
    }
    
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }
    
    pub fn len(&self) -> usize {
        self.data.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    
    pub fn to_owned(&self) -> Vec<u8> {
        self.data.to_owned()
    }
    
    pub fn make_mut(&mut self) -> &mut Vec<u8> {
        self.data.to_mut()
    }
}

impl<'a> Deref for ZeroCopyBuffer<'a> {
    type Target = [u8];
    
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

// String interning for zero-copy string handling
pub struct StringInterner {
    strings: HashMap<String, usize>,
    data: Vec<String>,
}

impl StringInterner {
    pub fn new() -> Self {
        StringInterner {
            strings: HashMap::new(),
            data: Vec::new(),
        }
    }
    
    pub fn intern(&mut self, s: &str) -> usize {
        if let Some(&id) = self.strings.get(s) {
            id
        } else {
            let id = self.data.len();
            self.data.push(s.to_string());
            self.strings.insert(s.to_string(), id);
            id
        }
    }
    
    pub fn get(&self, id: usize) -> Option<&str> {
        self.data.get(id).map(|s| s.as_str())
    }
    
    pub fn get_or_intern(&mut self, s: &str) -> usize {
        self.intern(s)
    }
    
    pub fn len(&self) -> usize {
        self.data.len()
    }
    
    pub fn memory_usage(&self) -> usize {
        self.data.iter().map(|s| s.len()).sum::<usize>()
    }
}

// Slice-based processing for zero-copy
pub struct SliceProcessor<'a, T> {
    data: &'a [T],
    chunk_size: usize,
}

impl<'a, T> SliceProcessor<'a, T> {
    pub fn new(data: &'a [T], chunk_size: usize) -> Self {
        SliceProcessor {
            data,
            chunk_size,
        }
    }
    
    pub fn chunks(&self) -> impl Iterator<Item = &'a [T]> {
        self.data.chunks(self.chunk_size)
    }
    
    pub fn windows(&self, window_size: usize) -> impl Iterator<Item = &'a [T]> {
        self.data.windows(window_size)
    }
    
    pub fn process_chunks<F, R>(&self, mut processor: F) -> Vec<R>
    where
        F: FnMut(&'a [T]) -> R,
    {
        self.chunks().map(&mut processor).collect()
    }
    
    pub fn parallel_process_chunks<F, R>(&self, processor: F) -> Vec<R>
    where
        F: Fn(&'a [T]) -> R + Sync + Send,
        R: Send,
    {
        use std::sync::mpsc;
        use std::thread;
        
        let chunks: Vec<&[T]> = self.chunks().collect();
        let (tx, rx) = mpsc::channel();
        
        for chunk in chunks {
            let tx = tx.clone();
            thread::spawn(move || {
                let result = processor(chunk);
                tx.send(result).unwrap();
            });
        }
        
        drop(tx);
        
        rx.into_iter().collect()
    }
}
```

---

## Memory Safety and Performance

### Unsafe Memory Operations

```rust
use std::ptr;
use std::slice;

pub struct UnsafeMemoryOperations;

impl UnsafeMemoryOperations {
    // Fast memory copy
    pub unsafe fn fast_copy(src: *const u8, dst: *mut u8, len: usize) {
        ptr::copy_nonoverlapping(src, dst, len);
    }
    
    // Zero-initialize memory
    pub unsafe fn zero_memory(ptr: *mut u8, len: usize) {
        ptr::write_bytes(ptr, 0, len);
    }
    
    // Create slice from raw pointer
    pub unsafe fn slice_from_raw_parts<'a>(ptr: *const u8, len: usize) -> &'a [u8] {
        slice::from_raw_parts(ptr, len)
    }
    
    // Create mutable slice from raw pointer
    pub unsafe fn slice_from_raw_parts_mut<'a>(ptr: *mut u8, len: usize) -> &'a mut [u8] {
        slice::from_raw_parts_mut(ptr, len)
    }
    
    // Aligned memory allocation
    pub fn allocate_aligned<T>(count: usize) -> *mut T {
        let layout = std::alloc::Layout::from_size_align(
            count * std::mem::size_of::<T>(),
            std::mem::align_of::<T>(),
        ).unwrap();
        
        unsafe { std::alloc::alloc(layout) as *mut T }
    }
    
    // Deallocate aligned memory
    pub unsafe fn deallocate_aligned<T>(ptr: *mut T, count: usize) {
        let layout = std::alloc::Layout::from_size_align(
            count * std::mem::size_of::<T>(),
            std::mem::align_of::<T>(),
        ).unwrap();
        
        std::alloc::dealloc(ptr as *mut u8, layout);
    }
}

// High-performance vector with custom allocation
pub struct HighPerformanceVec<T> {
    ptr: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> HighPerformanceVec<T> {
    pub fn new() -> Self {
        HighPerformanceVec {
            ptr: ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }
    
    pub fn with_capacity(capacity: usize) -> Self {
        let ptr = if capacity > 0 {
            unsafe { UnsafeMemoryOperations::allocate_aligned::<T>(capacity) }
        } else {
            ptr::null_mut()
        };
        
        HighPerformanceVec {
            ptr,
            len: 0,
            capacity,
        }
    }
    
    pub fn push(&mut self, item: T) {
        if self.len == self.capacity {
            self.grow();
        }
        
        unsafe {
            ptr::write(self.ptr.add(self.len), item);
        }
        self.len += 1;
    }
    
    fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
        
        let new_ptr = unsafe { UnsafeMemoryOperations::allocate_aligned::<T>(new_capacity) };
        
        unsafe {
            ptr::copy_nonoverlapping(self.ptr, new_ptr, self.len);
            UnsafeMemoryOperations::deallocate_aligned(self.ptr, self.capacity);
        }
        
        self.ptr = new_ptr;
        self.capacity = new_capacity;
    }
    
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            unsafe { Some(&*self.ptr.add(index)) }
        } else {
            None
        }
    }
    
    pub fn len(&self) -> usize {
        self.len
    }
    
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

impl<T> Drop for HighPerformanceVec<T> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            // Drop all elements
            for i in 0..self.len {
                unsafe {
                    ptr::drop_in_place(self.ptr.add(i));
                }
            }
            
            // Deallocate memory
            unsafe {
                UnsafeMemoryOperations::deallocate_aligned(self.ptr, self.capacity);
            }
        }
    }
}

unsafe impl<T: Send> Send for HighPerformanceVec<T> {}
unsafe impl<T: Sync> Sync for HighPerformanceVec<T> {}
```

---

## Key Takeaways

- **Memory layout** affects performance significantly
- **Cache-friendly** data structures improve speed
- **Zero-copy** operations reduce memory overhead
- **Custom allocators** provide fine-grained control
- **Unsafe operations** require careful handling
- **Benchmarking** identifies performance bottlenecks
- **Memory pools** reduce allocation overhead

---

## Memory Management Best Practices

| Practice | Description | Implementation |
|----------|-------------|----------------|
| **Prefer stack allocation** | Use stack for small, short-lived objects | Local variables |
| **Minimize heap allocations** | Reduce dynamic memory usage | Object pooling |
| **Use appropriate data structures** | Choose based on access patterns | SOA vs AOS |
| **Profile memory usage** | Monitor memory consumption | Memory profilers |
| **Avoid premature optimization** | Measure before optimizing | Benchmarks |
| **Use zero-copy when possible** | Reduce unnecessary copies | Slices, Cow |
| **Align data properly** | Respect alignment requirements | Aligned allocators |
