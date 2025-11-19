# 🦀 Rust Learning Hub

A comprehensive workspace documenting my journey through the Rust programming language. This repository serves as a centralized collection of code, notes, exercises, and experiments as I work through multiple Rust books and resources.

---

## 📖 About

This repository is my personal **Rust learning laboratory** — a structured approach to mastering Rust through hands-on practice, deliberate documentation, and iterative experimentation. Each book or resource gets its own dedicated space, making it easy to track progress and revisit concepts.

### Why This Repo?

- **Structured Learning**: Organized by book/resource for clear progression
- **Hands-On Practice**: Real code, not just theory
- **Knowledge Retention**: Written notes and examples to reinforce learning
- **Progress Tracking**: Easy to see what I've covered and what's next
- **Reference Material**: A personal Rust reference I can return to

---

## 🗂 Repository Structure

```
rust-learning/
├── Cargo.toml              # Root workspace configuration
├── src/
│   └── main.rs             # Quick experiments and scratch code
│   └──books/
│       ├── the-rust-programming-language/
│       │   ├── chapters/       # Chapter-by-chapter code
│       │   ├── examples/       # Worked examples from the rustlings
│       │   └── README.md       # Progress tracker for this book
│       │
│       ├── rust-by-example/
│       │   ├── examples/       # Code examples from RBE
│       │   └── README.md       # Progress and notes
│       │  
│       ├── async-rust/
│       │   ├── examples/       # Async/await examples
│       │   ├── experiments/    # Custom async projects
│       │   └── README.md       # Async learning progress
│       │
│       └── ...                 # Additional books as I progress
│
├── NOTES.md                # Notes and Insights
└── readme.md               # This file
```

### Folder Conventions

Each book folder follows a consistent structure:

- **`chapters/`** — Code organized by chapter or section
- **`examples/`** — Direct examples from the book (sometimes with my modifications)
- **`exercises/`** — My solutions to problems and challenges
- **`experiments/`** — Personal explorations beyond the book content
- **`notes/`** — Markdown notes on key concepts
- **`README.md`** — Book-specific progress, insights, and chapter checklist

---

## 📚 Learning Resources

### Currently Working Through

| Book/Resource | Status | Focus Areas |
|--------------|--------|-------------|
| [The Rust Programming Language](https://doc.rust-lang.org/book/) | 🔄 In Progress | Fundamentals, ownership, lifetimes |
| [Rust By Example](https://doc.rust-lang.org/rust-by-example/) | 📋 Planned | Practical patterns and idioms |
| [Async Programming in Rust](https://rust-lang.github.io/async-book/) | 📋 Planned | Async/await, futures, tokio |

### Future Resources

- Rustonomicon (unsafe Rust)
- Rust Design Patterns
- Command Line Apps in Rust
- Zero To Production in Rust

---

## 🎯 Learning Goals

### Core Competencies

- ✅ Understand ownership, borrowing, and lifetimes deeply
- ✅ Write idiomatic, safe Rust code
- ✅ Master pattern matching and error handling
- ✅ Gain proficiency with traits, generics, and advanced types
- ✅ Build confidence with async Rust and concurrency
- ✅ Explore unsafe Rust and FFI when necessary

----

## 🚀 Running Code

### Quick Experiments

To run the main scratch file:

```bash
cargo run
```

### Running Specific Examples

Navigate to a book's folder and run examples:

```bash
cd books/the-rust-programming-language/chapters/03-variables
cargo run
```

Or use the manifest path directly:

```bash
cargo run --manifest-path books/the-rust-programming-language/chapters/03-variables/Cargo.toml
```

### Testing

Run tests across the workspace:

```bash
cargo test
```

---

## 📝 Notes & Insights

As I progress, I'll document key learnings, "aha moments," and challenging concepts in each book's README. This helps reinforce understanding and creates a personal reference guide.

### Key Concepts I'm Tracking

- **Ownership & Borrowing**: The core of Rust's memory safety
- **Lifetimes**: When and why they're needed
- **Trait System**: Composition over inheritance
- **Error Handling**: `Result`, `Option`, and the `?` operator
- **Async/Await**: Writing non-blocking concurrent code
- **Macros**: Code generation and metaprogramming
- **Unsafe Rust**: When and how to use it responsibly

---

## 🛠 Development Setup

### Prerequisites

- **Rust** (latest stable): Install via [rustup](https://rustup.rs/)
- **Cargo**: Comes with Rust
- **IDE/Editor**: VS Code with rust-analyzer, or your preferred setup

### Recommended Tools

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Check without building
cargo check
```

---

## 📈 Progress Tracking

I maintain detailed progress in each book's individual README. Overall:

- **Total Books**: 3+ planned
- **Chapters Completed**: Tracked per book
- **Exercises Solved**: Growing collection
- **Projects Built**: See project milestones above

---

## 🤝 Contributing

This is primarily a personal learning repository, but if you spot errors, have suggestions for better approaches, or want to share Rust learning resources, feel free to open an issue or PR!

---

## 📄 License

This repository is licensed under the **MIT License**. All code is for educational purposes.

---

## 👤 Author

**Sumit Mazumdar**

- Learning Rust since: [May 2025]
- Connect: [https://x.com/sum1t_here]

---

## 🙏 Acknowledgments

- The Rust community for excellent learning resources
- Authors of "The Rust Programming Language" book
- Contributors to Rust By Example and other learning materials

---

**Happy Learning! 🦀**

> "Rust is not about making simple things harder; it's about making hard things possible." — Someone wise, probably