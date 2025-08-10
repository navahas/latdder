![latter_logo](https://raw.githubusercontent.com/navahas/latdder/assets/images/latdder.png)

# Latdder: Learn Type-Driven API Design in Rust

Latdder offers a progressive, test-driven approach to mastering type-driven API design in Rust, with a focus on the powerful **type-state pattern**.

## Motivation

Learn by doing. Each "ladder" is a series of levels, and each level presents a challenge that you solve by implementing Rust code. Your solution is then verified by running tests. This method allows you to:

* **Understand state transitions:** Grasp how to model and enforce state changes using Rust's type system.
* **Build robust APIs:** Learn to design APIs where invalid states are impossible to represent, leading to fewer bugs and more reliable code.
* **Practice idiomatic Rust:** Develop solutions using common and effective Rust patterns.

## How to Use

1.  **Choose a level:** Start with the first level of a `ladder/theme`, for example: `init_level_01`.
2.  **Implement the logic:** Open the corresponding file (e.g., `ladder/toast/level_01.rs`) and write your Rust code to make the tests pass.
3.  **Run the test:** Execute the test for that level to see the current requirements and failures:
```bash
cargo test --test init_level_01
```
4.  **Verify your solution:** Once it passes, you've successfully completed the level\!
5.  **Explore solutions (optional):** If you get stuck, reference the solutions in the `solutions/theme` directory for guidance.

### Themes
- `toast/`: Learn state transitions like `PluggedIn → Toasted`

-----

## Project Structure

```
.
├── Cargo.lock
├── Cargo.toml
├── ladder                 # Your working directory for solving levels
│   ├── lib.rs
│   └── theme
│       └── level_01.rs
├── README.md
├── solutions              # Reference solutions for each level
│   └── theme
│       └── level_01.rs
└── tests                  # Test files for each level
    └── theme
        └── level_01.rs
```
