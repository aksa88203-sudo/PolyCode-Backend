# C++ Templates

## Function Templates

### Basic Function Template
```cpp
#include <iostream>

// Basic template function
template<typename T>
T maximum(T a, T b) {
    return (a > b) ? a : b;
}

int main() {
    int max_int = maximum(5, 3);           // T = int
    double max_double = maximum(3.14, 2.71); // T = double
    char max_char = maximum('a', 'z');     // T = char
    
    std::cout << "Max int: " << max_int << std::endl;
    std::cout << "Max double: " << max_double << std::endl;
    std::cout << "Max char: " << max_char << std::endl;
    
    return 0;
}
```

### Multiple Template Parameters
```cpp
template<typename T, typename U>
auto add(T t, U u) -> decltype(t + u) {
    return t + u;
}

int main() {
    auto result1 = add(5, 3.14);    // double
    auto result2 = add(5.0, 3);     // double
    auto result3 = add(5, 3);      // int
    
    return 0;
}
```

### Template with Non-type Parameters
```cpp
template<typename T, int SIZE>
class FixedArray {
private:
    T data[SIZE];
    
public:
    T& operator[](int index) {
        return data[index];
    }
    
    const T& operator[](int index) const {
        return data[index];
    }
    
    int size() const {
        return SIZE;
    }
};

int main() {
    FixedArray<int, 5> int_array;
    FixedArray<double, 10> double_array;
    
    for (int i = 0; i < 5; i++) {
        int_array[i] = i * 2;
    }
    
    return 0;
}
```

## Class Templates

### Basic Class Template
```cpp
template<typename T>
class Stack {
private:
    std::vector<T> elements;
    
public:
    void push(const T& element) {
        elements.push_back(element);
    }
    
    T pop() {
        if (elements.empty()) {
            throw std::runtime_error("Stack is empty");
        }
        T top = elements.back();
        elements.pop_back();
        return top;
    }
    
    bool empty() const {
        return elements.empty();
    }
    
    size_t size() const {
        return elements.size();
    }
};

int main() {
    Stack<int> int_stack;
    Stack<std::string> string_stack;
    
    int_stack.push(1);
    int_stack.push(2);
    int_stack.push(3);
    
    string_stack.push("Hello");
    string_stack.push("World");
    
    while (!int_stack.empty()) {
        std::cout << int_stack.pop() << std::endl;
    }
    
    return 0;
}
```

### Template Specialization

#### Full Specialization
```cpp
template<typename T>
class Calculator {
public:
    T add(T a, T b) {
        return a + b;
    }
};

// Full specialization for bool
template<>
class Calculator<bool> {
public:
    bool add(bool a, bool b) {
        return a || b;  // Logical OR for booleans
    }
};

int main() {
    Calculator<int> int_calc;
    Calculator<bool> bool_calc;
    
    std::cout << int_calc.add(5, 3) << std::endl;    // 8
    std::cout << bool_calc.add(true, false) << std::endl; // true
    
    return 0;
}
```

#### Partial Specialization
```cpp
template<typename T>
class Container {
public:
    void store(const T& item) {
        std::cout << "Storing generic item" << std::endl;
    }
};

// Partial specialization for pointer types
template<typename T>
class Container<T*> {
public:
    void store(T* item) {
        std::cout << "Storing pointer item" << std::endl;
    }
};

int main() {
    Container<int> int_container;
    Container<int*> int_ptr_container;
    
    int value = 42;
    int_container.store(value);      // Storing generic item
    int_ptr_container.store(&value); // Storing pointer item
    
    return 0;
}
```

## Template Parameters

### Default Template Parameters
```cpp
template<typename T, typename Allocator = std::allocator<T>>
class CustomVector {
private:
    T* data;
    size_t size;
    Allocator allocator;
    
public:
    explicit CustomVector(size_t s = 0) : size(s) {
        data = allocator.allocate(s);
    }
    
    ~CustomVector() {
        allocator.deallocate(data, size);
    }
    
    T& operator[](size_t index) {
        return data[index];
    }
    
    const T& operator[](size_t index) const {
        return data[index];
    }
};

int main() {
    CustomVector<int> default_vector(10);      // Uses default allocator
    CustomVector<int, std::allocator<int>> explicit_vector(10);
    
    return 0;
}
```

### Template Template Parameters
```cpp
template<typename T, template<typename, typename> class Container>
class Adapter {
private:
    Container<T, std::allocator<T>> container;
    
public:
    void add(const T& item) {
        container.push_back(item);
    }
    
    void print() {
        for (const auto& item : container) {
            std::cout << item << " ";
        }
        std::cout << std::endl;
    }
};

int main() {
    Adapter<int, std::vector> vector_adapter;
    vector_adapter.add(1);
    vector_adapter.add(2);
    vector_adapter.add(3);
    vector_adapter.print();
    
    return 0;
}
```

## Advanced Template Concepts

### SFINAE (Substitution Failure Is Not An Error)
```cpp
#include <type_traits>

template<typename T>
typename std::enable_if<std::is_integral<T>::value, T>::type
double_value(T value) {
    return value * 2;
}

template<typename T>
typename std::enable_if<std::is_floating_point<T>::value, T>::type
double_value(T value) {
    return value * 2.0;
}

int main() {
    std::cout << double_value(5) << std::endl;      // 10 (int version)
    std::cout << double_value(3.14) << std::endl;    // 6.28 (double version)
    
    return 0;
}
```

### Variadic Templates (C++11)
```cpp
#include <iostream>

// Variadic function template
template<typename... Args>
void print(Args... args) {
    ((std::cout << args << " "), ...);  // Fold expression (C++17)
    std::cout << std::endl;
}

// Count arguments
template<typename... Args>
constexpr size_t count_args() {
    return sizeof...(Args);
}

// Sum all arguments
template<typename... Args>
auto sum(Args... args) {
    return (args + ...);  // Fold expression (C++17)
}

int main() {
    print(1, 2.5, "hello", 'a');  // 1 2.5 hello a
    std::cout << "Count: " << count_args<int, double, std::string>() << std::endl; // 3
    std::cout << "Sum: " << sum(1, 2, 3, 4, 5) << std::endl; // 15
    
    return 0;
}
```

### Perfect Forwarding
```cpp
template<typename T>
class Wrapper {
private:
    T data;
    
public:
    // Perfect forwarding constructor
    template<typename U>
    explicit Wrapper(U&& arg) : data(std::forward<U>(arg)) {}
    
    // Perfect forwarding method
    template<typename U>
    void set_data(U&& arg) {
        data = std::forward<U>(arg);
    }
    
    const T& get_data() const {
        return data;
    }
};

int main() {
    int value = 42;
    
    Wrapper<int> wrapper1(value);        // Copy
    Wrapper<int> wrapper2(std::move(value)); // Move
    
    wrapper1.set_data(100);              // Copy
    wrapper2.set_data(200);              // Move
    
    return 0;
}
```

## Modern C++ Template Features

### Concepts (C++20)
```cpp
#include <concepts>

// Define a concept
template<typename T>
concept Numeric = std::is_integral_v<T> || std::is_floating_point_v<T>;

// Use concept in function template
template<Numeric T>
T multiply(T a, T b) {
    return a * b;
}

// Constrained template parameter
template<typename T>
requires Numeric<T>
T divide(T a, T b) {
    return a / b;
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

### requires Expression (C++20)
```cpp
template<typename T>
concept HasBeginEnd = requires(T t) {
    typename T::iterator;
    { t.begin() } -> std::same_as<typename T::iterator>;
    { t.end() } -> std::same_as<typename T::iterator>;
};

template<typename T>
requires HasBeginEnd<T>
void process(T&& container) {
    std::cout << "Container has begin and end" << std::endl;
}

// Local requires
template<typename T>
void advanced_process(T&& container) {
    requires HasBeginEnd<T>;
    std::cout << "Processing container" << std::endl;
}
```

## Template Metaprogramming

### Compile-time Computations
```cpp
template<int N>
struct Factorial {
    static constexpr int value = N * Factorial<N - 1>::value;
};

template<>
struct Factorial<0> {
    static constexpr int value = 1;
};

// Compile-time factorial function (C++11)
constexpr int factorial(int n) {
    return (n <= 1) ? 1 : n * factorial(n - 1);
}

// Template recursion for power
template<int base, int exp>
struct Power {
    static constexpr int value = base * Power<base, exp - 1>::value;
};

template<int base>
struct Power<base, 0> {
    static constexpr int value = 1;
};

int main() {
    constexpr int fact5 = Factorial<5>::value;  // 120
    constexpr int fact5_func = factorial(5);    // 120
    constexpr int pow2_3 = Power<2, 3>::value;  // 8
    
    return 0;
}
```

### Type Traits
```cpp
template<typename T>
struct is_pointer {
    static constexpr bool value = false;
};

template<typename T>
struct is_pointer<T*> {
    static constexpr bool value = true;
};

template<typename T>
constexpr bool is_pointer_v = is_pointer<T>::value;

// Remove pointer
template<typename T>
struct remove_pointer {
    using type = T;
};

template<typename T>
struct remove_pointer<T*> {
    using type = T;
};

template<typename T>
using remove_pointer_t = typename remove_pointer<T>::type;

int main() {
    static_assert(is_pointer_v<int*> == true);
    static_assert(is_pointer_v<int> == false);
    
    static_assert(std::is_same_v<remove_pointer_t<int*>, int>);
    
    return 0;
}
```

## Best Practices
- Use templates for generic algorithms and data structures
- Prefer concepts over SFINAE when available (C++20)
- Use `auto` and `decltype` for return type deduction
- Use perfect forwarding for constructor and function parameters
- Keep templates simple and readable
- Use static assertions to validate template constraints
- Consider template specialization for type-specific optimizations
- Use variadic templates for flexible interfaces
- Document template requirements and constraints
- Be aware of compilation time impact of complex templates
