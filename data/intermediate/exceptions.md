# C++ Exceptions

## Exception Basics

### Throwing and Catching Exceptions
```cpp
#include <iostream>
#include <stdexcept>

void divide(int a, int b) {
    if (b == 0) {
        throw std::runtime_error("Division by zero!");
    }
    std::cout << "Result: " << a / b << std::endl;
}

int main() {
    try {
        divide(10, 2);  // Works fine
        divide(10, 0);  // Throws exception
    } catch (const std::runtime_error& e) {
        std::cout << "Error: " << e.what() << std::endl;
    }
    
    return 0;
}
```

### Standard Exception Hierarchy
```cpp
#include <iostream>
#include <stdexcept>
#include <vector>
#include <memory>

void demonstrateExceptions() {
    // std::exception - Base class
    try {
        throw std::exception();
    } catch (const std::exception& e) {
        std::cout << "Base exception: " << e.what() << std::endl;
    }
    
    // std::logic_error - Logic errors
    try {
        throw std::invalid_argument("Invalid argument provided");
    } catch (const std::invalid_argument& e) {
        std::cout << "Invalid argument: " << e.what() << std::endl;
    }
    
    try {
        throw std::out_of_range("Index out of range");
    } catch (const std::out_of_range& e) {
        std::cout << "Out of range: " << e.what() << std::endl;
    }
    
    // std::runtime_error - Runtime errors
    try {
        throw std::overflow_error("Arithmetic overflow");
    } catch (const std::overflow_error& e) {
        std::cout << "Overflow: " << e.what() << std::endl;
    }
    
    try {
        throw std::bad_alloc();
    } catch (const std::bad_alloc& e) {
        std::cout << "Bad allocation: " << e.what() << std::endl;
    }
}
```

## Custom Exception Classes

### Creating Custom Exceptions
```cpp
#include <iostream>
#include <stdexcept>
#include <string>

class BankException : public std::exception {
private:
    std::string message;
    
public:
    explicit BankException(const std::string& msg) : message(msg) {}
    
    const char* what() const noexcept override {
        return message.c_str();
    }
};

class InsufficientFundsException : public BankException {
public:
    InsufficientFundsException(double balance, double amount)
        : BankException("Insufficient funds: balance $" + 
                       std::to_string(balance) + ", attempted $" + 
                       std::to_string(amount)) {}
};

class AccountNotFoundException : public BankException {
public:
    explicit AccountNotFoundException(const std::string& accountNumber)
        : BankException("Account not found: " + accountNumber) {}
};

class BankAccount {
private:
    std::string accountNumber;
    double balance;
    
public:
    BankAccount(const std::string& accNum, double initialBalance)
        : accountNumber(accNum), balance(initialBalance) {}
    
    void withdraw(double amount) {
        if (amount > balance) {
            throw InsufficientFundsException(balance, amount);
        }
        balance -= amount;
    }
    
    void deposit(double amount) {
        if (amount <= 0) {
            throw std::invalid_argument("Deposit amount must be positive");
        }
        balance += amount;
    }
    
    double getBalance() const { return balance; }
    const std::string& getAccountNumber() const { return accountNumber; }
};

int main() {
    BankAccount account("12345", 100.0);
    
    try {
        account.deposit(50.0);
        std::cout << "New balance: $" << account.getBalance() << std::endl;
        
        account.withdraw(200.0);  // This will throw
    } catch (const InsufficientFundsException& e) {
        std::cout << "Bank error: " << e.what() << std::endl;
    } catch (const std::invalid_argument& e) {
        std::cout << "Invalid argument: " << e.what() << std::endl;
    } catch (const std::exception& e) {
        std::cout << "General error: " << e.what() << std::endl;
    }
    
    return 0;
}
```

## Exception Safety

### RAII and Exception Safety
```cpp
#include <iostream>
#include <memory>
#include <fstream>

class FileHandler {
private:
    std::unique_ptr<std::ofstream> file;
    
public:
    explicit FileHandler(const std::string& filename) 
        : file(std::make_unique<std::ofstream>(filename)) {
        if (!file->is_open()) {
            throw std::runtime_error("Cannot open file: " + filename);
        }
        std::cout << "File opened: " << filename << std::endl;
    }
    
    ~FileHandler() {
        if (file && file->is_open()) {
            file->close();
            std::cout << "File closed" << std::endl;
        }
    }
    
    void write(const std::string& data) {
        if (!file || !file->is_open()) {
            throw std::runtime_error("File is not open");
        }
        *file << data;
    }
    
    // Delete copy operations
    FileHandler(const FileHandler&) = delete;
    FileHandler& operator=(const FileHandler&) = delete;
    
    // Allow move operations
    FileHandler(FileHandler&&) = default;
    FileHandler& operator=(FileHandler&&) = default;
};

void processFile(const std::string& filename) {
    FileHandler handler(filename);
    handler.write("Hello, World!");
    
    // If an exception occurs here, handler's destructor will close the file
    throw std::runtime_error("Simulated error during processing");
}

int main() {
    try {
        processFile("output.txt");
    } catch (const std::exception& e) {
        std::cout << "Error: " << e.what() << std::endl;
    }
    
    return 0;
}
```

### Exception Safety Levels
```cpp
#include <iostream>
#include <vector>
#include <algorithm>

class UnsafeArray {
private:
    int* data;
    size_t size;
    
public:
    UnsafeArray(size_t s) : size(s), data(new int[s]) {}
    
    ~UnsafeArray() { delete[] data; }
    
    // No exception safety - can leak memory
    void unsafe_copy(const UnsafeArray& other) {
        delete[] data;  // Delete old data
        data = new int[other.size];  // Might throw, leaving data invalid
        std::copy(other.data, other.data + other.size, data);
        size = other.size;
    }
};

class BasicSafeArray {
private:
    int* data;
    size_t size;
    
public:
    BasicSafeArray(size_t s) : size(s), data(new int[s]) {}
    
    ~BasicSafeArray() { delete[] data; }
    
    // Basic exception safety - no leaks, but object may be invalid
    void basic_safe_copy(const BasicSafeArray& other) {
        int* new_data = new int[other.size];  // Might throw
        try {
            std::copy(other.data, other.data + other.size, new_data);
            delete[] data;
            data = new_data;
            size = other.size;
        } catch (...) {
            delete[] new_data;  // Clean up on exception
            throw;  // Re-throw
        }
    }
};

class StrongSafeArray {
private:
    std::unique_ptr<int[]> data;
    size_t size;
    
public:
    StrongSafeArray(size_t s) : size(s), data(std::make_unique<int[]>(s)) {}
    
    // Strong exception safety - either complete success or no change
    void strong_safe_copy(const StrongSafeArray& other) {
        auto new_data = std::make_unique<int[]>(other.size);
        std::copy(other.data.get(), other.data.get() + other.size, new_data.get());
        
        // All operations succeeded, now commit
        data = std::move(new_data);
        size = other.size;
    }
    
    size_t getSize() const { return size; }
    int& operator[](size_t index) { return data[index]; }
    const int& operator[](size_t index) const { return data[index]; }
};
```

## Exception Handling Patterns

### Function Try Blocks
```cpp
#include <iostream>

class MyClass {
private:
    int value;
    
public:
    // Function try block for constructor
    MyClass(int v) try : value(v) {
        if (v < 0) {
            throw std::invalid_argument("Value cannot be negative");
        }
        std::cout << "MyClass constructed with value: " << value << std::endl;
    } catch (const std::exception& e) {
        std::cout << "Constructor failed: " << e.what() << std::endl;
        // Re-throw to propagate the exception
        throw;
    }
    
    // Function try block for regular function
    void process() try {
        if (value > 100) {
            throw std::out_of_range("Value too large");
        }
        std::cout << "Processing value: " << value << std::endl;
    } catch (const std::exception& e) {
        std::cout << "Processing failed: " << e.what() << std::endl;
        throw;
    }
};

int main() {
    try {
        MyClass obj1(50);
        obj1.process();
        
        MyClass obj2(-10);  // Will throw
    } catch (const std::exception& e) {
        std::cout << "Caught exception: " << e.what() << std::endl;
    }
    
    return 0;
}
```

### Nested Exception Handling
```cpp
#include <iostream>
#include <stdexcept>
#include <exception>

void innerFunction() {
    throw std::runtime_error("Inner function error");
}

void middleFunction() {
    try {
        innerFunction();
    } catch (const std::exception& e) {
        std::cout << "Middle function caught: " << e.what() << std::endl;
        // Wrap and re-throw with additional context
        std::throw_with_nested(std::runtime_error("Middle function failed"));
    }
}

void outerFunction() {
    try {
        middleFunction();
    } catch (const std::exception& e) {
        std::cout << "Outer function caught: " << e.what() << std::endl;
        
        // Print nested exceptions
        try {
            std::rethrow_if_nested(e);
        } catch (const std::exception& nested) {
            std::cout << "Nested exception: " << nested.what() << std::endl;
        }
        
        throw;
    }
}

int main() {
    try {
        outerFunction();
    } catch (const std::exception& e) {
        std::cout << "Main caught: " << e.what() << std::endl;
    }
    
    return 0;
}
```

### Exception Specifications (C++11 and later)
```cpp
#include <iostream>
#include <stdexcept>

// noexcept specification - function promises not to throw
void safeFunction() noexcept {
    std::cout << "This function promises not to throw" << std::endl;
}

// Conditional noexcept
void conditionalSafeFunction(bool safe) noexcept(safe) {
    if (!safe) {
        throw std::runtime_error("Unsafe operation");
    }
    std::cout << "This function is conditionally noexcept" << std::endl;
}

// noexcept operator - compile-time check
template<typename T>
void processElement(T element) noexcept(noexcept(element.process())) {
    element.process();
}

class Processable {
public:
    void process() noexcept {
        std::cout << "Processing element" << std::endl;
    }
};

class RiskyProcessable {
public:
    void process() {
        throw std::runtime_error("Risky processing");
    }
};

int main() {
    safeFunction();
    
    try {
        conditionalSafeFunction(false);  // Will call terminate
    } catch (...) {
        std::cout << "Caught exception from conditional safe function" << std::endl;
    }
    
    Processable safe;
    processElement(safe);  // OK, process() is noexcept
    
    RiskyProcessable risky;
    // processElement(risky);  // Compile error: processElement is not noexcept
    
    return 0;
}
```

## Modern C++ Exception Features

### std::optional and Exceptions (C++17)
```cpp
#include <iostream>
#include <optional>
#include <string>

// Function that might fail without throwing
std::optional<int> divide_safe(int a, int b) noexcept {
    if (b == 0) {
        return std::nullopt;  // Return empty optional
    }
    return a / b;
}

// Function that throws on error
int divide_unsafe(int a, int b) {
    if (b == 0) {
        throw std::runtime_error("Division by zero");
    }
    return a / b;
}

int main() {
    // Using std::optional for error handling
    auto result1 = divide_safe(10, 2);
    if (result1) {
        std::cout << "Result: " << *result1 << std::endl;
    } else {
        std::cout << "Division failed" << std::endl;
    }
    
    auto result2 = divide_safe(10, 0);
    if (result2) {
        std::cout << "Result: " << *result2 << std::endl;
    } else {
        std::cout << "Division failed" << std::endl;
    }
    
    // Using value_or for default values
    int result3 = divide_safe(10, 0).value_or(-1);
    std::cout << "Result with default: " << result3 << std::endl;
    
    // Traditional exception handling
    try {
        int result4 = divide_unsafe(10, 2);
        std::cout << "Result: " << result4 << std::endl;
    } catch (const std::exception& e) {
        std::cout << "Error: " << e.what() << std::endl;
    }
    
    return 0;
}
```

### std::variant and Exceptions (C++17)
```cpp
#include <iostream>
#include <variant>
#include <string>
#include <vector>

// Function that can return different types or error
std::variant<int, std::string, std::vector<int>> processData(int input) {
    if (input < 0) {
        return std::string("Negative input not allowed");
    } else if (input == 0) {
        return std::vector<int>{1, 2, 3};
    } else {
        return input * 2;
    }
}

int main() {
    auto result = processData(5);
    
    // Visit with lambda that handles all types
    std::visit([](auto&& arg) {
        using T = std::decay_t<decltype(arg)>;
        if constexpr (std::is_same_v<T, int>) {
            std::cout << "Got integer: " << arg << std::endl;
        } else if constexpr (std::is_same_v<T, std::string>) {
            std::cout << "Got string: " << arg << std::endl;
        } else if constexpr (std::is_same_v<T, std::vector<int>>) {
            std::cout << "Got vector: ";
            for (int v : arg) {
                std::cout << v << " ";
            }
            std::cout << std::endl;
        }
    }, result);
    
    // Check specific type
    if (std::holds_alternative<int>(result)) {
        std::cout << "Integer value: " << std::get<int>(result) << std::endl;
    }
    
    return 0;
}
```

## Performance Considerations

### Exception Overhead
```cpp
#include <iostream>
#include <chrono>

// Traditional error code approach
int divide_error_code(int a, int b, int& result) {
    if (b == 0) {
        return -1;  // Error code
    }
    result = a / b;
    return 0;  // Success
}

// Exception-based approach
int divide_exception(int a, int b) {
    if (b == 0) {
        throw std::runtime_error("Division by zero");
    }
    return a / b;
}

void benchmarkErrorCodes() {
    const int iterations = 1000000;
    int result;
    int success_count = 0;
    
    auto start = std::chrono::high_resolution_clock::now();
    
    for (int i = 0; i < iterations; ++i) {
        if (divide_error_code(10, i % 2 + 1, result) == 0) {
            success_count++;
        }
    }
    
    auto end = std::chrono::high_resolution_clock::now();
    auto duration = std::chrono::duration_cast<std::chrono::microseconds>(end - start);
    
    std::cout << "Error codes: " << duration.count() << " microseconds" << std::endl;
    std::cout << "Success count: " << success_count << std::endl;
}

void benchmarkExceptions() {
    const int iterations = 1000000;
    int success_count = 0;
    
    auto start = std::chrono::high_resolution_clock::now();
    
    for (int i = 0; i < iterations; ++i) {
        try {
            divide_exception(10, i % 2 + 1);
            success_count++;
        } catch (...) {
            // Handle exception
        }
    }
    
    auto end = std::chrono::high_resolution_clock::now();
    auto duration = std::chrono::duration_cast<std::chrono::microseconds>(end - start);
    
    std::cout << "Exceptions: " << duration.count() << " microseconds" << std::endl;
    std::cout << "Success count: " << success_count << std::endl;
}

int main() {
    benchmarkErrorCodes();
    benchmarkExceptions();
    
    return 0;
}
```

## Best Practices

### Exception Handling Guidelines
```cpp
#include <iostream>
#include <memory>
#include <vector>
#include <fstream>

// GOOD: Use RAII for resource management
class DatabaseConnection {
private:
    bool connected;
    
public:
    DatabaseConnection() : connected(false) {
        // Connect to database
        connected = true;
        std::cout << "Database connected" << std::endl;
    }
    
    ~DatabaseConnection() {
        if (connected) {
            // Disconnect from database
            connected = false;
            std::cout << "Database disconnected" << std::endl;
        }
    }
    
    void execute(const std::string& query) {
        if (!connected) {
            throw std::runtime_error("Not connected to database");
        }
        std::cout << "Executing: " << query << std::endl;
    }
};

// GOOD: Create custom exception hierarchy
class ApplicationException : public std::exception {
protected:
    std::string message;
    
public:
    explicit ApplicationException(const std::string& msg) : message(msg) {}
    const char* what() const noexcept override { return message.c_str(); }
};

class DatabaseException : public ApplicationException {
public:
    explicit DatabaseException(const std::string& msg) 
        : ApplicationException("Database error: " + msg) {}
};

// GOOD: Use exceptions for exceptional circumstances
void processData(const std::string& filename) {
    std::ifstream file(filename);
    if (!file.is_open()) {
        throw std::runtime_error("Cannot open file: " + filename);
    }
    
    // Process file...
}

// GOOD: Catch specific exceptions first
void handleErrors() {
    try {
        processData("nonexistent.txt");
    } catch (const DatabaseException& e) {
        std::cout << "Database error: " << e.what() << std::endl;
    } catch (const std::ios_base::failure& e) {
        std::cout << "IO error: " << e.what() << std::endl;
    } catch (const std::exception& e) {
        std::cout << "General error: " << e.what() << std::endl;
    } catch (...) {
        std::cout << "Unknown error occurred" << std::endl;
    }
}

// GOOD: Use noexcept for functions that won't throw
int add(int a, int b) noexcept {
    return a + b;
}

// GOOD: Use std::optional for functions that might not return a value
std::optional<int> findValue(const std::vector<int>& vec, int target) {
    for (int value : vec) {
        if (value == target) {
            return value;
        }
    }
    return std::nullopt;
}

int main() {
    // RAII example
    try {
        DatabaseConnection db;
        db.execute("SELECT * FROM users");
        throw std::runtime_error("Simulated error");
    } catch (const std::exception& e) {
        std::cout << "Caught: " << e.what() << std::endl;
    }
    
    // Error handling example
    handleErrors();
    
    // Optional example
    std::vector<int> numbers = {1, 2, 3, 4, 5};
    auto result = findValue(numbers, 3);
    if (result) {
        std::cout << "Found: " << *result << std::endl;
    } else {
        std::cout << "Not found" << std::endl;
    }
    
    return 0;
}
```

## Best Practices Summary
- Use exceptions for exceptional circumstances, not for normal control flow
- Create custom exception classes that inherit from `std::exception`
- Use RAII for automatic resource cleanup
- Catch exceptions by reference to const
- Order catch blocks from most specific to most general
- Use `noexcept` for functions that won't throw exceptions
- Consider `std::optional` (C++17) for functions that might not return a value
- Use `std::variant` (C++17) for functions that can return different types
- Be aware of the performance cost of exceptions
- Don't throw exceptions from destructors
- Use exception specifications appropriately
- Log exceptions for debugging purposes
- Provide meaningful error messages in exceptions
- Use smart pointers to prevent memory leaks during exceptions
- Consider using error codes for performance-critical code paths
