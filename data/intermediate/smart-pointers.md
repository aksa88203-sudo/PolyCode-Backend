# C++ Smart Pointers

## Introduction to Smart Pointers

Smart pointers are objects that act like pointers but provide automatic memory management through RAII (Resource Acquisition Is Initialization).

## std::unique_ptr

### Basic Usage
```cpp
#include <memory>
#include <iostream>

class Resource {
public:
    Resource() { std::cout << "Resource created" << std::endl; }
    ~Resource() { std::cout << "Resource destroyed" << std::endl; }
    void doSomething() { std::cout << "Doing something" << std::endl; }
};

int main() {
    // Create unique_ptr
    std::unique_ptr<Resource> ptr1 = std::make_unique<Resource>();
    ptr1->doSomething();
    
    // unique_ptr cannot be copied
    // std::unique_ptr<Resource> ptr2 = ptr1;  // ERROR!
    
    // But can be moved
    std::unique_ptr<Resource> ptr2 = std::move(ptr1);
    ptr2->doSomething();
    
    // ptr1 is now nullptr
    if (!ptr1) {
        std::cout << "ptr1 is null after move" << std::endl;
    }
    
    // Automatic cleanup when ptr2 goes out of scope
    return 0;
}
```

### unique_ptr with Custom Deleters
```cpp
#include <memory>
#include <iostream>
#include <cstdio>

void custom_deleter(FILE* file) {
    if (file) {
        std::cout << "Closing file" << std::endl;
        std::fclose(file);
    }
}

int main() {
    // unique_ptr with custom deleter
    std::unique_ptr<FILE, decltype(&custom_deleter)> file_ptr(
        std::fopen("test.txt", "w"), custom_deleter);
    
    if (file_ptr) {
        std::fprintf(file_ptr.get(), "Hello, World!");
    }
    
    // File automatically closed when file_ptr goes out of scope
    return 0;
}
```

### unique_ptr with Arrays
```cpp
#include <memory>
#include <iostream>

int main() {
    // unique_ptr for arrays
    std::unique_ptr<int[]> arr = std::make_unique<int[]>(5);
    
    // Initialize array elements
    for (int i = 0; i < 5; ++i) {
        arr[i] = i * 10;
    }
    
    // Access elements
    for (int i = 0; i < 5; ++i) {
        std::cout << arr[i] << " ";
    }
    std::cout << std::endl;
    
    return 0;
}
```

## std::shared_ptr

### Basic Usage
```cpp
#include <memory>
#include <iostream>

class Resource {
public:
    Resource() { std::cout << "Resource created" << std::endl; }
    ~Resource() { std::cout << "Resource destroyed" << std::endl; }
    void doSomething() { std::cout << "Doing something" << std::endl; }
};

int main() {
    // Create shared_ptr
    std::shared_ptr<Resource> ptr1 = std::make_shared<Resource>();
    std::cout << "Reference count: " << ptr1.use_count() << std::endl;
    
    {
        // Copy shared_ptr (increases reference count)
        std::shared_ptr<Resource> ptr2 = ptr1;
        std::cout << "Reference count: " << ptr1.use_count() << std::endl;
        
        ptr2->doSomething();
    } // ptr2 goes out of scope
    
    std::cout << "Reference count after ptr2 destruction: " << ptr1.use_count() << std::endl;
    
    return 0;
} // Resource destroyed when last shared_ptr goes out of scope
```

### shared_ptr with Custom Deleters
```cpp
#include <memory>
#include <iostream>

class CustomResource {
private:
    int* data;
    
public:
    CustomResource(int size) : data(new int[size]) {
        std::cout << "CustomResource created" << std::endl;
    }
    
    ~CustomResource() {
        std::cout << "CustomResource destroyed" << std::endl;
    }
    
    int* getData() { return data; }
};

int main() {
    // shared_ptr with custom deleter
    std::shared_ptr<CustomResource> ptr(
        new CustomResource(10),
        [](CustomResource* res) {
            std::cout << "Custom deleter called" << std::endl;
            delete[] res->getData();
            delete res;
        });
    
    return 0;
}
```

### shared_ptr and Polymorphism
```cpp
#include <memory>
#include <iostream>

class Animal {
public:
    virtual ~Animal() = default;
    virtual void speak() const = 0;
};

class Dog : public Animal {
public:
    void speak() const override {
        std::cout << "Woof!" << std::endl;
    }
};

class Cat : public Animal {
public:
    void speak() const override {
        std::cout << "Meow!" << std::endl;
    }
};

void makeAnimalSpeak(const std::shared_ptr<Animal>& animal) {
    animal->speak();
}

int main() {
    std::shared_ptr<Animal> dog = std::make_shared<Dog>();
    std::shared_ptr<Animal> cat = std::make_shared<Cat>();
    
    makeAnimalSpeak(dog);
    makeAnimalSpeak(cat);
    
    return 0;
}
```

## std::weak_ptr

### Breaking Circular References
```cpp
#include <memory>
#include <iostream>

class Node {
private:
    std::string name;
    std::shared_ptr<Node> next;
    std::weak_ptr<Node> parent;  // Use weak_ptr to avoid circular reference
    
public:
    Node(const std::string& n) : name(n) {
        std::cout << "Node " << name << " created" << std::endl;
    }
    
    ~Node() {
        std::cout << "Node " << name << " destroyed" << std::endl;
    }
    
    void setNext(std::shared_ptr<Node> n) {
        next = n;
        n->parent = shared_from_this();  // Enable shared_from_this
    }
    
    void printInfo() const {
        std::cout << "Node: " << name;
        if (next) {
            std::cout << " -> " << next->name;
        }
        if (auto p = parent.lock()) {  // Convert weak_ptr to shared_ptr
            std::cout << " (parent: " << p->name << ")";
        }
        std::cout << std::endl;
    }
};

int main() {
    auto node1 = std::make_shared<Node>("A");
    auto node2 = std::make_shared<Node>("B");
    auto node3 = std::make_shared<Node>("C");
    
    node1->setNext(node2);
    node2->setNext(node3);
    
    node1->printInfo();
    node2->printInfo();
    node3->printInfo();
    
    return 0;
}  // All nodes destroyed properly
```

### weak_ptr for Caching
```cpp
#include <memory>
#include <iostream>
#include <unordered_map>

class Cache {
private:
    std::unordered_map<std::string, std::weak_ptr<int>> cache;
    
public:
    std::shared_ptr<int> get(const std::string& key) {
        auto it = cache.find(key);
        if (it != cache.end()) {
            if (auto ptr = it->second.lock()) {
                return ptr;  // Object still exists
            } else {
                cache.erase(it);  // Remove expired weak_ptr
            }
        }
        return nullptr;
    }
    
    void put(const std::string& key, std::shared_ptr<int> value) {
        cache[key] = value;
    }
};

int main() {
    Cache cache;
    
    {
        auto value = std::make_shared<int>(42);
        cache.put("answer", value);
        
        auto retrieved = cache.get("answer");
        if (retrieved) {
            std::cout << "Retrieved value: " << *retrieved << std::endl;
        }
    } // value goes out of scope
    
    auto retrieved = cache.get("answer");
    if (!retrieved) {
        std::cout << "Value expired" << std::endl;
    }
    
    return 0;
}
```

## Advanced Smart Pointer Techniques

### Factory Functions
```cpp
#include <memory>
#include <iostream>

class Widget {
public:
    virtual ~Widget() = default;
    virtual void draw() const = 0;
};

class Button : public Widget {
public:
    void draw() const override {
        std::cout << "Drawing button" << std::endl;
    }
};

class Slider : public Widget {
public:
    void draw() const override {
        std::cout << "Drawing slider" << std::endl;
    }
};

// Factory function returning smart pointer
std::unique_ptr<Widget> createWidget(const std::string& type) {
    if (type == "button") {
        return std::make_unique<Button>();
    } else if (type == "slider") {
        return std::make_unique<Slider>();
    }
    return nullptr;
}

int main() {
    auto widget = createWidget("button");
    if (widget) {
        widget->draw();
    }
    
    return 0;
}
```

### Smart Pointers with STL Containers
```cpp
#include <memory>
#include <vector>
#include <algorithm>
#include <iostream>

class GameObject {
public:
    virtual ~GameObject() = default;
    virtual void update() = 0;
};

class Player : public GameObject {
public:
    void update() override {
        std::cout << "Updating player" << std::endl;
    }
};

class Enemy : public GameObject {
public:
    void update() override {
        std::cout << "Updating enemy" << std::endl;
    }
};

int main() {
    // Container of smart pointers
    std::vector<std::unique_ptr<GameObject>> gameObjects;
    
    // Add objects to container
    gameObjects.push_back(std::make_unique<Player>());
    gameObjects.push_back(std::make_unique<Enemy>());
    gameObjects.emplace_back(std::make_unique<Player>());
    
    // Update all objects
    for (auto& obj : gameObjects) {
        obj->update();
    }
    
    // Move objects between containers
    std::vector<std::unique_ptr<GameObject>> enemies;
    
    auto it = std::remove_if(gameObjects.begin(), gameObjects.end(),
        [&enemies](std::unique_ptr<GameObject>& obj) {
            if (dynamic_cast<Enemy*>(obj.get())) {
                enemies.push_back(std::move(obj));
                return true;
            }
            return false;
        });
    
    gameObjects.erase(it, gameObjects.end());
    
    std::cout << "Enemies: " << enemies.size() << std::endl;
    std::cout << "Others: " << gameObjects.size() << std::endl;
    
    return 0;
}
```

### enable_shared_from_this
```cpp
#include <memory>
#include <iostream>

class NetworkConnection : public std::enable_shared_from_this<NetworkConnection> {
private:
    std::string address;
    
public:
    NetworkConnection(const std::string& addr) : address(addr) {
        std::cout << "Connection to " << address << " established" << std::endl;
    }
    
    ~NetworkConnection() {
        std::cout << "Connection to " << address << " closed" << std::endl;
    }
    
    void sendData(const std::string& data) {
        std::cout << "Sending data to " << address << ": " << data << std::endl;
        
        // Schedule another operation using shared_from_this
        scheduleHeartbeat();
    }
    
    void scheduleHeartbeat() {
        // Get shared_ptr to this object
        std::shared_ptr<NetworkConnection> self = shared_from_this();
        
        // Simulate async operation
        std::cout << "Scheduling heartbeat for " << address << std::endl;
        // In real code, this would be passed to an async operation
    }
};

int main() {
    auto connection = std::make_shared<NetworkConnection>("192.168.1.1");
    connection->sendData("Hello, Server!");
    
    return 0;
}
```

## Performance Considerations

### unique_ptr vs Raw Pointers
```cpp
#include <memory>
#include <chrono>
#include <iostream>

void benchmarkRawPointers() {
    const int size = 1000000;
    int** raw_ptrs = new int*[size];
    
    auto start = std::chrono::high_resolution_clock::now();
    
    for (int i = 0; i < size; ++i) {
        raw_ptrs[i] = new int(i);
    }
    
    for (int i = 0; i < size; ++i) {
        delete raw_ptrs[i];
    }
    
    auto end = std::chrono::high_resolution_clock::now();
    auto duration = std::chrono::duration_cast<std::chrono::microseconds>(end - start);
    
    std::cout << "Raw pointers: " << duration.count() << " microseconds" << std::endl;
    delete[] raw_ptrs;
}

void benchmarkUniquePointers() {
    const int size = 1000000;
    std::unique_ptr<int*[]> unique_ptrs = std::make_unique<int*[]>(size);
    
    auto start = std::chrono::high_resolution_clock::now();
    
    for (int i = 0; i < size; ++i) {
        unique_ptrs[i] = std::make_unique<int>(i).release();
    }
    
    for (int i = 0; i < size; ++i) {
        std::unique_ptr<int>(unique_ptrs[i]);
    }
    
    auto end = std::chrono::high_resolution_clock::now();
    auto duration = std::chrono::duration_cast<std::chrono::microseconds>(end - start);
    
    std::cout << "Unique pointers: " << duration.count() << " microseconds" << std::endl;
}

int main() {
    benchmarkRawPointers();
    benchmarkUniquePointers();
    
    return 0;
}
```

## Best Practices

### Guidelines for Smart Pointer Usage
```cpp
#include <memory>
#include <vector>
#include <iostream>

class Resource {
public:
    Resource() { std::cout << "Resource created" << std::endl; }
    ~Resource() { std::cout << "Resource destroyed" << std::endl; }
    void use() { std::cout << "Using resource" << std::endl; }
};

// GOOD: Use make_unique and make_shared
std::unique_ptr<Resource> createResource() {
    return std::make_unique<Resource>();
}

// GOOD: Pass smart pointers by const reference when not transferring ownership
void useResource(const std::shared_ptr<Resource>& resource) {
    resource->use();
}

// GOOD: Return smart pointers from factory functions
std::shared_ptr<Resource> createSharedResource() {
    return std::make_shared<Resource>();
}

// GOOD: Use unique_ptr for exclusive ownership
class ResourceManager {
private:
    std::unique_ptr<Resource> resource;
    
public:
    ResourceManager() : resource(std::make_unique<Resource>()) {}
    
    // Move constructor for ownership transfer
    ResourceManager(ResourceManager&& other) noexcept 
        : resource(std::move(other.resource)) {}
    
    // Move assignment
    ResourceManager& operator=(ResourceManager&& other) noexcept {
        if (this != &other) {
            resource = std::move(other.resource);
        }
        return *this;
    }
    
    // Delete copy operations
    ResourceManager(const ResourceManager&) = delete;
    ResourceManager& operator=(const ResourceManager&) = delete;
};

// GOOD: Use weak_ptr for non-owning references
class Observer {
private:
    std::weak_ptr<Resource> observed_resource;
    
public:
    void observe(const std::shared_ptr<Resource>& resource) {
        observed_resource = resource;
    }
    
    void onEvent() {
        if (auto resource = observed_resource.lock()) {
            resource->use();
        } else {
            std::cout << "Resource no longer exists" << std::endl;
        }
    }
};

int main() {
    // Use make_unique/make_shared instead of new
    auto resource = createResource();
    
    // Use shared_ptr for shared ownership
    auto shared_resource = createSharedResource();
    useResource(shared_resource);
    
    // Use unique_ptr for exclusive ownership
    ResourceManager manager;
    
    // Use weak_ptr for observer pattern
    Observer observer;
    observer.observe(shared_resource);
    observer.onEvent();
    
    return 0;
}
```

## Common Pitfalls and Solutions

### Circular References
```cpp
// BAD: Circular reference prevents destruction
class BadNode {
public:
    std::shared_ptr<BadNode> next;
    std::shared_ptr<BadNode> prev;
    
    ~BadNode() { std::cout << "BadNode destroyed" << std::endl; }
};

// GOOD: Use weak_ptr to break cycles
class GoodNode {
public:
    std::shared_ptr<GoodNode> next;
    std::weak_ptr<GoodNode> prev;  // Use weak_ptr
    
    ~GoodNode() { std::cout << "GoodNode destroyed" << std::endl; }
};
```

### Dangling Pointers
```cpp
// BAD: Raw pointer can become dangling
class BadExample {
    int* raw_ptr;
public:
    BadExample() : raw_ptr(new int(42)) {}
    ~BadExample() { delete raw_ptr; }
    int* get() { return raw_ptr; }  // Dangerous
};

// GOOD: Return smart pointer or reference
class GoodExample {
    std::unique_ptr<int> smart_ptr;
public:
    GoodExample() : smart_ptr(std::make_unique<int>(42)) {}
    int& get() { return *smart_ptr; }  // Safe reference
    std::shared_ptr<int> getShared() {
        return std::shared_ptr<int>(smart_ptr.get(), [](int*){});  // Non-owning shared_ptr
    }
};
```

## Best Practices Summary
- Use `std::make_unique` and `std::make_shared` instead of `new`
- Use `std::unique_ptr` for exclusive ownership
- Use `std::shared_ptr` for shared ownership
- Use `std::weak_ptr` to break circular references
- Pass smart pointers by `const&` when not transferring ownership
- Use `std::move` to transfer ownership of `unique_ptr`
- Avoid mixing raw pointers and smart pointers
- Use `enable_shared_from_this` when a class needs `shared_ptr` to itself
- Be aware of the overhead of `shared_ptr` (reference counting)
- Use custom deleters for non-standard cleanup
- Prefer smart pointers over raw pointers for automatic memory management
