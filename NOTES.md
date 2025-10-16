# 📝 Rust Study Labs - Notes & Key Points

Comprehensive notes, key concepts, and important takeaways 

---

# Variables and Mutability

## Shadowing

**Definition:** Shadowing in Rust means redeclaring a variable with the same name, allowing you to change its value or even its type while keeping the same variable name in scope.

**Example:**
```rust
fn main() {
    let x = 5;
    let x = x + 1;  // shadowing the previous x
    let x = x * 2;  // shadowing again

    println!("The value of x is: {}", x);  // prints 12
}
```

**Key Points:**
- Each `let` creates a new variable that shadows the previous one
- The old value is no longer accessible, but the name `x` is reused
- Shadowing is different from mutation

**Shadowing vs Mutability:**

```rust
// ✅ Shadowing allows type change
let spaces = "   ";
let spaces = spaces.len();  // now an integer - OK!

// ❌ Mutability cannot change type
let mut spaces = "   ";
spaces = spaces.len();  // compile error - type mismatch!
```

**Why Use Shadowing?**
- Transform values while keeping the same variable name
- Change the type of a variable without creating a new name
- Maintain immutability while allowing transformations

---

# Data Types

> **Important:** Rust is a **statically typed** language, which means that it must know the types of all variables at compile time.

Every value in Rust has a specific data type that tells Rust what kind of data is being specified. Rust has two main categories: **scalar** and **compound** types.

---

## Scalar Types

Scalar types represent a single value. Rust has four primary scalar types:

1. **Integers**
2. **Floating-point numbers**
3. **Booleans**
4. **Characters**

---

### 1. Integer Types

An integer is a number without a fractional component.

**Integer Type Reference:**

| Length | Signed | Unsigned | Range (Signed) | Range (Unsigned) |
|--------|--------|----------|----------------|------------------|
| 8-bit  | `i8`   | `u8`     | -128 to 127    | 0 to 255         |
| 16-bit | `i16`  | `u16`    | -32,768 to 32,767 | 0 to 65,535   |
| 32-bit | `i32`  | `u32`    | -2³¹ to 2³¹-1  | 0 to 2³²-1       |
| 64-bit | `i64`  | `u64`    | -2⁶³ to 2⁶³-1  | 0 to 2⁶⁴-1       |
| 128-bit| `i128` | `u128`   | -2¹²⁷ to 2¹²⁷-1| 0 to 2¹²⁸-1      |
| arch   | `isize`| `usize`  | Depends on architecture | Depends on architecture |

**Key Points:**
- **Signed** (`i`) integers can store negative numbers
- **Unsigned** (`u`) integers can only store positive numbers
- `isize` and `usize` depend on the architecture (32-bit or 64-bit)
- Default integer type is `i32`

**Integer Literals:**

Number literals can use:
- **Type suffix** to designate the type: `57u8`
- **Underscore** (`_`) as a visual separator: `1_000` (same as `1000`)

**Number Literal Formats:**

| Literal Type | Syntax | Example | Decimal Value | Notes |
|--------------|--------|---------|---------------|-------|
| **Decimal** | `[0-9_]+` | `98_222` | 98,222 | Underscores ignored, improve readability |
| **Hexadecimal** | `0x[0-9a-fA-F_]+` | `0xff` | 255 | Base-16, case-insensitive |
| **Octal** | `0o[0-7_]+` | `0o77` | 63 | Base-8 |
| **Binary** | `0b[01_]+` | `0b1111_0000` | 240 | Base-2 |
| **Byte** | `b'[char]'` | `b'A'` | 65 | `u8` only, ASCII character |

**Examples:**
```rust
let decimal = 98_222;
let hex = 0xff;
let octal = 0o77;
let binary = 0b1111_0000;
let byte = b'A';
```

---

### 2. Floating-Point Types

Rust has two primitive floating-point types for numbers with decimal points:

| Type | Size | Precision |
|------|------|-----------|
| `f32` | 32 bits | Single precision |
| `f64` | 64 bits | Double precision |

**Key Points:**
- Default type is `f64` (roughly same speed as `f32` on modern CPUs, but more precision)
- All floating-point types are **signed**
- Follow IEEE-754 standard

**Example:**
```rust
fn main() {
    let x = 2.0;        // f64 (default)
    let y: f32 = 3.0;   // f32 (explicit annotation)
}
```

---

### 3. The Boolean Type

Boolean type has two possible values: `true` and `false`.

**Key Points:**
- Type annotation: `bool`
- One byte in size
- Used in conditionals and logical operations

**Example:**
```rust
fn main() {
    let t = true;                // inferred type
    let f: bool = false;         // explicit type annotation
}
```

---

### 4. The Character Type

Rust's `char` type represents a Unicode Scalar Value.

**Key Points:**
- Specified with **single quotes**: `'z'`
- Four bytes in size
- Can represent more than just ASCII: emoji, accented letters, Chinese/Japanese/Korean characters, etc.

**Example:**
```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ';              // with explicit type annotation
    let heart_eyed_cat = '😻';      // Unicode emoji
}
```

---

## Compound Types

Compound types can group multiple values into one type. Rust has two primitive compound types:

1. **Tuples**
2. **Arrays**

---

### 1. The Tuple Type

**Definition:** A tuple is a way of grouping together a number of values with a variety of types into one compound type.

**Key Characteristics:**
- **Fixed length**: once declared, cannot grow or shrink
- Can contain values of **different types**
- Created using parentheses with comma-separated values

**Creating Tuples:**

```rust
fn main() {
    // With explicit type annotation
    let tup1: (i32, f64, u8) = (500, 6.4, 1);

    // Type inference
    let tup2 = (500, 6.4, 1);
}
```

**Destructuring:**

```rust
fn main() {
    let tup = (500, 6.4, 1);
    
    // Destructure into individual variables
    let (x, y, z) = tup;
    
    println!("The value of y is: {y}");
}
```

**Accessing Elements:**

```rust
fn main() {
    let a: (i32, f64, u8) = (500, 6.4, 1);
    
    let five_hundred = a.0;     // access first element
    let six_point_four = a.1;   // access second element
    let one = a.2;              // access third element
}
```

**Special Tuple - Unit Type:**

The tuple without any values is called **unit**.

```rust
let unit = ();  // unit value and unit type
```

**Key Points:**
- Written as `()`
- Represents an empty value or empty return type
- Implicitly returned when a function doesn't return anything

---

### 2. The Array Type

**Definition:** An array is a collection of multiple values of the **same type**.

**Key Characteristics:**
- **Fixed length**: cannot grow or shrink after declaration
- All elements must be the **same type**
- Allocated on the **stack** (unlike vectors which use the heap)

**Creating Arrays:**

```rust
fn main() {
    // Basic array
    let a = [1, 2, 3, 4, 5];
    
    // With explicit type annotation
    // [type; length]
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    
    // Initialize with same value
    // [value; length]
    let a = [3; 5];  // same as [3, 3, 3, 3, 3]
}
```

**Array Type Syntax:**
```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
//     ^^^ ^^^ 
//     |   |
//     |   length (must be known at compile time)
//     element type
```

**Initialization Shorthand:**
```rust
let a = [3; 5];  // creates [3, 3, 3, 3, 3]
//       ^ ^
//       | |
//       | length
//       initial value
```

**Accessing Array Elements:**
```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    
    let first = a[0];   // 1
    let second = a[1];  // 2
}
```

**Slicing Array Elements:**
```rust
let a = [1, 2, 3, 4, 5];
// upper idx 4 is excluded
let nice_slice = &a[1..4];
// The upper index can be included by using the syntax `..=` (with `=` sign)
let nice_slice = &a[1..=3];
```

**When to Use Arrays vs Vectors:**
- Use **arrays** when you know the number of elements won't change
- Use **vectors** (covered later) when you need a growable collection

---
# Functions

*Syntax*
```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

## Parameters
```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
```

## Statements and Expressions

Function bodies are made up of a series of statements optionally ending in an expression.

Statements are instructions that perform some action and do not return a value.
Expressions evaluate to a resultant value.


```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
```

This expression:

```rust
{
    let x = 3;
    x + 1
}
```

is a block that, in this case, evaluates to 4. That value gets bound to y as part of the let statement. Note that the x + 1 line *doesn’t have a semicolon at the end*, which is unlike most of the lines you’ve seen so far. *Expressions do not include ending semicolons*. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.

## Functions with return values

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}
```

# Control Flow

## if Expressions

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
```

## Repetition with Loops

Rust has three kinds of loops: loop, while, and for. Let’s try each one.

### Repeating Code with *loop*

```rust
fn main() {
    loop {
        println!("again!");
    }
}
```
*When we run this program, we’ll see again! printed over and over continuously until we stop the program manually.*

### Loop Labels to Disambiguate Between Multiple Loops
```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```

### Conditional Loops with while
```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

### Looping Through a Collection
```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```
```rust
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

// 3!
// 2!
// 1!
// LIFTOFF!!!
```
# Rust Ownership

## Stack vs Heap
- **Stack**: Fast LIFO storage for fixed-size data (integers, booleans, etc.)
- **Heap**: Slower storage for variable-size data (String, Vec, etc.) - returns a pointer

## The Three Ownership Rules
1. Each value has exactly **one owner**
2. Only **one owner** at a time
3. When owner goes **out of scope**, value is **dropped** (memory freed)

---

## Key Operations

### Move (Default Behavior)
```rust
let s1 = String::from("hello");
let s2 = s1;  // s1 is MOVED to s2
// println!("{}", s1); // ❌ ERROR: s1 no longer valid
```
- Transfers ownership from `s1` to `s2`
- Only the stack data (pointer, length, capacity) is copied
- Heap data is NOT duplicated
- `s1` becomes invalid to prevent double-free errors

### Clone (Explicit Deep Copy)
```rust
let s1 = String::from("hello");
let s2 = s1.clone();  // Deep copy of heap data
println!("{} {}", s1, s2); // ✅ Both valid
```
- Creates a full copy of both stack AND heap data
- Expensive operation - use sparingly
- Both variables remain valid

### Clear (Mutation)
```rust
let mut s = String::from("hello");
s.clear();  // Empties the string, s = ""
```
- Requires mutable reference (`mut`)
- Modifies existing data without transferring ownership
- The variable remains valid but empty

---

## Borrowing (References)

### Immutable References (`&T`)
```rust
let s1 = String::from("hello");
print_length(&s1);  // Borrows without taking ownership
println!("{}", s1); // ✅ s1 still valid
```
- Read-only access
- Can have **multiple** immutable references simultaneously
- Original owner retains ownership

### Mutable References (`&mut T`)
```rust
let mut s = String::from("hello");
change(&mut s);
```
- Allows modification
- Can have **only ONE** mutable reference at a time
- Cannot mix mutable and immutable references

### Reference Rules
1. Either **one mutable** reference OR **any number of immutable** references
2. References must **always be valid** (no dangling pointers)

---

## String Slices (`&str`)

```rust
let s = String::from("hello world");
let hello = &s[0..5];   // "hello"
let world = &s[6..11];  // "world"
let whole = &s[..];     // "hello world"
```

- A reference to part of a String
- Type: `&str` (immutable reference)
- Does NOT take ownership
- Syntax: `&s[start..end]` or `&s[..]` for full string

### Why Slices Matter
```rust
let mut s = String::from("hello");
let word = &s[0..5];
// s.clear(); // ❌ ERROR: can't mutate while immutable borrow exists
println!("{}", word);
```
- Slices prevent unsafe mutations
- Compiler enforces that data can't change while borrowed

---

## Quick Decision Chart

**Need to use value in multiple places?**
- Stack types (i32, bool, etc.) → Auto-copied ✓
- Heap types (String, Vec) → Use references `&` or `.clone()`

**Need to modify data?**
- Own it → Use `let mut` and modify directly
- Borrow it → Use `&mut` reference

**Passing to functions?**
- Don't need it after → Move (default)
- Still need it → Borrow with `&`

---

## Common Patterns

```rust
// ✅ Multiple reads
let s = String::from("data");
read(&s);
read(&s);
println!("{}", s);

// ✅ Single mutation
let mut s = String::from("data");
modify(&mut s);
println!("{}", s);

// ❌ Can't do both
let mut s = String::from("data");
let r1 = &s;
// let r2 = &mut s; // ERROR: already borrowed as immutable
```

---

## Memory Model

```
Stack (fast, fixed-size):
┌─────────┬────────┬──────────┐
│ pointer │ length │ capacity │  → This is what gets copied on move
└────┬────┴────────┴──────────┘
     │
     ↓
Heap (slower, variable-size):
┌───┬───┬───┬───┬───┐
│ h │ e │ l │ l │ o │  → Actual string data (expensive to copy)
└───┴───┴───┴───┴───┘
```

When you **move**: Only stack data is copied (cheap)  
When you **clone**: Stack + heap data is copied (expensive)  
When you **borrow**: Nothing is copied, just a reference (cheapest)

---

## Key Takeaways
- Ownership prevents memory bugs at compile time
- Move is default for heap types (no copy)
- Use `&` to borrow without taking ownership
- Use `.clone()` when you explicitly need a copy
- Slices (`&str`) are safe views into data

---

# Rust Structs

## Basic Struct Definition
```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

## Debug Printing
- `#[derive(Debug)]` - Auto-generates debug formatting
- `{:?}` - Compact debug print
- `{:#?}` - Pretty debug print
- `dbg!(&user)` - Prints value with file/line info (use `&` to avoid moving ownership)

## Display Trait
Must implement `fmt::Display` manually to use `{}` formatting:
```rust
impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Hello, {}. Active: {}", self.username, self.active)
    }
}
```

## Struct Update Syntax
```rust
let user2 = User {
    email: String::from("new@test.com"),
    ..user1  // Copy remaining fields from user1
};
```

## Tuple Structs
```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
// Same types but different struct names = different types
```

## Methods vs Associated Functions
```rust
impl Rectangle {
    // Method (takes &self)
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // Associated function (no self) - like a constructor
    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}
```

## Key Differences
- **Method**: Has `&self` parameter, called with dot notation: `rect.area()`
- **Associated Function**: No `self`, called with `::`: `Rectangle::square(10)`

## Borrowing Best Practices
- Use `&Rectangle` instead of `Rectangle` in functions to:
  - Avoid taking ownership
  - Avoid copying large structs
  - Maintain read-only access

## Ownership Notes
- Use `String` instead of `&str` in structs to avoid lifetime issues
- `&str` requires lifetime specifiers (borrowed data must not outlive the struct)

---

*More chapters and notes will be added as I progress through the book...*