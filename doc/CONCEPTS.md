# Core Concepts: The Theory Behind Latdder

This guide introduces the key theoretical concepts you'll encounter throughout Latdder. Understanding these concepts will make your journey through the exercises much smoother.

## Phantom Types

### What are Phantom Types?

Phantom types are types that exist only at compile time and carry no runtime data. They're "phantom" because they don't actually store any values - they only exist to help the type system track information.

### How PhantomData Works

```rust
use std::marker::PhantomData;

struct Container<T> {
    data: String,
    _phantom: PhantomData<T>,  // Stores no data, only type information
}
```

The `PhantomData<T>` field tells the compiler "this struct is associated with type `T`" without actually storing a `T`. This enables powerful compile-time tracking.

### Why This Matters

Phantom types allow us to:
- **Track state at compile time** without runtime overhead
- **Prevent invalid operations** by making them impossible to express
- **Create self-documenting APIs** where types tell you what's possible
- **Catch errors early** during compilation rather than at runtime

### Real-World Example

```rust
struct Database<State> {
    connection_string: String,
    _state: PhantomData<State>,
}

struct Connected;
struct Disconnected;

impl Database<Disconnected> {
    fn connect(self) -> Database<Connected> {
        Database {
            connection_string: self.connection_string,
            _state: PhantomData,
        }
    }
}

impl Database<Connected> {
    fn query(&self, sql: &str) -> Result<Vec<Row>, Error> {
        // Only connected databases can query!
    }
}
```

## Typestate Pattern

### The Core Idea

The typestate pattern uses Rust's type system to encode **state machines**. Instead of tracking state with enums or booleans (which can be wrong at runtime), we make invalid states **impossible to represent**.

### Traditional Approach (Runtime Checking)

```rust
struct Toaster {
    plugged_in: bool,
    has_bread: bool,
}

impl Toaster {
    fn toast(&self) -> Result<Toast, Error> {
        if !self.plugged_in {
            return Err(Error::NotPluggedIn);  // Runtime error!
        }
        if !self.has_bread {
            return Err(Error::NoBread);       // Runtime error!
        }
        Ok(Toast::new())
    }
}
```

**Problems**: 
- Can create invalid states (`plugged_in: false, has_bread: true`)
- Errors only caught at runtime
- Need to check conditions everywhere

### Typestate Approach (Compile-Time Checking)

```rust
struct Toaster<Power, Bread> {
    _phantom: PhantomData<(Power, Bread)>,
}

struct Plugged;
struct Unplugged;
struct HasBread;
struct NoBread;

impl Toaster<Plugged, HasBread> {
    fn toast(self) -> Toast {
        Toast::new()  // Can't fail - only valid states can call this!
    }
}
```

**Benefits**:
- Invalid states literally cannot be created
- Errors caught at compile time
- No runtime checking needed
- Self-documenting API

### State Transitions

The magic happens in the transitions:

```rust
impl Toaster<Unplugged, NoBread> {
    fn new() -> Self {
        Toaster { _phantom: PhantomData }
    }
    
    fn plug_in(self) -> Toaster<Plugged, NoBread> {
        Toaster { _phantom: PhantomData }
    }
}

impl<Power> Toaster<Power, NoBread> {
    fn insert_bread(self) -> Toaster<Power, HasBread> {
        Toaster { _phantom: PhantomData }
    }
}
```

Notice how methods **consume** `self` and **return** a new state. This prevents you from using the old state after transition.

## Zero-Cost Abstractions

### The Promise

One of Rust's core principles is "zero-cost abstractions" - you shouldn't pay runtime costs for compile-time safety.

### How Typestate Achieves This

```rust
// This complex type...
let toaster: Toaster<Plugged, HasBread> = Toaster::new()
    .plug_in()
    .insert_bread();

// Compiles to essentially this:
struct ToasterData {
    // No state fields needed!
}
```

The compiler can optimize away:
- All the phantom data (zero size)
- All the state transitions (just value moves)
- All the type information (exists only during compilation)

### Runtime Verification

To verify this, you can check that different typestate variants have the same size:

```rust
assert_eq!(
    std::mem::size_of::<Toaster<Plugged, HasBread>>(),
    std::mem::size_of::<Toaster<Unplugged, NoBread>>()
);
```

## Builder Patterns

### Enhanced with Typestate

Traditional builders can be misused:

```rust
// Traditional builder - can forget required fields
let config = ConfigBuilder::new()
    .name("app")
    // Forgot to set required database_url!
    .build(); // Runtime panic or error
```

Typestate builders make missing fields impossible:

```rust
// Typestate builder - missing fields won't compile
let config = ConfigBuilder::new()
    .name("app")
    .database_url("postgres://...")  // Required by type system
    .build(); // Only compiles when all required fields are set
```

### Implementation Pattern

```rust
struct ConfigBuilder<Name, Database> {
    name: Option<String>,
    database_url: Option<String>,
    _phantom: PhantomData<(Name, Database)>,
}

struct Set;
struct Unset;

impl ConfigBuilder<Unset, Unset> {
    fn new() -> Self { /* ... */ }
}

impl<Database> ConfigBuilder<Unset, Database> {
    fn name(self, name: String) -> ConfigBuilder<Set, Database> {
        // Transition to "name set" state
    }
}

impl ConfigBuilder<Set, Set> {
    fn build(self) -> Config {
        // Only builds when both required fields are set
    }
}
```

## Common Patterns and Idioms

### The `self` Consumption Pattern

```rust
impl State1 {
    fn transition(self) -> State2 {  // Takes ownership
        // Can't use State1 anymore after this call
    }
}
```

This prevents accidentally using old states after transitions.

### Generic State Parameters

```rust
impl<SomeState> MyType<SomeState, SpecificState> {
    fn works_for_any_first_state(&self) {
        // This method works regardless of the first state
    }
}
```

This allows methods that work across multiple states.

### Conditional Compilation

```rust
impl<State> MyType<State> 
where 
    State: ValidForOperation  // Trait bound limits which states work
{
    fn operation(&self) {
        // Only works for states that implement ValidForOperation
    }
}
```

This provides fine-grained control over which operations are valid for which states.

## Key Takeaways

1. **Phantom types** let you track information without runtime cost
2. **Typestate pattern** makes invalid states impossible to represent
3. **Zero-cost abstractions** mean compile-time safety is free at runtime
4. **Builder patterns** become much more powerful with typestate
5. **State transitions** consume old states to prevent misuse

These concepts form the foundation for everything you'll build in Latdder. Take time to understand them - they'll make the exercises much more intuitive!

## Next Steps

Ready to put these concepts into practice? Start with:
- [Init Theme Guide](THEMES.md#init-theme) for hands-on experience
- [Toast Theme Guide](THEMES.md#toast-theme) for progressive challenges
