# C++ Standard Template Library (STL)

## Containers

### Sequence Containers

#### std::vector
```cpp
#include <vector>
#include <iostream>
#include <algorithm>

int main() {
    // Creation and initialization
    std::vector<int> vec1;                    // Empty vector
    std::vector<int> vec2(5, 0);               // 5 elements, all 0
    std::vector<int> vec3 = {1, 2, 3, 4, 5};  // Initializer list
    std::vector<int> vec4(vec3);              // Copy constructor
    
    // Adding elements
    vec1.push_back(10);
    vec1.push_back(20);
    vec1.insert(vec1.begin() + 1, 15);        // Insert at position 1
    
    // Accessing elements
    std::cout << vec1[0] << std::endl;        // 10 (no bounds checking)
    std::cout << vec1.at(0) << std::endl;      // 10 (with bounds checking)
    std::cout << vec1.front() << std::endl;    // First element
    std::cout << vec1.back() << std::endl;     // Last element
    
    // Size and capacity
    std::cout << "Size: " << vec1.size() << std::endl;
    std::cout << "Capacity: " << vec1.capacity() << std::endl;
    
    // Removing elements
    vec1.pop_back();                          // Remove last element
    vec1.erase(vec1.begin());                  // Remove first element
    vec1.clear();                             // Remove all elements
    
    // Algorithms
    std::vector<int> numbers = {5, 2, 8, 1, 9};
    std::sort(numbers.begin(), numbers.end());
    std::reverse(numbers.begin(), numbers.end());
    
    // Range-based for loop
    for (int num : numbers) {
        std::cout << num << " ";
    }
    
    return 0;
}
```

#### std::deque
```cpp
#include <deque>
#include <iostream>

int main() {
    std::deque<int> dq;
    
    // Add elements at both ends
    dq.push_back(10);
    dq.push_back(20);
    dq.push_front(5);
    dq.push_front(1);
    
    // Insert in the middle
    dq.insert(dq.begin() + 2, 15);
    
    // Remove from both ends
    dq.pop_front();
    dq.pop_back();
    
    // Access elements
    for (int num : dq) {
        std::cout << num << " ";
    }
    
    return 0;
}
```

#### std::list
```cpp
#include <list>
#include <iostream>

int main() {
    std::list<int> lst = {1, 2, 3, 4, 5};
    
    // Add elements
    lst.push_front(0);
    lst.push_back(6);
    lst.insert(++lst.begin(), 15);
    
    // Remove elements
    lst.remove(15);                    // Remove all occurrences of 15
    lst.erase(++lst.begin());           // Remove element at iterator position
    
    // Splice (move elements from another list)
    std::list<int> other = {100, 200, 300};
    lst.splice(lst.end(), other);      // Move all elements from other
    
    // Merge sorted lists
    lst.sort();
    other.sort();
    lst.merge(other);                   // other becomes empty
    
    // Iterate
    for (int num : lst) {
        std::cout << num << " ";
    }
    
    return 0;
}
```

#### std::array (C++11)
```cpp
#include <array>
#include <iostream>

int main() {
    std::array<int, 5> arr = {1, 2, 3, 4, 5};
    
    // Access elements
    std::cout << arr[0] << std::endl;    // 1
    std::cout << arr.at(0) << std::endl;  // 1 (bounds checking)
    
    // Size is always fixed
    std::cout << "Size: " << arr.size() << std::endl;
    
    // STL algorithms work
    std::sort(arr.begin(), arr.end());
    
    // Range-based for loop
    for (int num : arr) {
        std::cout << num << " ";
    }
    
    return 0;
}
```

### Associative Containers

#### std::map
```cpp
#include <map>
#include <string>
#include <iostream>

int main() {
    // Creation and initialization
    std::map<std::string, int> ages;
    std::map<std::string, int> scores = {
        {"Alice", 95},
        {"Bob", 87},
        {"Charlie", 92}
    };
    
    // Insert elements
    ages["John"] = 25;
    ages.insert({"Alice", 30});
    ages.emplace("Bob", 28);  // More efficient than insert
    
    // Access elements
    std::cout << "John's age: " << ages["John"] << std::endl;
    std::cout << "Alice's age: " << ages.at("Alice") << std::endl;
    
    // Check if key exists
    if (ages.find("Bob") != ages.end()) {
        std::cout << "Bob exists in the map" << std::endl;
    }
    
    // Safe access
    auto it = ages.find("Charlie");
    if (it != ages.end()) {
        std::cout << "Charlie's age: " << it->second << std::endl;
    }
    
    // Count occurrences (0 or 1 for map)
    if (ages.count("Alice")) {
        std::cout << "Alice exists" << std::endl;
    }
    
    // Iterate
    for (const auto& [name, age] : ages) {  // Structured bindings (C++17)
        std::cout << name << ": " << age << std::endl;
    }
    
    // Remove elements
    ages.erase("John");
    
    return 0;
}
```

#### std::unordered_map (C++11)
```cpp
#include <unordered_map>
#include <iostream>

int main() {
    std::unordered_map<std::string, int> word_counts;
    
    // Insert elements
    word_counts["hello"] = 1;
    word_counts["world"] = 2;
    word_counts.insert({"cpp", 3});
    
    // Access (same interface as map)
    std::cout << "hello count: " << word_counts["hello"] << std::endl;
    
    // Bucket information
    std::cout << "Bucket count: " << word_counts.bucket_count() << std::endl;
    std::cout << "Load factor: " << word_counts.load_factor() << std::endl;
    
    return 0;
}
```

#### std::set
```cpp
#include <set>
#include <iostream>

int main() {
    std::set<int> numbers;
    
    // Insert elements (automatically sorted, no duplicates)
    numbers.insert(5);
    numbers.insert(2);
    numbers.insert(8);
    numbers.insert(2);  // Duplicate ignored
    
    // Check existence
    if (numbers.count(5)) {
        std::cout << "5 exists in the set" << std::endl;
    }
    
    // Find element
    auto it = numbers.find(8);
    if (it != numbers.end()) {
        std::cout << "Found: " << *it << std::endl;
    }
    
    // Iterate (sorted order)
    for (int num : numbers) {
        std::cout << num << " ";
    }
    
    // Remove element
    numbers.erase(5);
    
    return 0;
}
```

### Container Adapters

#### std::stack
```cpp
#include <stack>
#include <iostream>

int main() {
    std::stack<int> st;
    
    // Push elements
    st.push(1);
    st.push(2);
    st.push(3);
    
    // Access top element
    std::cout << "Top: " << st.top() << std::endl;
    
    // Pop element
    st.pop();
    std::cout << "New top: " << st.top() << std::endl;
    
    // Check if empty
    while (!st.empty()) {
        std::cout << st.top() << " ";
        st.pop();
    }
    
    return 0;
}
```

#### std::queue
```cpp
#include <queue>
#include <iostream>

int main() {
    std::queue<int> q;
    
    // Enqueue elements
    q.push(1);
    q.push(2);
    q.push(3);
    
    // Access front element
    std::cout << "Front: " << q.front() << std::endl;
    std::cout << "Back: " << q.back() << std::endl;
    
    // Dequeue element
    q.pop();
    std::cout << "New front: " << q.front() << std::endl;
    
    return 0;
}
```

#### std::priority_queue
```cpp
#include <queue>
#include <iostream>

int main() {
    // Max-heap by default
    std::priority_queue<int> max_heap;
    max_heap.push(3);
    max_heap.push(1);
    max_heap.push(4);
    max_heap.push(2);
    
    while (!max_heap.empty()) {
        std::cout << max_heap.top() << " ";  // 4 3 2 1
        max_heap.pop();
    }
    
    // Min-heap
    std::priority_queue<int, std::vector<int>, std::greater<int>> min_heap;
    min_heap.push(3);
    min_heap.push(1);
    min_heap.push(4);
    min_heap.push(2);
    
    while (!min_heap.empty()) {
        std::cout << min_heap.top() << " ";  // 1 2 3 4
        min_heap.pop();
    }
    
    return 0;
}
```

## Algorithms

### Non-modifying Algorithms
```cpp
#include <algorithm>
#include <vector>
#include <iostream>

int main() {
    std::vector<int> numbers = {1, 2, 3, 4, 5, 3, 2, 1};
    
    // Find
    auto it = std::find(numbers.begin(), numbers.end(), 3);
    if (it != numbers.end()) {
        std::cout << "Found 3 at position: " << std::distance(numbers.begin(), it) << std::endl;
    }
    
    // Find if
    auto has_even = std::find_if(numbers.begin(), numbers.end(), 
                                [](int x) { return x % 2 == 0; });
    
    // Count
    int count_3 = std::count(numbers.begin(), numbers.end(), 3);
    std::cout << "Count of 3: " << count_3 << std::endl;
    
    // Count if
    int even_count = std::count_if(numbers.begin(), numbers.end(),
                                  [](int x) { return x % 2 == 0; });
    
    // All of, any of, none of
    bool all_positive = std::all_of(numbers.begin(), numbers.end(),
                                   [](int x) { return x > 0; });
    
    bool has_negative = std::any_of(numbers.begin(), numbers.end(),
                                   [](int x) { return x < 0; });
    
    bool no_zero = std::none_of(numbers.begin(), numbers.end(),
                                [](int x) { return x == 0; });
    
    // Search for subrange
    std::vector<int> pattern = {3, 4, 5};
    auto search_it = std::search(numbers.begin(), numbers.end(),
                                 pattern.begin(), pattern.end());
    
    return 0;
}
```

### Modifying Algorithms
```cpp
#include <algorithm>
#include <vector>
#include <iostream>

int main() {
    std::vector<int> numbers = {1, 2, 3, 4, 5};
    std::vector<int> destination(5);
    
    // Copy
    std::copy(numbers.begin(), numbers.end(), destination.begin());
    
    // Copy if
    std::vector<int> evens;
    std::copy_if(numbers.begin(), numbers.end(),
                 std::back_inserter(evens),
                 [](int x) { return x % 2 == 0; });
    
    // Transform
    std::vector<int> doubled;
    std::transform(numbers.begin(), numbers.end(),
                   std::back_inserter(doubled),
                   [](int x) { return x * 2; });
    
    // Fill
    std::vector<int> filled(5);
    std::fill(filled.begin(), filled.end(), 42);
    
    // Generate
    std::vector<int> generated(5);
    std::generate(generated.begin(), generated.end(),
                  []() { return rand() % 100; });
    
    // Replace
    std::vector<int> replace_example = {1, 2, 3, 2, 4, 2, 5};
    std::replace(replace_example.begin(), replace_example.end(), 2, 99);
    
    // Remove
    std::vector<int> remove_example = {1, 2, 3, 4, 5};
    auto new_end = std::remove(remove_example.begin(), remove_example.end(), 3);
    remove_example.erase(new_end, remove_example.end());
    
    // Unique
    std::vector<int> unique_example = {1, 2, 2, 3, 3, 3, 4};
    auto unique_end = std::unique(unique_example.begin(), unique_example.end());
    unique_example.erase(unique_end, unique_example.end());
    
    // Reverse
    std::reverse(numbers.begin(), numbers.end());
    
    // Shuffle
    std::random_device rd;
    std::mt19937 g(rd());
    std::shuffle(numbers.begin(), numbers.end(), g);
    
    return 0;
}
```

### Sorting Algorithms
```cpp
#include <algorithm>
#include <vector>
#include <iostream>

int main() {
    std::vector<int> numbers = {5, 2, 8, 1, 9, 3};
    
    // Sort (ascending)
    std::sort(numbers.begin(), numbers.end());
    
    // Sort (descending)
    std::sort(numbers.begin(), numbers.end(), std::greater<int>());
    
    // Sort with custom comparator
    std::sort(numbers.begin(), numbers.end(),
              [](int a, int b) { return a % 3 < b % 3; });
    
    // Partial sort
    std::vector<int> partial = {5, 2, 8, 1, 9, 3};
    std::partial_sort(partial.begin(), partial.begin() + 3, partial.end());
    
    // Nth element
    std::vector<int> nth = {5, 2, 8, 1, 9, 3};
    std::nth_element(nth.begin(), nth.begin() + 3, nth.end());
    // Element at position 3 is in its final sorted position
    
    // Binary search (requires sorted range)
    std::vector<int> sorted = {1, 2, 3, 4, 5, 6, 7, 8, 9};
    bool found = std::binary_search(sorted.begin(), sorted.end(), 5);
    
    auto lower = std::lower_bound(sorted.begin(), sorted.end(), 5);
    auto upper = std::upper_bound(sorted.begin(), sorted.end(), 5);
    
    // Merge
    std::vector<int> vec1 = {1, 3, 5};
    std::vector<int> vec2 = {2, 4, 6};
    std::vector<int> merged(6);
    std::merge(vec1.begin(), vec1.end(),
               vec2.begin(), vec2.end(),
               merged.begin());
    
    return 0;
}
```

## Iterators

### Iterator Types
```cpp
#include <vector>
#include <iostream>

int main() {
    std::vector<int> numbers = {1, 2, 3, 4, 5};
    
    // Input iterator (read-only)
    std::cout << "Input iterator: ";
    for (auto it = numbers.begin(); it != numbers.end(); ++it) {
        std::cout << *it << " ";
    }
    std::cout << std::endl;
    
    // Output iterator (write-only)
    std::vector<int> destination(5);
    std::copy(numbers.begin(), numbers.end(), destination.begin());
    
    // Forward iterator (read and write, forward only)
    std::cout << "Forward iterator: ";
    for (auto it = numbers.begin(); it != numbers.end(); ++it) {
        *it *= 2;  // Modify
        std::cout << *it << " ";
    }
    std::cout << std::endl;
    
    // Bidirectional iterator (can go backward)
    std::cout << "Bidirectional iterator (reverse): ";
    for (auto it = numbers.rbegin(); it != numbers.rend(); ++it) {
        std::cout << *it << " ";
    }
    std::cout << std::endl;
    
    // Random access iterator (can jump to any position)
    std::cout << "Random access: ";
    std::cout << numbers[2] << " ";  // Direct access
    std::cout << *(numbers.begin() + 2) << " ";  // Pointer arithmetic
    std::cout << std::endl;
    
    // Const iterator
    std::cout << "Const iterator: ";
    for (auto it = numbers.cbegin(); it != numbers.cend(); ++it) {
        std::cout << *it << " ";
        // *it = 10;  // Error: cannot modify through const_iterator
    }
    std::cout << std::endl;
    
    return 0;
}
```

### Iterator Adapters
```cpp
#include <vector>
#include <iterator>
#include <iostream>

int main() {
    std::vector<int> numbers = {1, 2, 3, 4, 5};
    
    // Back inserter
    std::vector<int> destination;
    std::copy(numbers.begin(), numbers.end(),
              std::back_inserter(destination));
    
    // Front inserter (requires container with push_front)
    std::deque<int> deque_dest;
    std::copy(numbers.begin(), numbers.end(),
              std::front_inserter(deque_dest));
    
    // Inserter (inserts at specific position)
    std::vector<int> insert_dest = {100, 200};
    std::copy(numbers.begin(), numbers.end(),
              std::inserter(insert_dest, insert_dest.begin() + 1));
    
    // Reverse iterator
    std::cout << "Reverse: ";
    std::copy(numbers.rbegin(), numbers.rend(),
              std::ostream_iterator<int>(std::cout, " "));
    std::cout << std::endl;
    
    // Move iterator (C++11)
    std::vector<std::string> strings = {"hello", "world", "cpp"};
    std::vector<std::string> moved_strings;
    std::move(strings.begin(), strings.end(),
              std::back_inserter(moved_strings));
    
    return 0;
}
```

## Function Objects

### Standard Function Objects
```cpp
#include <functional>
#include <algorithm>
#include <vector>
#include <iostream>

int main() {
    std::vector<int> numbers = {5, 2, 8, 1, 9};
    
    // Arithmetic operations
    std::transform(numbers.begin(), numbers.end(), numbers.begin(),
                  std::negate<int>());
    
    std::vector<int> doubled(numbers.size());
    std::transform(numbers.begin(), numbers.end(), doubled.begin(),
                  std::bind2nd(std::multiplies<int>(), 2));
    
    // Comparisons
    std::sort(numbers.begin(), numbers.end(), std::greater<int>());
    
    // Logical operations
    std::vector<bool> flags = {true, false, true, false};
    std::transform(flags.begin(), flags.end(), flags.begin(),
                  std::logical_not<bool>());
    
    return 0;
}
```

### Custom Function Objects
```cpp
#include <functional>
#include <algorithm>
#include <vector>
#include <iostream>

struct MultiplyBy {
    int factor;
    
    MultiplyBy(int f) : factor(f) {}
    
    int operator()(int x) const {
        return x * factor;
    }
};

struct IsEven {
    bool operator()(int x) const {
        return x % 2 == 0;
    }
};

int main() {
    std::vector<int> numbers = {1, 2, 3, 4, 5};
    
    // Use custom function object
    std::transform(numbers.begin(), numbers.end(), numbers.begin(),
                  MultiplyBy(3));
    
    // Use with algorithm
    auto even_it = std::find_if(numbers.begin(), numbers.end(), IsEven());
    
    return 0;
}
```

## Best Practices
- Choose the right container for your use case
- Use `std::vector` as default sequence container
- Use `std::map` for ordered key-value pairs
- Use `std::unordered_map` for faster lookups when order doesn't matter
- Use range-based for loops when possible (C++11+)
- Use algorithms instead of manual loops
- Prefer `std::array` over C-style arrays for fixed-size containers
- Use `std::move` for efficient transfers of ownership
- Use `std::back_inserter` for dynamic insertion
- Be aware of iterator invalidation rules
- Use const iterators when not modifying
