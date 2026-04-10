# Modern C++ Features

## C++11 Features

### Auto Type Deduction
```cpp
#include <vector>
#include <map>
#include <iostream>

int main() {
    // Basic auto usage
    auto i = 42;                    // int
    auto d = 3.14;                  // double
    auto s = "hello";               // const char*
    
    // With containers
    std::vector<int> vec = {1, 2, 3};
    auto it = vec.begin();          // std::vector<int>::iterator
    
    // Range-based for loop
    for (auto element : vec) {
        std::cout << element << " ";
    }
    
    // With complex types
    std::map<std::string, int> m = {{"one", 1}, {"two", 2}};
    auto pair = *m.begin();         // std::pair<const std::string, int>
    
    return 0;
}
```

### Range-based For Loops
```cpp
#include <vector>
#include <array>
#include <iostream>

int main() {
    std::vector<int> vec = {1, 2, 3, 4, 5};
    std::array<int, 3> arr = {10, 20, 30};
    
    // Non-reference iteration
    for (auto x : vec) {
        std::cout << x << " ";  // x is a copy
    }
    
    // Reference iteration (can modify)
    for (auto& x : vec) {
        x *= 2;                  // Modifies elements
    }
    
    // Const reference iteration (read-only)
    for (const auto& x : vec) {
        std::cout << x << " ";  // Efficient, read-only
    }
    
    return 0;
}
```

### Lambda Expressions
```cpp
#include <algorithm>
#include <vector>
#include <iostream>

int main() {
    std::vector<int> numbers = {5, 2, 8, 1, 9};
    
    // Basic lambda
    auto square = [](int x) { return x * 2; };
    
    // Lambda with capture
    int multiplier = 3;
    auto multiply = [multiplier](int x) { return x * multiplier; };
    
    // Lambda with capture by reference
    int sum = 0;
    auto accumulate = [&sum](int x) { sum += x; };
    
    // Use with algorithms
    std::transform(numbers.begin(), numbers.end(), numbers.begin(), square);
    std::for_each(numbers.begin(), numbers.end(), accumulate);
    
    std::cout << "Sum: " << sum << std::endl;
    
    // Generic lambda (C++14)
    auto print = [](const auto& value) {
        std::cout << value << " ";
    };
    
    std::for_each(numbers.begin(), numbers.end(), print);
    
    return 0;
}
```

### Smart Pointers
```cpp
#include <memory>
#include <iostream>

class Resource {
public:
    Resource() { std::cout << "Resource created\n"; }
    ~Resource() { std::cout << "Resource destroyed\n"; }
    void use() { std::cout << "Using resource\n"; }
};

int main() {
    // unique_ptr - exclusive ownership
    auto unique_res = std::make_unique<Resource>();
    unique_res->use();
    
    // shared_ptr - shared ownership
    auto shared_res1 = std::make_shared<Resource>();
    auto shared_res2 = shared_res1;  // Reference count = 2
    
    std::cout << "Use count: " << shared_res1.use_count() << std::endl;
    
    // weak_ptr - non-owning reference
    std::weak_ptr<Resource> weak_res = shared_res1;
    
    return 0;
}
```

### Move Semantics
```cpp
#include <vector>
#include <string>
#include <iostream>

class Movable {
private:
    std::string data;
    int* ptr;
    
public:
    Movable(const std::string& s) : data(s), ptr(new int(42)) {
        std::cout << "Constructor\n";
    }
    
    // Copy constructor
    Movable(const Movable& other) : data(other.data), ptr(new int(*other.ptr)) {
        std::cout << "Copy constructor\n";
    }
    
    // Move constructor
    Movable(Movable&& other) noexcept : data(std::move(other.data)), ptr(other.ptr) {
        other.ptr = nullptr;
        std::cout << "Move constructor\n";
    }
    
    ~Movable() {
        delete ptr;
        std::cout << "Destructor\n";
    }
};

Movable create_movable() {
    Movable obj("test");
    return obj;  // Return value optimization
}

int main() {
    Movable obj1 = create_movable();  // Move
    Movable obj2 = std::move(obj1);  // Explicit move
    
    std::vector<Movable> vec;
    vec.emplace_back("hello");  // Emplace - construct in place
    
    return 0;
}
```

### constexpr Functions
```cpp
#include <iostream>

// Compile-time function
constexpr int factorial(int n) {
    return (n <= 1) ? 1 : n * factorial(n - 1);
}

// Compile-time computation
constexpr int fact5 = factorial(5);

int main() {
    std::cout << "Factorial of 5: " << fact5 << std::endl;
    
    // Can also be used at runtime
    int n;
    std::cin >> n;
    std::cout << "Factorial of " << n << ": " << factorial(n) << std::endl;
    
    return 0;
}
```

## C++14 Features

### Generic Lambdas
```cpp
#include <iostream>
#include <vector>

int main() {
    // Generic lambda with auto parameters
    auto print = [](const auto& value) {
        std::cout << value << std::endl;
    };
    
    print(42);
    print(3.14);
    print("Hello");
    
    // Generic lambda with return type deduction
    auto add = [](const auto& a, const auto& b) {
        return a + b;
    };
    
    std::cout << add(5, 3) << std::endl;
    std::cout << add(2.5, 1.5) << std::endl;
    
    return 0;
}
```

### Return Type Deduction
```cpp
#include <vector>
#include <algorithm>

// Return type deduced as int
auto add(int a, int b) {
    return a + b;
}

// Return type deduced as iterator
auto find_max(std::vector<int>& vec) {
    return std::max_element(vec.begin(), vec.end());
}

// Trailing return type still useful for complex cases
auto get_element(std::vector<int>& vec, size_t index) -> int& {
    return vec[index];
}

int main() {
    std::vector<int> numbers = {1, 5, 3, 9, 2};
    auto max_it = find_max(numbers);
    
    std::cout << "Max: " << *max_it << std::endl;
    
    return 0;
}
```

### Variable Templates
```cpp
#include <iostream>

// Variable template
template<typename T>
constexpr T pi = T(3.1415926535897932385);

// Specialization
template<>
constexpr float pi<float> = 3.14159f;

template<typename T>
T area_of_circle(T radius) {
    return pi<T> * radius * radius;
}

int main() {
    std::cout << "Pi (double): " << pi<double> << std::endl;
    std::cout << "Pi (float): " << pi<float> << std::endl;
    
    std::cout << "Area (radius 2.0): " << area_of_circle(2.0) << std::endl;
    
    return 0;
}
```

## C++17 Features

### Structured Bindings
```cpp
#include <tuple>
#include <map>
#include <array>
#include <iostream>

std::tuple<int, std::string, double> get_person() {
    return {25, "John", 1.75};
}

int main() {
    // With tuple
    auto [age, name, height] = get_person();
    std::cout << name << " is " << age << " years old" << std::endl;
    
    // With map iteration
    std::map<int, std::string> students = {{1, "Alice"}, {2, "Bob"}};
    
    for (const auto& [id, name] : students) {
        std::cout << "ID: " << id << ", Name: " << name << std::endl;
    }
    
    // With array
    std::array<int, 3> coords = {10, 20, 30};
    auto [x, y, z] = coords;
    
    return 0;
}
```

### std::optional
```cpp
#include <optional>
#include <string>
#include <iostream>

std::optional<int> divide(int a, int b) {
    if (b == 0) {
        return std::nullopt;
    }
    return a / b;
}

std::optional<std::string> find_user(int id) {
    if (id == 1) {
        return "Alice";
    } else if (id == 2) {
        return "Bob";
    }
    return std::nullopt;
}

int main() {
    auto result1 = divide(10, 2);
    if (result1) {
        std::cout << "Result: " << *result1 << std::endl;
    }
    
    auto result2 = divide(10, 0);
    if (!result2) {
        std::cout << "Division by zero" << std::endl;
    }
    
    // Using value_or
    int safe_result = divide(10, 0).value_or(-1);
    std::cout << "Safe result: " << safe_result << std::endl;
    
    // With strings
    auto user = find_user(1);
    std::cout << "User: " << user.value_or("Unknown") << std::endl;
    
    return 0;
}
```

### std::variant
```cpp
#include <variant>
#include <string>
#include <iostream>
#include <vector>

std::variant<int, double, std::string> process_data(int input) {
    if (input < 0) {
        return std::string("Negative input");
    } else if (input == 0) {
        return std::vector<int>{1, 2, 3};
    } else {
        return input * 2;
    }
}

int main() {
    auto result = process_data(5);
    
    // Using std::visit
    std::visit([](const auto& value) {
        using T = std::decay_t<decltype(value)>;
        if constexpr (std::is_same_v<T, int>) {
            std::cout << "Integer: " << value << std::endl;
        } else if constexpr (std::is_same_v<T, double>) {
            std::cout << "Double: " << value << std::endl;
        } else if constexpr (std::is_same_v<T, std::string>) {
            std::cout << "String: " << value << std::endl;
        }
    }, result);
    
    // Using holds_alternative
    if (std::holds_alternative<int>(result)) {
        std::cout << "Contains int: " << std::get<int>(result) << std::endl;
    }
    
    return 0;
}
```

### std::string_view
```cpp
#include <string_view>
#include <string>
#include <iostream>

void print_string(std::string_view sv) {
    std::cout << sv << std::endl;
}

std::string_view get_extension(std::string_view filename) {
    auto pos = filename.find_last_of('.');
    if (pos == std::string_view::npos) {
        return "";
    }
    return filename.substr(pos + 1);
}

int main() {
    std::string str = "Hello, World!";
    std::string_view sv = str;
    
    print_string(sv);
    print_string("Literal string");
    
    std::cout << "Extension: " << get_extension("document.txt") << std::endl;
    std::cout << "Extension: " << get_extension("archive") << std::endl;
    
    return 0;
}
```

### if with Initializer
```cpp
#include <map>
#include <string>
#include <iostream>

int main() {
    std::map<int, std::string> data = {{1, "one"}, {2, "two"}};
    
    // Traditional way
    auto it = data.find(1);
    if (it != data.end()) {
        std::cout << "Found: " << it->second << std::endl;
    }
    
    // C++17 way
    if (auto it = data.find(2); it != data.end()) {
        std::cout << "Found: " << it->second << std::endl;
    }
    
    // With switch
    switch (int value = 42; value) {
        case 42:
            std::cout << "The answer" << std::endl;
            break;
        default:
            std::cout << "Something else" << std::endl;
    }
    
    return 0;
}
```

## C++20 Features

### Concepts
```cpp
#include <concepts>
#include <iostream>
#include <vector>

// Define a concept
template<typename T>
concept Numeric = std::is_integral_v<T> || std::is_floating_point_v<T>;

// Use concept in function template
template<Numeric T>
T multiply(T a, T b) {
    return a * b;
}

// Concept with multiple requirements
template<typename T>
concept Container = requires(T t) {
    typename T::value_type;
    typename T::iterator;
    { t.begin() } -> std::same_as<typename T::iterator>;
    { t.end() } -> std::same_as<typename T::iterator>;
};

template<Container T>
void print_container(const T& container) {
    for (const auto& item : container) {
        std::cout << item << " ";
    }
    std::cout << std::endl;
}

int main() {
    auto result1 = multiply(5, 3);      // OK
    auto result2 = multiply(3.14, 2.0); // OK
    
    std::vector<int> vec = {1, 2, 3, 4, 5};
    print_container(vec);               // OK
    
    return 0;
}
```

### Ranges
```cpp
#include <ranges>
#include <vector>
#include <iostream>
#include <algorithm>

int main() {
    std::vector<int> numbers = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
    
    // Filter even numbers
    auto evens = numbers | std::views::filter([](int n) { return n % 2 == 0; });
    
    // Transform to squares
    auto squares = evens | std::views::transform([](int n) { return n * n; });
    
    // Take first 3
    auto first_three = squares | std::views::take(3);
    
    for (int n : first_three) {
        std::cout << n << " ";  // 4 16 36
    }
    std::cout << std::endl;
    
    // Compose views
    auto result = numbers 
        | std::views::filter([](int n) { return n > 5; })
        | std::views::transform([](int n) { return n * 2; });
    
    for (int n : result) {
        std::cout << n << " ";  // 12 14 16 18 20
    }
    
    return 0;
}
```

### Coroutines
```cpp
#include <coroutine>
#include <iostream>

// Simple generator using coroutines
template<typename T>
struct Generator {
    struct promise_type {
        T current_value;
        
        Generator get_return_object() {
            return Generator{std::coroutine_handle<promise_type>::from_promise(*this)};
        }
        
        std::suspend_always initial_suspend() { return {}; }
        std::suspend_always final_suspend() noexcept { return {}; }
        
        std::suspend_always yield_value(T value) {
            current_value = value;
            return {};
        }
        
        void return_void() {}
        void unhandled_exception() {}
    };
    
    std::coroutine_handle<promise_type> h;
    
    Generator(std::coroutine_handle<promise_type> handle) : h(handle) {}
    
    ~Generator() { if (h) h.destroy(); }
    
    bool next() {
        h.resume();
        return !h.done();
    }
    
    T value() {
        return h.promise().current_value;
    }
};

Generator<int> count_to(int n) {
    for (int i = 1; i <= n; ++i) {
        co_yield i;
    }
}

int main() {
    auto gen = count_to(5);
    
    while (gen.next()) {
        std::cout << gen.value() << " ";
    }
    std::cout << std::endl;
    
    return 0;
}
```

### Three-way Comparison (Spaceship Operator)
```cpp
#include <compare>
#include <iostream>

struct Point {
    int x, y;
    
    auto operator<=>(const Point& other) const = default;
    
    // Custom comparison
    std::strong_ordering operator<=>(const Point& other) const {
        if (auto cmp = x <=> other.x; cmp != 0) return cmp;
        return y <=> other.y;
    }
};

int main() {
    Point p1{1, 2};
    Point p2{1, 3};
    Point p3{1, 2};
    
    if (p1 < p2) std::cout << "p1 < p2\n";
    if (p1 == p3) std::cout << "p1 == p3\n";
    if (p2 > p1) std::cout << "p2 > p1\n";
    
    // Comparison result
    auto cmp = p1 <=> p2;
    if (cmp < 0) std::cout << "p1 is less\n";
    if (cmp > 0) std::cout << "p1 is greater\n";
    if (cmp == 0) std::cout << "p1 is equal\n";
    
    return 0;
}
```

## Modern Best Practices

### Modern C++ Style Guide
```cpp
#include <vector>
#include <string>
#include <memory>
#include <algorithm>
#include <ranges>

class ModernClass {
private:
    std::string name_;
    std::vector<int> data_;
    
public:
    // Use member initializer list
    explicit ModernClass(std::string name) : name_(std::move(name)) {}
    
    // Use default for special members when appropriate
    ModernClass(const ModernClass&) = default;
    ModernClass& operator=(const ModernClass&) = default;
    ModernClass(ModernClass&&) = default;
    ModernClass& operator=(ModernClass&&) = default;
    ~ModernClass() = default;
    
    // Use const correctness
    const std::string& name() const { return name_; }
    
    // Use smart pointers
    std::unique_ptr<ModernClass> clone() const {
        return std::make_unique<ModernClass>(*this);
    }
    
    // Use algorithms and ranges
    void process_data() {
        // Modern C++20 ranges
        auto processed = data_ 
            | std::views::filter([](int n) { return n > 0; })
            | std::views::transform([](int n) { return n * 2; });
        
        data_.assign(processed.begin(), processed.end());
    }
    
    // Use concepts for templates
    template<std::integral T>
    void add_number(T value) {
        data_.push_back(static_cast<int>(value));
    }
};

// Use auto and structured bindings
auto create_objects() {
    std::vector<std::unique_ptr<ModernClass>> objects;
    objects.push_back(std::make_unique<ModernClass>("Object1"));
    objects.push_back(std::make_unique<ModernClass>("Object2"));
    return objects;
}

int main() {
    auto objects = create_objects();
    
    for (const auto& obj : objects) {
        std::cout << obj->name() << std::endl;
    }
    
    return 0;
}
```

## Best Practices Summary
- Use `auto` for type deduction when the type is obvious
- Prefer range-based for loops over traditional loops
- Use smart pointers instead of raw pointers
- Use `constexpr` for compile-time computations
- Leverage move semantics for performance
- Use structured bindings for cleaner code (C++17)
- Use `std::optional` for functions that may not return values (C++17)
- Use concepts for template constraints (C++20)
- Prefer algorithms over manual loops
- Use `std::string_view` for read-only string operations (C++17)
- Use `if` with initializer for cleaner scoping (C++17)
- Consider ranges for functional-style programming (C++20)
- Use the spaceship operator for comparisons (C++20)
- Keep up with modern C++ standards and idioms
