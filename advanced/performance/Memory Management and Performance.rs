// memory_management_and_performance.rs
// Memory management and performance examples in Rust

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;
use std::borrow::Cow;
use std::ops::Deref;

// Memory profiler
#[derive(Debug)]
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

// Memory pool implementation
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

// Cache-friendly array
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
    
    pub fn len(&self) -> usize {
        self.data.len() / self.stride
    }
    
    pub fn memory_usage(&self) -> usize {
        self.data.len() * std::mem::size_of::<T>()
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
        (self.positions.len() * std::mem::size_of::<[f32; 3]>() +
         self.velocities.len() * std::mem::size_of::<[f32; 3]>() +
         self.masses.len() * std::mem::size_of::<f32>() +
         self.radii.len() * std::mem::size_of::<f32>())
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
        let start = Instant::now();
        
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
}

#[derive(Debug)]
pub struct BenchmarkResult {
    pub name: String,
    pub iterations: usize,
    pub total_time: std::time::Duration,
    pub avg_time: std::time::Duration,
    pub ops_per_second: f64,
}

// Zero-copy buffer
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
    
    pub fn to_owned(&self) -> Vec<u8> {
        self.data.to_owned()
    }
}

impl<'a> Deref for ZeroCopyBuffer<'a> {
    type Target = [u8];
    
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

// String interning
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
    
    pub fn len(&self) -> usize {
        self.data.len()
    }
    
    pub fn memory_usage(&self) -> usize {
        self.data.iter().map(|s| s.len()).sum::<usize>()
    }
}

// Slice processor
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
}

// High-performance vector
pub struct HighPerformanceVec<T> {
    ptr: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> HighPerformanceVec<T> {
    pub fn new() -> Self {
        HighPerformanceVec {
            ptr: std::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }
    
    pub fn with_capacity(capacity: usize) -> Self {
        let ptr = if capacity > 0 {
            unsafe { Self::allocate_aligned::<T>(capacity) }
        } else {
            std::ptr::null_mut()
        };
        
        HighPerformanceVec {
            ptr,
            len: 0,
            capacity,
        }
    }
    
    fn allocate_aligned<T>(count: usize) -> *mut T {
        let layout = std::alloc::Layout::from_size_align(
            count * std::mem::size_of::<T>(),
            std::mem::align_of::<T>(),
        ).unwrap();
        
        unsafe { std::alloc::alloc(layout) as *mut T }
    }
    
    fn deallocate_aligned<T>(ptr: *mut T, count: usize) {
        let layout = std::alloc::Layout::from_size_align(
            count * std::mem::size_of::<T>(),
            std::mem::align_of::<T>(),
        ).unwrap();
        
        unsafe {
            std::alloc::dealloc(ptr as *mut u8, layout);
        }
    }
    
    pub fn push(&mut self, item: T) {
        if self.len == self.capacity {
            self.grow();
        }
        
        unsafe {
            std::ptr::write(self.ptr.add(self.len), item);
        }
        self.len += 1;
    }
    
    fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
        
        let new_ptr = unsafe { Self::allocate_aligned::<T>(new_capacity) };
        
        unsafe {
            std::ptr::copy_nonoverlapping(self.ptr, new_ptr, self.len);
            Self::deallocate_aligned(self.ptr, self.capacity);
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
                    std::ptr::drop_in_place(self.ptr.add(i));
                }
            }
            
            // Deallocate memory
            Self::deallocate_aligned(self.ptr, self.capacity);
        }
    }
}

unsafe impl<T: Send> Send for HighPerformanceVec<T> {}
unsafe impl<T: Sync> Sync for HighPerformanceVec<T> {}

// Main demonstration
fn main() {
    println!("=== MEMORY MANAGEMENT AND PERFORMANCE DEMONSTRATIONS ===\n");
    
    // Memory profiling
    println!("=== MEMORY PROFILING ===");
    let mut profiler = MemoryProfiler::new();
    
    let stack_value = profiler.profile_stack_allocation(42u32);
    let heap_value = profiler.profile_heap_allocation(String::from("Hello"));
    
    let stats = profiler.get_memory_stats();
    println!("Memory stats: {:?}", stats);
    
    // Memory pool
    println!("\n=== MEMORY POOL ===");
    let mut pool = MemoryPool::new(10);
    
    let index1 = pool.allocate().unwrap();
    let index2 = pool.allocate().unwrap();
    
    pool.get_mut(index1).map(|slot| *slot = 42);
    pool.get_mut(index2).map(|slot| *slot = 100);
    
    println!("Pool slot 1: {:?}", pool.get(index1));
    println!("Pool slot 2: {:?}", pool.get(index2));
    println!("Available slots: {}", pool.available_count());
    println!("Used slots: {}", pool.used_count());
    
    pool.deallocate(index1);
    println!("After deallocation - Available: {}", pool.available_count());
    
    // Cache-friendly arrays
    println!("\n=== CACHE-FRIENDLY ARRAYS ===");
    let mut cache_array = CacheFriendlyArray::new(100);
    
    for i in 0..100 {
        cache_array.push(i);
    }
    
    println!("Cache array length: {}", cache_array.len());
    println!("Memory usage: {} bytes", cache_array.memory_usage());
    
    // SOA vs AOS comparison
    println!("\n=== SOA vs AOS ===");
    let mut particles_aos: Vec<ParticleAOS> = Vec::new();
    let mut particles_soa = ParticleSOA::new(1000);
    
    // Create particles
    for i in 0..1000 {
        let particle = ParticleAOS {
            position: [i as f32, i as f32 * 2.0, i as f32 * 3.0],
            velocity: [1.0, 2.0, 3.0],
            mass: (i % 100) as f32,
            radius: 0.5,
        };
        particles_aos.push(particle);
        
        particles_soa.add_particle(
            [i as f32, i as f32 * 2.0, i as f32 * 3.0],
            [1.0, 2.0, 3.0],
            (i % 100) as f32,
            0.5,
        );
    }
    
    println!("AOS memory: {} bytes", particles_aos.len() * std::mem::size_of::<ParticleAOS>());
    println!("SOA memory: {} bytes", particles_soa.memory_usage());
    
    // Update positions (SOA is more cache-friendly)
    particles_soa.update_positions(0.016); // 60 FPS
    
    // Performance benchmarking
    println!("\n=== PERFORMANCE BENCHMARKING ===");
    let benchmark = PerformanceBenchmark::new("Vector operations".to_string(), 100000);
    
    let result = benchmark.benchmark(|| {
        let mut vec = Vec::new();
        for i in 0..100 {
            vec.push(i);
        }
        let sum: i32 = vec.iter().sum();
        std::hint::black_box(sum);
    });
    
    println!("Benchmark result: {:?}", result);
    
    // Zero-copy operations
    println!("\n=== ZERO-COPY OPERATIONS ===");
    let data = vec![1, 2, 3, 4, 5];
    let zero_copy = ZeroCopyBuffer::from_borrowed(&data);
    
    println!("Zero-copy buffer length: {}", zero_copy.len());
    println!("Original data: {:?}", zero_copy.as_slice());
    
    let owned_data = zero_copy.to_owned();
    println!("Owned data: {:?}", owned_data);
    
    // String interning
    println!("\n=== STRING INTERNING ===");
    let mut interner = StringInterner::new();
    
    let id1 = interner.intern("hello");
    let id2 = interner.intern("world");
    let id3 = interner.intern("hello"); // Should reuse
    
    println!("String IDs: {}, {}, {}", id1, id2, id3);
    println!("String 1: {:?}", interner.get(id1));
    println!("String 2: {:?}", interner.get(id2));
    println!("String 3: {:?}", interner.get(id3));
    println!("Memory usage: {} bytes", interner.memory_usage());
    
    // Slice processing
    println!("\n=== SLICE PROCESSING ===");
    let numbers: Vec<i32> = (0..20).collect();
    let processor = SliceProcessor::new(&numbers, 5);
    
    let sums = processor.process_chunks(|chunk| {
        chunk.iter().sum::<i32>()
    });
    
    println!("Chunk sums: {:?}", sums);
    
    // High-performance vector
    println!("\n=== HIGH-PERFORMANCE VECTOR ===");
    let mut hp_vec = HighPerformanceVec::new();
    
    for i in 0..1000 {
        hp_vec.push(i * 2);
    }
    
    println!("HP vector length: {}", hp_vec.len());
    println!("HP vector capacity: {}", hp_vec.capacity());
    println!("First element: {:?}", hp_vec.get(0));
    println!("Last element: {:?}", hp_vec.get(999));
    
    println!("\n=== MEMORY MANAGEMENT AND PERFORMANCE DEMONSTRATIONS COMPLETE ===");
    println!("Key concepts demonstrated:");
    println!("- Memory profiling and allocation tracking");
    println!("- Memory pools for efficient allocation");
    println!("- Cache-friendly data structures");
    println!("- SOA vs AOS memory layouts");
    println!("- Performance benchmarking");
    println!("- Zero-copy operations with Cow");
    println!("- String interning for memory efficiency");
    println!("- Slice-based processing");
    println!("- High-performance custom vector");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_memory_pool() {
        let mut pool = MemoryPool::new(5);
        
        let id1 = pool.allocate().unwrap();
        let id2 = pool.allocate().unwrap();
        
        assert_eq!(pool.used_count(), 2);
        assert_eq!(pool.available_count(), 3);
        
        pool.deallocate(id1);
        assert_eq!(pool.used_count(), 1);
        assert_eq!(pool.available_count(), 4);
    }
    
    #[test]
    fn test_cache_friendly_array() {
        let mut array = CacheFriendlyArray::new(10);
        
        for i in 0..10 {
            array.push(i);
        }
        
        assert_eq!(array.len(), 10);
        assert_eq!(array.get(5), Some(&5));
        assert_eq!(array.get(10), None);
    }
    
    #[test]
    fn test_string_interner() {
        let mut interner = StringInterner::new();
        
        let id1 = interner.intern("test");
        let id2 = interner.intern("test");
        let id3 = interner.intern("other");
        
        assert_eq!(id1, id2); // Should be the same
        assert_ne!(id1, id3); // Should be different
        
        assert_eq!(interner.get(id1), Some("test"));
        assert_eq!(interner.get(id3), Some("other"));
    }
    
    #[test]
    fn test_zero_copy_buffer() {
        let data = vec![1, 2, 3];
        let buffer = ZeroCopyBuffer::from_borrowed(&data);
        
        assert_eq!(buffer.len(), 3);
        assert_eq!(buffer.as_slice(), &[1, 2, 3]);
    }
    
    #[test]
    fn test_high_performance_vec() {
        let mut vec = HighPerformanceVec::new();
        
        for i in 0..10 {
            vec.push(i);
        }
        
        assert_eq!(vec.len(), 10);
        assert_eq!(vec.get(5), Some(&5));
        assert_eq!(vec.get(10), None);
    }
}
