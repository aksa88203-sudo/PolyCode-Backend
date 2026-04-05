
---

### **functions/parameters.md**
```md
# Function Parameters in C

Functions can take input values called parameters.

## Example
```c
#include <stdio.h>

void greetPerson(char name[]) {
    printf("Hello, %s!\n", name);
}

int main() {
    greetPerson("Alice");
    greetPerson("Bob");
    return 0;
}
##Practice

Create a function multiply that takes two integers as parameters and prints their product.
