<p align="center">
    <img src="https://raw.githubusercontent.com/navahas/latdder/assets/images/latdder.png"
        alt="latter_logo" width="200"/>
</p>

# Latdder: Type-Driven API Design in Rust

> **Note**: This project is currently under construction and is not yet ready for public use. It will be shared publicly in the near future once the initial stages are complete, at which point contributions will be welcomed.

**Latdder** is an educational Rust library designed to teach advanced type system concepts through practical, progressive exercises. The name combines "ladder" (representing the step-by-step learning progression) with "tdd" (from Type-Driven-Design).

## Motivation

After finding the concept of Type Driven Design enforced at compile time, I wanted to learn and practice. Since no resource was available, I created my own exercises and, in fact, I practice while building this project.

Learn by doing. Each "ladder" is a series of levels, and each level presents a challenge that you solve by implementing Rust code. Your solution is then verified by running tests. This method allows you to:

* **Understand state transitions:** Grasp how to model and enforce state changes using Rust's type system.
* **Build robust APIs:** Learn to design APIs where invalid states are impossible to represent, leading to fewer bugs and more reliable code.
* **Practice idiomatic Rust:** Develop solutions using common and effective Rust patterns.

### Recommended Resources

- [Will Crichton - Rust API Type Patterns](https://willcrichton.net/rust-api-type-patterns/)
- Jon Gjengset

## Project Goals

This library provides hands-on experience with:
- **Typestate Pattern**: Encoding state machines in the type system
- **Zero-Cost Abstractions**: Compile-time guarantees without runtime overhead
- **Feature Gates**: Conditional compilation and progressive complexity
- **Documentation Testing**: Embedded examples that prove correctness

## Learning Modules

### Init Module
Basic introduction to typestate patterns using a simple state machine.

### Toast Module  
Progressive typestate exercises using a toaster metaphor, with 5 difficulty levels:
- **Level 1**: Basic typestate - prevent toasting when unplugged
- **Level 2**: Multi-state validation - require both power and bread
- **Level 3**: Enhanced state tracking with additional constraints
- **Level 4**: Complex state transitions and error handling
- **Level 5**: Advanced typestate patterns and optimizations

## Usage & Workflow

Each level is gated behind cargo features to allow progressive learning:

```bash
# Start with level 1
cargo test --features toast_level_01

# Progress to level 2
cargo test --features toast_level_02

# Generate documentation for all levels
cargo doc --open
```

## Exercise Workflow

**IMPORTANT**: For each exercise theme (toast, init, etc.), you'll work primarily in the corresponding `theme/api.rs` file. This is where you implement your typestate API that gets tested by the level documentation examples.

**Typical workflow**:
1. Read the level documentation to understand the goal
2. Implement your API in the `theme/api.rs` file  
3. Run `cargo test --features theme_level_X` to test your implementation
4. Iterate until all doc tests pass (including compile_fail tests)
5. Move to the next level

The level files contain the **tests and examples** that validate your API implementation. Your job is to make those tests pass by building the right typestate API!

## Documentation Philosophy

Each level includes:
- **Goal statements**: What you'll learn
- **Working examples**: Code that compiles and runs
- **Compile-fail tests**: Code that should NOT compile (proving type safety)
- **Progressive complexity**: Each level builds on previous concepts

## Learning Path

1. Read the level documentation to understand the goal and requirements
2. Study the working examples to see the expected API usage
3. Examine the compile-fail tests to understand what should be prevented
4. **Implement your API** in the corresponding `theme/api.rs` file
5. Test your implementation with `cargo test --features theme_level_X`
6. Iterate until all tests pass, then progress to the next level

**Key insight**: The level files are your **specification** - they show what your API should do. Your task is to implement an API that satisfies these specifications!

Start your journey with the init module or jump into the toast module for the main curriculum!

-----

## Project Structure

```text
.
├── Cargo.lock
├── Cargo.toml
├── ladder
│   ├── init
│   │   ├── api.rs
│   │   └── levels
│   │       └── level_01.rs
│   ├── lib.rs
│   └── toast
│       ├── api.rs
│       └── levels
│           ├── level_01.rs
│           ├── level_02.rs
│           ├── level_03.rs
│           ├── level_04.rs
│           └── level_05.rs
├── README.md
└── solutions
    └── toast
        ├── level_01.rs
        ├── level_02.rs
        ├── level_03.rs
        ├── level_04.rs
        └── level_05.rs
```
