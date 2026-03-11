# Halo Language Syntax Guide

## Overview

Halo is a minimal, dynamically-typed programming language designed for educational purposes. It features a clean, Python-like syntax without explicit type declarations or semicolons.

## Key Features

- **No Type Declarations**: Variables are created with simple assignment
- **No `fn` Keyword**: Functions are declared with just `name() { ... }`
- **No Semicolons**: Statements are terminated by newlines
- **Mandatory Blocks**: `if`, `else`, `while` use `{ }` for scope
- **Dynamic Typing**: Types are inferred at runtime
- **Simple Recursion**: Full support for recursive functions

## Variables

### Declaration and Initialization

Variables are created implicitly with assignment:

```halo
x = 5
name = "Hello"
active = true
price = 99.99
```

Variables can be reassigned:

```halo
contador = 0
contador = contador + 1
```

## Data Types (Inferred)

Halo infers types from values:

- **Integer**: `42`, `-10`
- **Float**: `3.14`, `99.99`
- **Boolean**: `true`, `false`
- **String**: `"Hello"` (planned)

## Operators

### Arithmetic Operators

```halo
a = 10 + 5      // Addition: 15
b = 10 - 5      // Subtraction: 5
c = 10 * 5      // Multiplication: 50
d = 10 / 5      // Division: 2
```

### Comparison Operators

```halo
a == b          // Equal
a != b          // Not equal
a < b           // Less than
a > b           // Greater than
a <= b          // Less than or equal
a >= b          // Greater than or equal
```

### Logical Operators (Planned)

```halo
a && b          // AND
a || b          // OR
!a              // NOT
```

## Control Flow

### If / Else Statements

Blocks are mandatory with `{ }`:

```halo
if edad >= 18 {
    // Code for adults
} else {
    // Code for minors
}
```

Multiple conditions:

```halo
if x == 0 {
    // x is zero
} else {
    if x > 0 {
        // x is positive
    } else {
        // x is negative
    }
}
```

### While Loops

```halo
while contador < 10 {
    contador = contador + 1
}
```

Nested loops:

```halo
while i < 5 {
    while j < 3 {
        j = j + 1
    }
    i = i + 1
}
```

## Functions

### Basic Function Declaration

No `fn` keyword needed:

```halo
saludar() {
    // Function body
}
```

### Functions with Parameters

Parameters are comma-separated, no type declarations:

```halo
sumar(a, b) {
    return a + b
}

multiplicar(x, y) {
    resultado = x * y
    return resultado
}
```

### Return Statements

```halo
factorial(n) {
    if n == 1 {
        return 1
    }
    return n * factorial(n - 1)
}
```

Functions can return without a value (returns implicit `null`):

```halo
printMessage() {
    // No return statement
}
```

### Calling Functions

```halo
result = sumar(5, 3)

factorial(5)

multiplicar(10, 20)
```

Nested calls:

```halo
a = sumar(multiplicar(2, 3), 4)  // (2 * 3) + 4 = 10
```

## Program Structure

A Halo program consists of top-level function and variable declarations:

```halo
// Global variable (optional)
MAX_VALOR = 100

// Function declarations
factorial(n) {
    if n == 1 {
        return 1
    }
    return n * factorial(n - 1)
}

main() {
    result = factorial(5)
    return result
}
```

The `main()` function is the entry point (when implemented).

## Complete Example Programs

### Factorial

```halo
factorial(n) {
    if n == 1 {
        return 1
    }
    return n * factorial(n - 1)
}

main() {
    result = factorial(5)
    return result
}
```

### Fibonacci Sequence

```halo
fibonacci(n) {
    if n <= 1 {
        return n
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}

main() {
    i = 0
    while i < 10 {
        fib = fibonacci(i)
        i = i + 1
    }
}
```

### Even Numbers

```halo
main() {
    numero = 1
    
    while numero <= 10 {
        esPar = (numero % 2) == 0
        
        if esPar == true {
            // print(numero)
        }
        
        numero = numero + 1
    }
}
```

### Simple Recursion

```halo
suma(n) {
    if n == 0 {
        return 0
    }
    return n + suma(n - 1)
}

main() {
    total = suma(10)
    return total
}
```

## Comments

Single-line comments use `//`:

```halo
x = 5  // This is a comment

// This is also a comment
y = 10
```

Multi-line comments (planned):

```halo
/*
This is a multi-line comment
spanning several lines
*/
```

## Statement Termination

- **NO semicolons** at end of statements
- **Newlines** mark the end of a statement
- **Blocks** use `{ }` for scope
- End of file also terminates statements

Valid:

```halo
x = 5
y = 10
```

Invalid (would cause parse error):

```halo
x = 5;      // Semicolon not needed (but might be ignored)
y = 10;     // Same
```

## Expressions

Expressions are evaluated left-to-right with standard operator precedence:

```halo
// Precedence: * / > + - > comparisons > equality

a = 2 + 3 * 4      // 14 (not 20)
b = (2 + 3) * 4    // 20 (parentheses override)

c = 5 < 10 == true // true
d = x == 5 != false // depends on x
```

## Blocks

Blocks group statements and create scope:

```halo
if condition {
    // Block 1
    x = 5
    y = 10
} else {
    // Block 2
    x = 0
    y = 0
}

while x < 100 {
    // Block 3
    x = x + 1
}

funcion() {
    // Block 4
    return x
}
```

## Built-in Functions (Planned)

When implemented, Halo will support:

```halo
print(x)           // Print to console
len(s)             // Length of string/array
str(x)             // Convert to string
int(x)             // Convert to integer
float(x)           // Convert to float
```

## Roadmap Features

### Phase 2: Type System
- Type inference
- Optional type annotations
- Type checking

### Phase 3: Advanced Control Flow
- `break` and `continue` in loops
- `switch` statements (planned)

### Phase 4: Data Structures
- Arrays/Lists
- Dictionaries/Objects
- String manipulation

### Phase 5: Standard Library
- File I/O
- Math functions
- String utilities

## Differences from Other Languages

| Feature | Halo | Python | JavaScript | C |
|---------|------|--------|------------|---|
| Type Declaration | No | No | No | Yes |
| Semicolons | No | No | Yes | Yes |
| `fn` keyword | No | No | No | Yes |
| Blocks with `{}` | Yes | No | Yes | Yes |
| Dynamic Typing | Yes | Yes | Yes | No |

## Error Handling (Planned)

Future versions will support:

```halo
try {
    // Code that might fail
} catch error {
    // Handle error
}
```

## Best Practices

1. **Use descriptive variable names**:
   ```halo
   contador = 0       // Good
   c = 0              // Less clear
   ```

2. **Indent consistently** (2 or 4 spaces):
   ```halo
   if x > 0 {
       y = x + 1
       if y > 10 {
           z = y * 2
       }
   }
   ```

3. **Add comments for complex logic**:
   ```halo
   // Calculate nth Fibonacci number
   fib(n) {
       if n <= 1 {
           return n
       }
       return fib(n - 1) + fib(n - 2)
   }
   ```

4. **Avoid deep nesting**:
   ```halo
   // Hard to read
   if a {
       if b {
           if c {
               // Too nested
           }
       }
   }
   
   // Better
   if a && b && c {
       // Clearer intent
   }
   ```

---

For more information, see:
- `README.md` - Project overview
- `init.txt` - Initial specification
- `examples/` - Example programs