# C++ Concurrency

## Thread Basics

### std::thread (C++11)
```cpp
#include <iostream>
#include <thread>
#include <chrono>

void worker_function(int id) {
    std::cout << "Worker thread " << id << " started" << std::endl;
    std::this_thread::sleep_for(std::chrono::seconds(1));
    std::cout << "Worker thread " << id << " finished" << std::endl;
}

int main() {
    std::thread t1(worker_function, 1);
    std::thread t2(worker_function, 2);
    std::thread t3(worker_function, 3);
    
    std::cout << "Main thread waiting for workers" << std::endl;
    
    t1.join();
    t2.join();
    t3.join();
    
    std::cout << "All threads completed" << std::endl;
    return 0;
}
```

### Lambda Functions with Threads
```cpp
#include <thread>
#include <iostream>

int main() {
    int value = 42;
    
    std::thread t([&value]() {
        std::cout << "Thread captured value: " << value << std::endl;
        value = 100;
    });
    
    t.join();
    std::cout << "Value after thread: " << value << std::endl;
    
    return 0;
}
```

## Thread Synchronization

### std::mutex
```cpp
#include <iostream>
#include <thread>
#include <mutex>
#include <vector>

class Counter {
private:
    int count;
    std::mutex mtx;
    
public:
    Counter() : count(0) {}
    
    void increment() {
        std::lock_guard<std::mutex> lock(mtx);
        count++;
    }
    
    int getCount() {
        std::lock_guard<std::mutex> lock(mtx);
        return count;
    }
};

void worker(Counter& counter, int increments) {
    for (int i = 0; i < increments; ++i) {
        counter.increment();
    }
}

int main() {
    Counter counter;
    std::vector<std::thread> threads;
    
    // Create multiple threads
    for (int i = 0; i < 10; ++i) {
        threads.emplace_back(worker, std::ref(counter), 1000);
    }
    
    // Wait for all threads to complete
    for (auto& t : threads) {
        t.join();
    }
    
    std::cout << "Final count: " << counter.getCount() << std::endl;
    return 0;
}
```

### std::lock_guard and std::unique_lock
```cpp
#include <mutex>
#include <iostream>

class SafeResource {
private:
    std::mutex mtx;
    int value;
    
public:
    SafeResource() : value(0) {}
    
    void setValue(int new_value) {
        std::lock_guard<std::mutex> lock(mtx);  // RAII lock
        value = new_value;
    }
    
    int getValue() {
        std::unique_lock<std::mutex> lock(mtx);  // More flexible
        return value;
    }
    
    bool tryUpdate(int old_value, int new_value) {
        std::unique_lock<std::mutex> lock(mtx, std::try_to_lock);
        if (!lock.owns_lock()) {
            return false;  // Couldn't acquire lock
        }
        
        if (value == old_value) {
            value = new_value;
            return true;
        }
        return false;
    }
};
```

### std::condition_variable
```cpp
#include <iostream>
#include <thread>
#include <mutex>
#include <condition_variable>
#include <queue>
#include <chrono>

template<typename T>
class ThreadSafeQueue {
private:
    std::queue<T> data_queue;
    std::mutex mtx;
    std::condition_variable cv;
    
public:
    void push(T value) {
        {
            std::lock_guard<std::mutex> lock(mtx);
            data_queue.push(value);
        }
        cv.notify_one();
    }
    
    T pop() {
        std::unique_lock<std::mutex> lock(mtx);
        cv.wait(lock, [this] { return !data_queue.empty(); });
        
        T value = data_queue.front();
        data_queue.pop();
        return value;
    }
    
    bool tryPop(T& value, std::chrono::milliseconds timeout) {
        std::unique_lock<std::mutex> lock(mtx);
        
        if (!cv.wait_for(lock, timeout, [this] { return !data_queue.empty(); })) {
            return false;  // Timeout
        }
        
        value = data_queue.front();
        data_queue.pop();
        return true;
    }
};

void producer(ThreadSafeQueue<int>& queue) {
    for (int i = 0; i < 10; ++i) {
        std::cout << "Producing: " << i << std::endl;
        queue.push(i);
        std::this_thread::sleep_for(std::chrono::milliseconds(100));
    }
}

void consumer(ThreadSafeQueue<int>& queue) {
    for (int i = 0; i < 10; ++i) {
        int value = queue.pop();
        std::cout << "Consuming: " << value << std::endl;
    }
}

int main() {
    ThreadSafeQueue<int> queue;
    
    std::thread producer_thread(producer, std::ref(queue));
    std::thread consumer_thread(consumer, std::ref(queue));
    
    producer_thread.join();
    consumer_thread.join();
    
    return 0;
}
```

## Atomic Operations

### std::atomic
```cpp
#include <atomic>
#include <thread>
#include <vector>
#include <iostream>

class AtomicCounter {
private:
    std::atomic<int> count;
    
public:
    AtomicCounter() : count(0) {}
    
    void increment() {
        count.fetch_add(1, std::memory_order_relaxed);
    }
    
    int getValue() const {
        return count.load(std::memory_order_relaxed);
    }
    
    void reset() {
        count.store(0, std::memory_order_relaxed);
    }
};

void atomic_worker(AtomicCounter& counter, int increments) {
    for (int i = 0; i < increments; ++i) {
        counter.increment();
    }
}

int main() {
    AtomicCounter counter;
    std::vector<std::thread> threads;
    
    // Create multiple threads
    for (int i = 0; i < 10; ++i) {
        threads.emplace_back(atomic_worker, std::ref(counter), 1000);
    }
    
    // Wait for all threads to complete
    for (auto& t : threads) {
        t.join();
    }
    
    std::cout << "Final count: " << counter.getValue() << std::endl;
    return 0;
}
```

### Compare-and-Swap Loop
```cpp
#include <atomic>
#include <iostream>

class LockFreeStack {
private:
    struct Node {
        int data;
        Node* next;
        Node(int val) : data(val), next(nullptr) {}
    };
    
    std::atomic<Node*> head;
    
public:
    LockFreeStack() : head(nullptr) {}
    
    void push(int value) {
        Node* new_node = new Node(value);
        new_node->next = head.load();
        
        while (!head.compare_exchange_weak(new_node->next, new_node)) {
            // Retry if failed
        }
    }
    
    bool pop(int& result) {
        Node* old_head = head.load();
        
        while (old_head && 
               !head.compare_exchange_weak(old_head, old_head->next)) {
            // Retry if failed
        }
        
        if (old_head) {
            result = old_head->data;
            delete old_head;
            return true;
        }
        return false;
    }
};
```

## Future and Promise

### std::future and std::promise
```cpp
#include <future>
#include <iostream>
#include <thread>

void calculate_sum(std::promise<int> prom, int a, int b) {
    std::this_thread::sleep_for(std::chrono::seconds(1));
    prom.set_value(a + b);
}

int main() {
    std::promise<int> prom;
    std::future<int> fut = prom.get_future();
    
    std::thread t(calculate_sum, std::move(prom), 10, 20);
    
    std::cout << "Waiting for result..." << std::endl;
    int result = fut.get();
    std::cout << "Sum: " << result << std::endl;
    
    t.join();
    return 0;
}
```

### std::async
```cpp
#include <future>
#include <iostream>
#include <vector>

int calculate_factorial(int n) {
    if (n <= 1) return 1;
    return n * calculate_factorial(n - 1);
}

int main() {
    std::vector<std::future<int>> futures;
    
    // Launch multiple asynchronous tasks
    for (int i = 5; i <= 10; ++i) {
        futures.push_back(std::async(std::launch::async, calculate_factorial, i));
    }
    
    // Collect results
    for (size_t i = 0; i < futures.size(); ++i) {
        int result = futures[i].get();
        std::cout << "Factorial of " << (i + 5) << " is " << result << std::endl;
    }
    
    return 0;
}
```

### std::packaged_task
```cpp
#include <future>
#include <functional>
#include <iostream>

int multiply(int a, int b) {
    return a * b;
}

int main() {
    // Wrap function in packaged_task
    std::packaged_task<int(int, int)> task(multiply);
    
    // Get future
    std::future<int> result = task.get_future();
    
    // Run task in separate thread
    std::thread t(std::move(task), 6, 7);
    
    // Get result
    std::cout << "Result: " << result.get() << std::endl;
    
    t.join();
    return 0;
}
```

## Advanced Synchronization

### std::shared_mutex (C++17)
```cpp
#include <shared_mutex>
#include <thread>
#include <vector>
#include <iostream>

class ThreadSafeCounter {
private:
    mutable std::shared_mutex mtx;
    int value;
    
public:
    ThreadSafeCounter() : value(0) {}
    
    // Write operation - exclusive lock
    void increment() {
        std::unique_lock<std::shared_mutex> lock(mtx);
        ++value;
    }
    
    // Read operation - shared lock
    int getValue() const {
        std::shared_lock<std::shared_mutex> lock(mtx);
        return value;
    }
    
    // Multiple readers can access simultaneously
    void read_multiple_times(int count) const {
        for (int i = 0; i < count; ++i) {
            std::shared_lock<std::shared_mutex> lock(mtx);
            std::cout << "Reader sees: " << value << std::endl;
            std::this_thread::sleep_for(std::chrono::milliseconds(10));
        }
    }
};

void writer(ThreadSafeCounter& counter, int increments) {
    for (int i = 0; i < increments; ++i) {
        counter.increment();
        std::this_thread::sleep_for(std::chrono::milliseconds(50));
    }
}

void reader(ThreadSafeCounter& counter, int reads) {
    counter.read_multiple_times(reads);
}

int main() {
    ThreadSafeCounter counter;
    std::vector<std::thread> threads;
    
    // Create multiple readers and one writer
    threads.emplace_back(writer, std::ref(counter), 5);
    
    for (int i = 0; i < 3; ++i) {
        threads.emplace_back(reader, std::ref(counter), 10);
    }
    
    for (auto& t : threads) {
        t.join();
    }
    
    return 0;
}
```

### std::call_once
```cpp
#include <mutex>
#include <iostream>

class Singleton {
private:
    static std::once_flag initialized;
    static Singleton* instance;
    
    Singleton() { std::cout << "Singleton created" << std::endl; }
    
public:
    static Singleton& getInstance() {
        std::call_once(initialized, []() {
            instance = new Singleton();
        });
        return *instance;
    }
};

std::once_flag Singleton::initialized;
Singleton* Singleton::instance = nullptr;

void use_singleton() {
    Singleton& singleton = Singleton::getInstance();
    std::cout << "Using singleton instance" << std::endl;
}

int main() {
    std::thread t1(use_singleton);
    std::thread t2(use_singleton);
    std::thread t3(use_singleton);
    
    t1.join();
    t2.join();
    t3.join();
    
    return 0;
}
```

## Thread Pool

### Simple Thread Pool Implementation
```cpp
#include <vector>
#include <queue>
#include <thread>
#include <mutex>
#include <condition_variable>
#include <functional>
#include <future>
#include <memory>

class ThreadPool {
private:
    std::vector<std::thread> workers;
    std::queue<std::function<void()>> tasks;
    
    std::mutex queue_mutex;
    std::condition_variable condition;
    bool stop;
    
public:
    ThreadPool(size_t threads) : stop(false) {
        for (size_t i = 0; i < threads; ++i) {
            workers.emplace_back([this] {
                for (;;) {
                    std::function<void()> task;
                    
                    {
                        std::unique_lock<std::mutex> lock(queue_mutex);
                        
                        condition.wait(lock, [this] { 
                            return stop || !tasks.empty(); 
                        });
                        
                        if (stop && tasks.empty()) {
                            return;
                        }
                        
                        task = std::move(tasks.front());
                        tasks.pop();
                    }
                    
                    task();
                }
            });
        }
    }
    
    template<class F, class... Args>
    auto enqueue(F&& f, Args&&... args) 
        -> std::future<typename std::result_of<F(Args...)>::type> {
        
        using return_type = typename std::result_of<F(Args...)>::type;
        
        auto task = std::make_shared<std::packaged_task<return_type()>>(
            std::bind(std::forward<F>(f), std::forward<Args>(args)...)
        );
        
        std::future<return_type> result = task->get_future();
        
        {
            std::unique_lock<std::mutex> lock(queue_mutex);
            
            if (stop) {
                throw std::runtime_error("enqueue on stopped ThreadPool");
            }
            
            tasks.emplace([task](){ (*task)(); });
        }
        
        condition.notify_one();
        return result;
    }
    
    ~ThreadPool() {
        {
            std::unique_lock<std::mutex> lock(queue_mutex);
            stop = true;
        }
        
        condition.notify_all();
        
        for (std::thread &worker : workers) {
            worker.join();
        }
    }
};

int main() {
    ThreadPool pool(4);
    
    std::vector<std::future<int>> results;
    
    // Enqueue tasks
    for (int i = 0; i < 8; ++i) {
        results.emplace_back(
            pool.enqueue([i] {
                std::cout << "Task " << i << " executing" << std::endl;
                std::this_thread::sleep_for(std::chrono::milliseconds(100));
                return i * i;
            })
        );
    }
    
    // Get results
    for (auto&& result : results) {
        std::cout << "Result: " << result.get() << std::endl;
    }
    
    return 0;
}
```

## Memory Ordering

### Memory Order Examples
```cpp
#include <atomic>
#include <thread>
#include <iostream>

std::atomic<bool> ready(false);
std::atomic<int> data(0);

void producer() {
    data.store(42, std::memory_order_release);
    ready.store(true, std::memory_order_release);
}

void consumer() {
    while (!ready.load(std::memory_order_acquire)) {
        // Spin wait
    }
    std::cout << "Data: " << data.load(std::memory_order_acquire) << std::endl;
}

int main() {
    std::thread t1(producer);
    std::thread t2(consumer);
    
    t1.join();
    t2.join();
    
    return 0;
}
```

## Best Practices

### Thread Safety Guidelines
```cpp
#include <mutex>
#include <memory>
#include <vector>
#include <algorithm>

// GOOD: Use RAII for lock management
class SafeContainer {
private:
    std::vector<int> data;
    mutable std::mutex mtx;
    
public:
    void add(int value) {
        std::lock_guard<std::mutex> lock(mtx);
        data.push_back(value);
    }
    
    bool contains(int value) const {
        std::lock_guard<std::mutex> lock(mtx);
        return std::find(data.begin(), data.end(), value) != data.end();
    }
    
    // GOOD: Separate read and write operations
    std::vector<int> copy() const {
        std::lock_guard<std::mutex> lock(mtx);
        return data;
    }
};

// GOOD: Use atomic operations for simple counters
class AtomicCounter {
private:
    std::atomic<int> count{0};
    
public:
    void increment() {
        count.fetch_add(1, std::memory_order_relaxed);
    }
    
    int get() const {
        return count.load(std::memory_order_relaxed);
    }
};

// GOOD: Use smart pointers with thread-safe containers
class ThreadSafeObjectPool {
private:
    std::queue<std::unique_ptr<int>> pool;
    std::mutex mtx;
    
public:
    std::unique_ptr<int> acquire() {
        std::lock_guard<std::mutex> lock(mtx);
        if (pool.empty()) {
            return std::make_unique<int>();
        }
        
        auto obj = std::move(pool.front());
        pool.pop();
        return obj;
    }
    
    void release(std::unique_ptr<int> obj) {
        std::lock_guard<std::mutex> lock(mtx);
        pool.push(std::move(obj));
    }
};
```

## Best Practices Summary
- Use RAII for lock management (`std::lock_guard`, `std::unique_lock`)
- Prefer `std::atomic` for simple operations over mutexes
- Use `std::shared_mutex` for read-heavy workloads (C++17)
- Avoid deadlocks by consistent lock ordering
- Use `std::condition_variable` for thread coordination
- Prefer `std::async` over manual thread management
- Use thread pools for managing many short tasks
- Be aware of memory ordering in atomic operations
- Minimize shared mutable state
- Use `std::call_once` for one-time initialization
- Consider lock-free data structures for high-performance scenarios
- Profile and optimize critical sections
- Handle exceptions properly in multi-threaded code
- Use appropriate synchronization primitives for the use case
