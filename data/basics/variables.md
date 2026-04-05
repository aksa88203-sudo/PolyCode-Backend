# C Variables

## Overview
Variables are containers for storing data values in C programming.

## Variable Declaration and Initialization

### Basic Declaration
```c
int age;
float salary;
char grade;
double pi;
```

### Declaration with Initialization
```c
int age = 25;
float salary = 50000.50;
char grade = 'A';
double pi = 3.14159;
```

## Data Types

### Integer Types
```c
char ch;          // 1 byte
short s;          // 2 bytes
int i;            // 4 bytes
long l;           // 4 or 8 bytes
long long ll;     // 8 bytes
```

### Floating-Point Types
```c
float f;          // 4 bytes
double d;         // 8 bytes
long double ld;   // 16 bytes
```

### Modifiers
```c
unsigned int ui;  // Non-negative integers
signed int si;    // Can be positive or negative
const int ci = 10; // Constant value
```

## Variable Scope

### Local Variables
```c
void function() {
    int local_var = 10; // Only accessible within this function
}
```

### Global Variables
```c
int global_var = 100; // Accessible throughout the program

void function() {
    global_var = 200; // Can modify global variable
}
```

## Best Practices
- Use descriptive variable names
- Initialize variables before use
- Use appropriate data types
- Limit scope of variables when possible
- Use `const` for values that don't change
