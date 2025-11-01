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

# Rust Enums

## Basic Enum with Data
```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```
- Enums can hold data directly in variants
- Each variant can have different types and amounts of data

## Option<T> - Safe Null Alternative
```rust
enum Option<T> {
    Some(T),  // Value exists
    None,     // Value is absent
}
```

### Benefits
- **No null pointer errors** - forces explicit handling of missing values
- **Compile-time safety** - must handle both `Some` and `None` cases
- **Clear intent** - makes optional values explicit in function signatures

### Common Methods
```rust
let user = find_user(1);

// Method 1: match
match user {
    Some(name) => println!("Found: {}", name),
    None => println!("Not found"),
}

// Method 2: unwrap_or (provides default)
let name = find_user(2).unwrap_or(String::from("Guest"));
```

## Pattern Matching with match
```rust
fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```
- Must be **exhaustive** - all cases must be handled
- Returns a value from each arm

## if let - Syntactic Sugar
Use when you only care about **one specific pattern**:

```rust
// Verbose match
match config_max {
    Some(max) => println!("Max: {max}"),
    _ => (),  // Required boilerplate
}

// Concise if let
if let Some(max) = config_max {
    println!("Max: {max}");
}
```

### With else
```rust
if let Coin::Quarter = coin {
    println!("Quarter");
} else {
    count += 1;  // All other coins
}
```

## let-else Pattern (Early Return)
```rust
fn describe_cents(coin: Coin) -> Option<String> {
    let Coin::Quarter = coin else {
        return None;  // Exit early if not Quarter
    };
    Some(format!("Quarter it is"))
}
```
- Useful for **guard clauses** and early returns
- Cleaner than nested if statements

## Key Takeaways
- **Enums over structs** when variants are mutually exclusive
- **Option<T>** is Rust's safe replacement for null
- **match** for exhaustive pattern matching
- **if let** for single-pattern matching
- **let-else** for early-return guard patterns
---

# Rust Generics, Traits & Lifetimes - Key Notes

## Generics

### Generic Functions with Trait Bounds
```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```
- `T: PartialOrd` is a **trait bound** - restricts `T` to types that can be compared
- Enables code reuse across different types (numbers, chars, etc.)

## Traits (Similar to Interfaces)

### Defining a Trait
```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")  // Default implementation
    }
}
```

### Implementing a Trait
```rust
impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

### Traits as Parameters

**Syntactic Sugar:**
```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

**Trait Bound (Full Syntax):**
```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

### Multiple Trait Bounds
```rust
// Multiple traits with +
pub fn notify<T: Summary + Display>(item: &T) { }

// Alternative syntax
pub fn notify(item: &(impl Summary + Display)) { }

// where clause for complex bounds (cleaner)
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // function body
}
```

## Lifetimes

### Why Lifetimes?
Prevent **dangling references** - references that point to invalid memory.

### Lifetime Error Example
```rust
// ❌ WRONG
let r;
{
    let x = 5;
    r = &x;  // ERROR: x will be dropped, r becomes dangling
}
println!("r: {r}");  // r points to deallocated memory

// ✅ CORRECT
let x = 5;
let r = &x;  // Both live in same scope
println!("r: {r}");  // Valid - x is still alive
```

### Lifetime Annotations
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```
- `'a` is a **lifetime parameter** - tells Rust that the returned reference lives as long as the shorter of `x` or `y`
- All references with `'a` must live at least as long as `'a`

### Lifetime in Structs
```rust
struct ImportantExcerpt<'a> {
    part: &'a str,  // This reference must be valid for lifetime 'a
}
```

### Lifetime in Methods
```rust
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3  // No lifetime needed - returns owned data
    }
    
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention: {}", announcement);
        self.part  // Lifetime elision rules apply
    }
}
```

### The 'static Lifetime
```rust
let s: &'static str = "I have a static lifetime.";
```
- Lives for the **entire program duration**
- String literals are `'static` by default
- Use sparingly - most references don't need to be static

## Combining Generics, Traits & Lifetimes
```rust
fn longest_with_an_announcement<'a, T>(
    x: &'a str, 
    y: &'a str, 
    ann: T
) -> &'a str 
where 
    T: Display 
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}
```

## Lifetime Elision Rules (Automatic)
Compiler can infer lifetimes in simple cases:

1. Each parameter gets its own lifetime
2. If one input lifetime, it's assigned to all outputs
3. If `&self` or `&mut self` exists, its lifetime goes to all outputs

```rust
// You write:
fn first_word(s: &str) -> &str { }

// Compiler infers:
fn first_word<'a>(s: &'a str) -> &'a str { }
```

## Key Takeaways

### Generics
- Enable code reuse across types
- Use trait bounds (`T: Trait`) to restrict generic types
- Zero runtime cost (monomorphization)

### Traits
- Define shared behavior
- Default implementations available
- Use `impl Trait` or `<T: Trait>` for parameters
- `where` clause for complex bounds

### Lifetimes
- Prevent dangling references at compile time
- `'a` syntax connects reference lifetimes
- `'static` for program-duration references
- Usually inferred, explicit only when needed

## Common Patterns

**Trait Objects (Dynamic Dispatch):**
```rust
fn notify(item: &dyn Summary) {  // Runtime polymorphism
    println!("{}", item.summarize());
}
```

**Returning Traits:**
```rust
fn returns_summarizable() -> impl Summary {
    NewsArticle { /* ... */ }
}
```

**Trait Inheritance:**
```rust
trait OutlinePrint: Display {  // Requires Display
    fn outline_print(&self) { }
}
```
---

# Rust Module Scope & `super` Keyword Guide

## Understanding Module Hierarchy

### Example Project Structure
```
crate (root)
 ├── main.rs
 └── books/
      ├── mod.rs
      └── the_rust_programming_language/
           ├── mod.rs
           └── excercises/
                ├── mod.rs
                └── sol_09_strings/
                     ├── mod.rs
                     └── strings4.rs
```

## The `super` Keyword

`super` is **not global** — it only works inside a module to refer to its **parent module**.

### Navigation Rules

| From `strings4.rs` | Code | Goes To |
|-------------------|------|---------|
| Parent module | `use super::*;` | `sol_09_strings` |
| Grandparent | `use super::super::*;` | `excercises` |
| Root | `use crate::books;` | `main.rs` level |

## Practical Example

### File: `sol_09_strings/mod.rs`
```rust
pub mod strings4;

pub fn print_header() {
    println!("Running Strings Section!");
}
```

### File: `sol_09_strings/strings4.rs`
```rust
pub fn run() {
    super::print_header(); // 👈 Calls parent module's function
    println!("Running strings4!");
}
```

**Output:**
```
Running Strings Section!
Running strings4!
```

## Why `super` Doesn't Work in `main.rs`

`main.rs` is the **crate root** — it has no parent module. Instead, use absolute paths:

```rust
// Option 1: Direct path
books::the_rust_programming_language::excercises::sol_09_strings::strings4::run();

// Option 2: Explicit crate prefix (same thing)
crate::books::the_rust_programming_language::excercises::sol_09_strings::strings4::run();
```

## Quick Reference Table

| Context | What to Use | Meaning |
|---------|-------------|---------|
| Inside submodule | `super::something()` | Access parent module |
| Deeply nested | `super::super::something()` | Go up two levels |
| From root (`main.rs`/`lib.rs`) | `crate::something()` or just `something()` | Access from root |
| Same module | `self::something()` or `use super::*` | Access current scope/parent |

## Key Takeaways

1. **`super` is relative** — it navigates up the module tree
2. **`crate` is absolute** — it always starts from the root
3. **Root modules have no parent** — `super` is meaningless in `main.rs`/`lib.rs`
4. **Use absolute paths from root** — clearer and avoids confusion
---

# Closures and Iterators in Rust

### Closures

**Closures** are anonymous functions that can capture variables from their surrounding environment. They're defined using the `|params|` syntax.

**Basic Syntax:**
```rust
let closure = |param| param + 1;
let add = |a, b| a + b;
```

**Key Features:**
- Type inference (usually don't need explicit type annotations)
- Can capture values from their environment
- More flexible than regular functions for short, context-specific operations

**Capturing Values:**

Closures can capture environment values in three ways:

1. **Immutable borrow** - Just reads the value
   ```rust
   let list = vec![1, 2, 3];
   let print_list = || println!("{:?}", list); // borrows immutably
   ```

2. **Mutable borrow** - Modifies the value
   ```rust
   let mut list = vec![1, 2, 3];
   let mut add_item = || list.push(4); // borrows mutably
   ```

3. **Takes ownership** - Uses `move` keyword
   ```rust
   let list = vec![1, 2, 3];
   thread::spawn(move || println!("{:?}", list)); // moves ownership
   ```

**Closure Traits:**

Closures automatically implement one or more of these traits based on how they capture values:

- **`FnOnce`** - Consumes captured values (can be called only once)
- **`FnMut`** - Mutates captured values (can be called multiple times with `&mut`)
- **`Fn`** - Borrows immutably (can be called multiple times safely)

**Rule:** Every closure implements `FnOnce`. If it doesn't move values, it also implements `FnMut`. If it neither moves nor mutates, it also implements `Fn`.

### Iterators

**Iterators** provide a way to process sequences of elements efficiently and expressively.

**Creating Iterators:**
```rust
let v = vec![1, 2, 3];
let iter = v.iter();        // immutable references
let iter_mut = v.iter_mut(); // mutable references
let into_iter = v.into_iter(); // takes ownership
```

**Common Iterator Methods:**

**Consuming adaptors** (consume the iterator):
- `.sum()` - Sums all elements
- `.collect()` - Converts iterator into a collection
- `.for_each()` - Executes closure on each element

**Iterator adaptors** (transform iterators into other iterators):
- `.map(|x| ...)` - Transforms each element
- `.filter(|x| ...)` - Keeps elements matching a condition
- `.take(n)` - Takes first n elements
- `.skip(n)` - Skips first n elements

**Example - Chaining Iterators:**
```rust
let v: Vec<i32> = vec![1, 2, 3, 4, 5];
let result: Vec<i32> = v.iter()
    .filter(|x| *x % 2 == 0)  // keep even numbers
    .map(|x| x * 2)            // double them
    .collect();                // [4, 8]
```

**Why Use Iterators?**
- Zero-cost abstraction (as fast as manual loops)
- More expressive and functional style
- Chainable operations
- Less prone to off-by-one errors
- Easier to parallelize
___

# Smart Pointers in Rust

**Smart pointers** are data structures that act like pointers but have additional metadata and capabilities. They implement the `Deref` and `Drop` traits, allowing them to behave like references while providing extra functionality.

---

### Common Smart Pointers

#### 1. `Box<T>` - Heap Allocation

**Purpose:** Store data on the heap instead of the stack.

**Use Cases:**
- When type size can't be known at compile time (e.g., recursive types)
- When you have large data and want to transfer ownership without copying
- When you need trait objects (care about trait implementation, not concrete type)

**Example:**
```rust
let b = Box::new(5); // stores 5 on the heap
```

**Enabling Recursive Types:**
```rust
enum List {
    Cons(i32, Box<List>),  // Box breaks the infinite size cycle
    Nil,
}

let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
```

---

#### 2. `Rc<T>` - Reference Counting

**Purpose:** Enable multiple ownership of the same data (single-threaded only).

**Key Features:**
- Keeps track of the number of references to a value
- Value is cleaned up when reference count reaches 0
- Only allows immutable borrows

**Example:**
```rust
use std::rc::Rc;

let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
let b = Cons(3, Rc::clone(&a));  // increases ref count
let c = Cons(4, Rc::clone(&a));  // increases ref count again

println!("count = {}", Rc::strong_count(&a)); // prints: 3
```

**Important:** Use `Rc::clone(&a)` instead of `a.clone()` - it only clones the pointer, not the data.

---

#### 3. `RefCell<T>` - Interior Mutability

**Purpose:** Allow mutation of data even with immutable references (borrowing rules checked at runtime).

**Key Characteristics:**
- Single ownership (like `Box<T>`)
- Allows mutable borrows checked at **runtime** (not compile time)
- Single-threaded only
- Can mutate value inside `RefCell<T>` even when `RefCell<T>` itself is immutable

**Methods:**
- `.borrow()` - returns immutable reference
- `.borrow_mut()` - returns mutable reference
- Panics at runtime if borrowing rules violated

**Example:**
```rust
use std::cell::RefCell;

struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
}

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        // self is immutable, but we can mutate through RefCell
        self.sent_messages.borrow_mut().push(String::from(message));
    }
}
```

---

### Comparison Table

| Type | Ownership | Borrow Checking | Use Case |
|------|-----------|----------------|----------|
| `Box<T>` | Single | Compile time | Heap allocation, recursive types |
| `Rc<T>` | Multiple | Compile time (immutable only) | Shared ownership |
| `RefCell<T>` | Single | **Runtime** | Interior mutability |

---

### The `Deref` Trait

**Purpose:** Customize behavior of the dereference operator `*`.

**Custom Implementation:**
```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

let x = 5;
let y = MyBox::new(x);
assert_eq!(5, *y);  // Behind the scenes: *(y.deref())
```

**Deref Coercion:**

Rust automatically converts references using the `Deref` trait:
```rust
fn hello(name: &str) {
    println!("Hello, {name}!");
}

let m = MyBox::new(String::from("Rust"));
hello(&m);  // Automatically: &MyBox<String> → &String → &str
```

**Without deref coercion, you'd need:** `hello(&(*m)[..])`

**Deref Coercion Rules:**
- `&T` to `&U` when `T: Deref<Target=U>`
- `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
- `&mut T` to `&U` when `T: Deref<Target=U>`

---

### The `Drop` Trait

**Purpose:** Customize cleanup code when a value goes out of scope.

**Example:**
```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping with data `{}`!", self.data);
    }
}

let c = CustomSmartPointer { data: String::from("my stuff") };
// Drop called automatically when c goes out of scope
```

**Manual Drop:**
```rust
let c = CustomSmartPointer { data: String::from("data") };
drop(c);  // drop early using std::mem::drop
// c.drop() is NOT allowed - use drop(c) instead
```

---

### Preventing Memory Leaks with `Weak<T>`

**Problem:** `Rc<T>` can create reference cycles causing memory leaks.

**Reference Cycle Example:**
```rust
// This creates a cycle: a → b → a
// Neither can be dropped because strong_count never reaches 0
let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
*a.tail().borrow_mut() = Rc::clone(&b);  // cycle formed!
```

**Solution - Use `Weak<T>`:**

- Created with `Rc::downgrade(&rc)`
- Increases `weak_count`, not `strong_count`
- Doesn't prevent cleanup
- Access via `.upgrade()` → returns `Option<Rc<T>>`

**Example - Tree Structure:**
```rust
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,      // Weak to prevent cycle
    children: RefCell<Vec<Rc<Node>>>, // Strong ownership of children
}

let leaf = Rc::new(Node {
    value: 3,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![]),
});

let branch = Rc::new(Node {
    value: 5,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![Rc::clone(&leaf)]),
});

// Child holds weak reference to parent
*leaf.parent.borrow_mut() = Rc::downgrade(&branch);

// Access parent through upgrade
println!("{:?}", leaf.parent.borrow().upgrade());  // Some(Node {...})
```

**Reference Counting:**
- `Rc::strong_count(&rc)` - counts strong references
- `Rc::weak_count(&rc)` - counts weak references
- Value dropped when `strong_count == 0` (regardless of `weak_count`)

---

### Common Combinations

- **`Rc<RefCell<T>>`** - Multiple ownership with interior mutability
- **Tree/Graph structures** - Use `Rc<T>` for children, `Weak<T>` for parent references
- **Shared mutable state** - `Rc<RefCell<T>>` (single-threaded only)

*More chapters and notes will be added as I progress through the book...*