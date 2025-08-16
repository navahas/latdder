# Theme Guides: Hands-On Learning

This guide provides detailed instructions for each theme, including minimal working examples, step-by-step workflows, and clear learning objectives.

## Init Theme

### Overview
**Goal**: Introduction to typestate patterns using a simple state machine  
**Difficulty**: Beginner  
**Prerequisites**: Basic Rust knowledge  
**Time**: 15-30 minutes  

### What You'll Learn
- How phantom types work in practice
- Basic typestate pattern implementation
- Zero-cost abstractions in action
- Foundation for more complex patterns

### The Challenge
You'll implement a simple `Latdder` type that tracks a single state: `Ready`. This serves as your "Hello World" for typestate patterns.

### Expected API Usage
Your implementation should support this usage pattern:

```rust
use latdder::init::api::*;

// Basic usage
let latdder = Latdder::<Ready>::start();

// The type system tracks that this is a Ready latdder
let _ready_latdder: Latdder<Ready> = latdder;
```

### Step-by-Step Implementation

#### Step 1: Understand the Goal
Read the documentation in `ladder/init/levels/level_01.rs` to understand what API you need to create.

#### Step 2: Implement the Basic Structure
In `ladder/init/api.rs`, you'll need:

```rust
use std::marker::PhantomData;

// State type
pub struct Ready;

// Main type with phantom data
pub struct Latdder<State>(/* your phantom data here */);
```

#### Step 3: Implement the Constructor
Add the method that creates a new `Latdder<Ready>`:

```rust
impl Latdder<Ready> {
    pub fn start() -> Self {
        // Return a new instance
    }
}
```

#### Step 4: Test Your Implementation
```bash
cargo test --features init_level_01
```

#### Step 5: Verify Understanding
Your implementation should:
- Use `PhantomData` to track state without runtime cost
- Only allow creating `Latdder<Ready>` through the `start()` method
- Have zero runtime overhead

### Minimal Working Example

After completing this theme, you'll have created:

```rust
// This works
let latdder = Latdder::<Ready>::start();

// This also works (same thing)
let latdder = Latdder::start();

// The type system knows this is Latdder<Ready>
```

### Next Steps
Once you complete the init theme, you're ready for the [Toast Theme](#toast-theme), which builds on these concepts with much more complexity.

---

## Toast Theme

### Overview
**Goal**: Master typestate patterns through progressive complexity  
**Difficulty**: Beginner to Advanced  
**Prerequisites**: Complete Init Theme  
**Time**: 2-4 hours across 5 levels  

### What You'll Learn
- Multi-dimensional state tracking
- Complex state transitions
- Compile-time validation
- Real-world typestate API design
- Advanced phantom type techniques

### The Theme Metaphor
You'll build a typestate API for a toaster that tracks:
- **Power state**: `Plugged` or `Unplugged`
- **Bread state**: `HasBread` or `NoBread`
- **Additional states** in later levels

### Progressive Level Structure

#### Level 1: Basic Typestate
**Goal**: Prevent toasting when unplugged

**Expected API**:
```rust
use latdder::toast::api::*;

// This should work
let toast = Toaster::<Unplugged, NoBread>::new()
    .plug_in()
    .insert_bread()
    .toast();

// This should NOT compile
let toast = Toaster::<Unplugged, NoBread>::new()
    .toast(); // Error: can't toast while unplugged!
```

**Key Concepts**: 
- Two-dimensional state tracking
- Method availability based on state
- Consuming self in transitions

**Implementation Guide**:

1. **Define State Types**:
```rust
pub struct Plugged;
pub struct Unplugged;
pub struct HasBread;
pub struct NoBread;
```

2. **Create the Main Type**:
```rust
pub struct Toaster<Power, Bread>(/* phantom data for both states */);
```

3. **Implement State Transitions**:
```rust
impl Toaster<Unplugged, NoBread> {
    pub fn new() -> Self { /* ... */ }
    pub fn plug_in(self) -> Toaster<Plugged, NoBread> { /* ... */ }
}

impl<Power> Toaster<Power, NoBread> {
    pub fn insert_bread(self) -> Toaster<Power, HasBread> { /* ... */ }
}

impl Toaster<Plugged, HasBread> {
    pub fn toast(self) -> Self { /* ... */ }  // Only works when plugged AND has bread
}
```

**Testing**:
```bash
cargo test --features toast_level_01
```

#### Level 2: Enhanced State Validation
**Goal**: Require both power and bread for all operations

**New Challenges**:
- More complex state requirements
- Multiple valid paths to the same end state
- Enhanced type safety

**Expected Behavior**:
```rust
// Multiple valid paths
let route1 = Toaster::new().plug_in().insert_bread().toast();
let route2 = Toaster::new().insert_bread().plug_in().toast();

// Both invalid operations should fail to compile
let fail1 = Toaster::new().toast(); // No power, no bread
let fail2 = Toaster::new().plug_in().toast(); // Power but no bread
```

#### Level 3: State Constraints and Validation
**Goal**: Add additional constraints and state validation

**New Concepts**:
- More complex state relationships
- Conditional method availability
- Advanced phantom type usage

#### Level 4: Error Handling and Recovery
**Goal**: Handle error states and recovery mechanisms

**New Concepts**:
- Error states in the type system
- Recovery operations
- Fallible state transitions

#### Level 5: Advanced Patterns and Optimization
**Goal**: Master advanced typestate techniques

**New Concepts**:
- Performance optimizations
- Complex state hierarchies
- Real-world patterns

### General Workflow for All Levels

1. **Read the Level Documentation**
   - Open `ladder/toast/levels/level_X.rs`
   - Study the working examples (✅ Works)
   - Understand the compile-fail tests (❌ Fails to compile)

2. **Implement in API File**
   - Edit `ladder/toast/api.rs`
   - Add/modify types and implementations
   - Follow the patterns from previous levels

3. **Test Your Implementation**
   ```bash
   cargo test --features toast_level_X
   ```

4. **Debug Compilation Errors**
   - Read error messages carefully
   - Check that state transitions consume `self`
   - Verify phantom data is correctly structured

5. **Verify Success**
   - All doc tests should pass
   - Both working examples AND compile-fail tests should behave correctly
   - Move to next level

### Key Implementation Patterns

#### The Phantom Data Pattern
```rust
use std::marker::PhantomData;

pub struct Toaster<Power, Bread> {
    _phantom: PhantomData<(Power, Bread)>,
}
```

#### The State Transition Pattern
```rust
impl SomeState {
    pub fn transition(self) -> NewState {
        NewState { _phantom: PhantomData }
    }
}
```

#### The Generic Implementation Pattern
```rust
impl<SomeGenericState> Toaster<SomeGenericState, SpecificState> {
    pub fn method_for_any_power_state(&self) {
        // Works regardless of Power state
    }
}
```

### Common Pitfalls and Solutions

#### Pitfall 1: Forgetting to Consume Self
```rust
// Wrong - doesn't prevent using old state
pub fn transition(&self) -> NewState { /* ... */ }

// Right - consumes old state
pub fn transition(self) -> NewState { /* ... */ }
```

#### Pitfall 2: Incorrect Phantom Data
```rust
// Wrong - doesn't track both states
PhantomData<Power>

// Right - tracks both dimensions
PhantomData<(Power, Bread)>
```

#### Pitfall 3: Over-constraining Generic Implementations
```rust
// Wrong - too specific
impl Toaster<Plugged, HasBread> {
    pub fn some_method(&self) { /* ... */ }
}

// Better - works for any power state
impl<Power> Toaster<Power, HasBread> {
    pub fn some_method(&self) { /* ... */ }
}
```

### Success Criteria

For each level, your implementation is successful when:

1. **All working examples compile and run**
2. **All compile-fail examples fail to compile**  
3. **No runtime panics or errors**
4. **Clean, idiomatic Rust code**

### Troubleshooting

#### Tests Not Running?
- Check that you've enabled the correct feature: `--features toast_level_X`
- Verify your code is in `ladder/toast/api.rs`

#### Compile Errors?
- Read the error message carefully
- Check phantom data structure
- Ensure state transitions consume `self`

#### Compile-Fail Tests Passing?
- This means your API is too permissive
- Add more constraints to prevent invalid operations
- Review the expected failure scenarios

## General Tips for All Themes

### Understanding the Exercise Structure

1. **Level files are specifications** - they show what your API should do
2. **API files are your implementation** - where you write the actual code
3. **Tests validate behavior** - both positive and negative cases

### Development Workflow

1. Start with the simplest possible implementation
2. Add complexity incrementally
3. Test frequently
4. Read compiler messages carefully
5. Don't hesitate to start over if you get stuck

### Getting Help

- Re-read the [Core Concepts](CONCEPTS.md) if you're confused about theory
- Study the working examples to understand expected behavior
- Check the compile-fail examples to understand what should be prevented
- The compiler is your friend - read its messages carefully!

Ready to start? Begin with the [Init Theme](#init-theme) and work your way up!