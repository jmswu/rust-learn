# Rust cheat sheet — for C/C++ programmers

## Cargo

```bash
cargo new name          # new binary crate
cargo init              # add Cargo.toml to an existing directory
cargo check             # type-check, no binary
cargo run               # build + run
cargo test name         # run matching tests
cargo test -- --nocapture
cargo fmt
cargo clippy
cargo add serde         # add a dependency
cargo doc --open
```

`Cargo.toml` is the project manifest; `Cargo.lock` pins exact dependency versions.
`target/` is build output and is disposable.

## Comments and docs

```rust
// line comment
/* block comment */
/// doc comment for the next item; code fences inside become doc tests
//! doc comment for the enclosing module/crate
```

## Variables and types

```rust
let x = 5;                 // immutable, inferred i32
let mut y = 5;             // mutable
let x = "shadow";          // shadowing, can change type
const LIMIT: u32 = 100;    // compile-time constant
static NAME: &str = "app"; // global, fixed address

let n: i64 = 42;
let f: f64 = 1.5;
let b: bool = true;
let c: char = '🦀';         // 4-byte Unicode scalar
let text: &str = "hello";
let owned: String = String::from("hello");

let bytes: [u8; 3] = [1, 2, 3];
let tuple: (i32, &str) = (1, "one");
let nums: Vec<i32> = vec![1, 2, 3];
```

Integer types: `i8 i16 i32 i64 i128 isize` and `u8 u16 u32 u64 u128 usize`.
`usize` is for indices and lengths. No implicit numeric conversion; use `as`
(truncating) or `try_into()` (checked).

Overflow: debug panics, release wraps. Be explicit with
`wrapping_add`, `checked_add`, `saturating_add`.

## Functions and control flow

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b                    // last expression, no semicolon
}

fn abs(n: i32) -> i32 {
    if n < 0 { -n } else { n }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = std::fs::read_to_string("data.txt")?;
    println!("{text}");
    Ok(())
}
```

```rust
if cond { ... } else { ... }
let label = if ok { "yes" } else { "no" };

loop { break 42; }           // loop can return a value
'outer: loop { break 'outer; }
while cond { ... }
for i in 0..n { ... }        // excludes n
for i in 0..=n { ... }       // includes n
for item in &collection { ... }
```

No overloading, no default arguments, no truthiness.

## Ownership and borrowing

Four rules:

1. Every value has one owner.
2. Assignment/passing moves ownership, unless the type is `Copy`.
3. At any time: many `&T` **or** one `&mut T`, never both.
4. References never outlive the value they point to.

```rust
let a = String::from("hi");
let b = a;                   // move; `a` is no longer usable

let c = b.clone();           // explicit deep copy

fn take(s: String) {}        // consumes
fn read(s: &str) {}          // borrows immutably
fn edit(s: &mut String) {}   // borrows mutably
```

`Copy` types: integers, floats, `bool`, `char`, and tuples/arrays of `Copy`.
`String`, `Vec`, `Box`, `HashMap` are not `Copy`.

```rust
let mut v = vec![1, 2, 3];
let first = v[0];            // i32 is Copy
v.push(4);                   // fine

let mut v = vec![1, 2, 3];
let first = &v[0];           // shared borrow
v.push(4);                   // error: cannot borrow mutably while borrowed
println!("{first}");
```

The compiler uses NLL (non-lexical lifetimes): a borrow ends at its last use,
not at the end of the block.

## Strings

```rust
let owned: String = String::from("hello");
let borrowed: &str = &owned;        // full borrow
let slice: &str = &owned[1..3];     // byte range; may panic on UTF-8 boundary
let lit: &'static str = "hello";

owned.push_str(" world");
owned.push('!');
let n = owned.len();                // bytes, not chars
let upper = owned.to_uppercase();
let parts: Vec<&str> = owned.split_whitespace().collect();

println!("{owned}");                // Display
println!("{owned:?}");              // Debug
println!("{n}");                    // inline named argument
println!("{:>8}", "right");
```

Use `&str` in parameters; return `String` when you allocate. Indexing is by
bytes, so use `chars()` / `char_indices()` for Unicode-aware work.

## Structs, enums, traits

```rust
#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Point { x: f64, y: f64 }

impl Point {
    fn new(x: f64, y: f64) -> Self { Self { x, y } }
    fn len(&self) -> f64 { (self.x * self.x + self.y * self.y).sqrt() }
    fn shift(&mut self, dx: f64) { self.x += dx; }
}

enum Shape {
    Circle(f64),
    Rect { w: f64, h: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Rect { w, h } => w * h,
        }
    }
}

trait Summary {
    fn text(&self) -> String;
    fn short(&self) -> String { self.text().chars().take(10).collect() }
}

impl Summary for Point {
    fn text(&self) -> String { format!("({}, {})", self.x, self.y) }
}
```

`&self` = read-only receiver, `&mut self` = mutating, `self` = consuming.
No inheritance. Traits provide shared behavior; `dyn Trait` gives dynamic
dispatch (vtable), generic bounds give static dispatch.

## Pattern matching

```rust
match n {
    0 => "zero",
    x if x < 0 => "negative",
    1..=9 => "digit",
    _ => "other",
}

match (a, b) {
    (0, 0) => "origin",
    (0, _) => "y axis",
    (_, 0) => "x axis",
    _ => "elsewhere",
}

if let Some(x) = maybe { ... }
let Some(x) = maybe else { return None; };
let is_some = matches!(maybe, Some(_));
```

`match` must be exhaustive. Patterns work on enums, structs, tuples, arrays,
references, and ranges.

## `Option` and `Result`

```rust
enum Option<T> { Some(T), None }
enum Result<T, E> { Ok(T), Err(E) }

let x = maybe.unwrap_or(0);
let y = maybe.unwrap_or_else(|| compute());
let z = maybe.expect("maybe must be Some");

// `?` returns early on None/Err
fn first_even(xs: &[i32]) -> Option<i32> {
    for &x in xs {
        if x % 2 == 0 { return Some(x); }
    }
    None
}

fn parse(input: &str) -> Result<i32, std::num::ParseIntError> {
    let n: i32 = input.trim().parse()?;
    Ok(n * 2)
}
```

`unwrap`/`expect` panic on failure — fine in tests/prototypes, not for user input.
`?` converts errors with `From`; use `map_err` to change the error type.
For apps: `anyhow`; for libraries: `thiserror`.

## Generics and lifetimes

```rust
fn largest<T: PartialOrd + Copy>(xs: &[T]) -> Option<T> { ... }

fn largest2<T>(xs: &[T]) -> Option<&T>
where
    T: PartialOrd,
{ ... }

struct Stack<T> { items: Vec<T> }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { ... }

struct Excerpt<'a> { part: &'a str }
```

Lifetimes describe relationships between references; they never extend a value's
life. Elision covers one input reference, or `&self` methods. `'static` means
“for the whole program” (e.g. string literals). Most code needs no annotations.

Generics are monomorphized: a separate machine-code copy per concrete type.
Trait bounds are checked at the definition, unlike unchecked C++ templates.

## Collections and iterators

```rust
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

let mut counts: HashMap<String, usize> = HashMap::new();
*counts.entry("word".to_string()).or_insert(0) += 1;
counts.get("word");                 // Option<&usize>

let mut sorted: Vec<_> = counts.into_iter().collect();
sorted.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
sorted.truncate(10);
```

```rust
let total: i32 = xs.iter()
    .filter(|&&x| x % 2 == 0)
    .map(|&x| x * x)
    .sum();

let first: Option<&i32> = xs.iter().find(|&&x| x > 10);
let doubled: Vec<i32> = xs.iter().map(|&x| x * 2).collect();
```

`iter()` → `&T`, `iter_mut()` → `&mut T`, `into_iter()` → `T`.
Adapters are lazy; consumers (`collect`, `sum`, `fold`, `count`, `find`, `any`,
`all`) drive them. Returning a chain: `impl Iterator<Item = T>`.

## Smart pointers and concurrency

```rust
let boxed: Box<i32> = Box::new(5);             // unique ownership, heap
let shared = Rc::new(5);                       // single-thread ref counting
let cell = RefCell::new(5);                    // runtime borrow checking
*cell.borrow_mut() += 1;

let shared = Arc::new(5);                      // thread-safe ref counting
let mutex = Arc::new(Mutex::new(0));
let clone = Arc::clone(&mutex);
std::thread::spawn(move || {
    *clone.lock().unwrap() += 1;
});
```

| C++ | Rust |
|---|---|
| `unique_ptr<T>` | `Box<T>` |
| `shared_ptr<T>` | `Rc<T>` / `Arc<T>` |
| `shared_ptr` + mutex | `Arc<Mutex<T>>` |
| `mutable` | `RefCell<T>` (single thread) |
| `weak_ptr<T>` | `Weak<T>` |

`Rc`/`RefCell` are not thread-safe. `Send`/`Sync` make data races compile errors.
Scoped threads (`std::thread::scope`) can borrow local data; `spawn` needs `move`.
Channels: `std::sync::mpsc::channel()`.

## Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    #[should_panic]
    fn panics_on_bad_input() {
        divide(1, 0);
    }
}
```

`#[ignore]` skips a test unless you pass `--ignored`. Doc tests run with
`cargo test --doc`. Integration tests go in `tests/`.

## Common compiler errors

| Code | Meaning | Usual fix |
|---|---|---|
| E0382 | use of moved value | borrow instead, or `.clone()` |
| E0502 | borrow conflict | end the shared borrow before mutating |
| E0499 | two mutable borrows | restructure; one writer at a time |
| E0597 | borrowed value does not live long enough | own the data, or shorten the borrow |
| E0277 | trait bound not satisfied | add a bound, or implement the trait |
| E0308 | mismatched types | check the `&`, `mut`, and return type |
| E0072 | recursive type has infinite size | add `Box` |
| E0433 | unresolved path | add `use`, or fix the module path |

## Formatting

```rust
format!("{name}")            // Display
format!("{value:?}")         // Debug
format!("{value:#?}")        // pretty Debug
format!("{:>8}", text)       // right-align in 8 columns
format!("{:.2}", 3.14159)    // 2 decimal places
println!();                  // newline
eprintln!();                 // stderr
```

## Filename / module layout

```text
src/main.rs          binary crate root (fn main)
src/lib.rs           library crate root
src/days/mod.rs      module named `days`
src/days/day01.rs    submodule named `day01`
src/bin/tool.rs      extra binary target (`cargo run --bin tool`)
tests/integration.rs integration tests
```
