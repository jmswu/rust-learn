# Rust in 30 Days

A 5-minute-a-day Rust course for people who already know C or C++.

No “what is a variable” filler. Each day gives you:

- a 1-minute lesson as a comment at the top of the day's file,
- one tiny exercise with failing tests,
- a command that tells you whether you got it right.

**Done = `cargo test dayNN` passes and you can explain the idea in one sentence.**

## Start here

Rust is installed on this machine (verified with `rustc 1.99.1` / `cargo 1.99.1`). If a new terminal ever says `cargo: command not found`:

```bash
source "$HOME/.cargo/env"
```

Then open `TODO.md`, start at Day 01, and run:

```bash
cargo test day01
```

Edit `src/days/day01.rs` and replace the `todo!("implement me")` with your code. Run the command again until it passes.

## The daily 5-minute loop

1. **Read** the `//!` lesson at the top of `src/days/dayNN.rs` (1 min).
2. **Implement** the function(s) marked with `todo!` (3 min).
3. **Run** `cargo test dayNN` and fix what the compiler/tests say (1 min).
4. **Tick** the day in `TODO.md` and write a one-line takeaway.

If a day takes 10 minutes, that is normal. Checkpoints (days 14, 21, 28) and the capstone (29–30) are deliberately a little bigger. Stop when the test passes; do not binge. The “stretch” in `TODO.md` is for days when you have extra time.

## Commands

```bash
cargo test day08          # run one day
cargo test day08 -- --nocapture   # show println! output
cargo test                # everything (fails until all exercises are solved)
cargo check               # fast type check, no binary
cargo run                 # course welcome
cargo run --bin grep -- -i rust README.md   # capstone, after Days 29-30
cargo fmt                 # format
cargo clippy              # lints
cargo doc --open          # generate docs for this crate
rustc --explain E0382     # explain a compiler error code
```

`cargo test` runs all 30 days, so it will fail until you have solved all of them. Always use the targeted command during the course.

## What is in the repo

```
src/days/dayNN.rs   one lesson + exercise + tests per day
src/bin/scratch.rs  paste compiler-gym snippets here; errors here don't matter
src/bin/grep.rs     capstone mini-grep (days 29-30)
SOLUTIONS.md        worked answers, one body per exercise
TODO.md             your 30-day checklist and streak grid
CHEATSHEET.md       compact syntax/ownership/collections reference
```

## The 30-day map

| Week | Days | Focus |
|---|---|---|
| 1 | 1–7 | Cargo, syntax, functions, loops, ownership, first checkpoint |
| 2 | 8–14 | Borrowing, strings, structs, enums, pattern matching, RPN checkpoint |
| 3 | 15–21 | Errors, traits, generics, lifetimes, modules, generic stack checkpoint |
| 4 | 22–28 | Closures, iterators, collections, smart pointers, threads, testing, word-frequency checkpoint |
| 5 | 29–30 | Capstone mini-grep and next steps |

The exact daily list, commands, and stretch goals are in `TODO.md`.

## C++ → Rust mental model

| C++ | Rust | Notes |
|---|---|---|
| `std::vector<T>` | `Vec<T>` | growable heap buffer |
| `std::string` | `String` | owned, growable, UTF-8 |
| `const std::string&` | `&str` | borrowed UTF-8 slice |
| `std::array<T, N>` | `[T; N]` | fixed length, stack |
| `std::unique_ptr<T>` | `Box<T>` | one owner, automatic drop |
| `std::shared_ptr<T>` | `Rc<T>` / `Arc<T>` | `Rc` is single-thread, `Arc` is atomic |
| `std::optional<T>` | `Option<T>` | no null pointers |
| `std::variant<...>` | `enum` | variants may carry data |
| exceptions | `Result<T, E>` + `?` | errors are values |
| `std::function` | `Box<dyn Fn(...)>` or generics | prefer `impl Fn` |
| lambdas | closures | capture modes are checked |
| templates | generics | checked at definition, monomorphized |
| virtual functions / inheritance | traits + `dyn Trait` | composition over inheritance |
| constructors / destructors | `new` convention / `Drop` | drop is automatic and deterministic |
| `new` / `delete` | `Box::new` / scope end | no manual delete |
| `T*` that may be null | `Option<&T>` / `Option<Box<T>>` | null is not a value |
| `std::mutex` | `Mutex<T>` | the data lives inside the lock |
| `std::thread` | `thread::spawn` + `move` | data races are compile errors |
| CMake + vcpkg/Conan | Cargo | one tool for build + deps |
| headers + `.cpp` files | modules | `mod` + `use` |
| `printf` | `println!` / `format!` | checked at compile time |
| `std::move` | automatic move | `.clone()` for an explicit deep copy |
| `reinterpret_cast` | `unsafe` | avoid in safe code |

Two rules worth taping to the monitor:

1. **Move is the default; clone is explicit.** C++ leaves moved-from objects usable-but-unspecified. Rust forbids using them.
2. **Many readers XOR one writer.** `&T` or `&mut T`, never both at once. This is the borrow checker in one sentence.

## When you are stuck

- **Read the first compiler error, then recompile.** Later errors are often cascades.
- Read the `help:` and `note:` lines — they frequently name the exact fix.
- `rustc --explain E0382` (moved value), `E0502` (borrow conflict), `E0597` (lifetime), `E0277` (trait bound), `E0308` (type mismatch).
- Run `cargo test dayNN` rather than the whole suite while learning.
- Paste “compiler gym” snippets into `src/bin/scratch.rs`; break things there safely.
- Try for 5 minutes, then compare with `SOLUTIONS.md`. Retype the answer instead of copy-pasting; muscle memory matters.
- If a day is still opaque, mark it, move on, and come back after two more days. Ownership clicks through repetition, not through one perfect explanation.

## After Day 30

- Re-read *The Rust Programming Language* chapters 4, 10, and 13: <https://doc.rust-lang.org/book/>
- Do `rustlings` for more small compiler exercises.
- Build a real tool: a JSON CLI with `serde`, a web service with `axum`, or a CLI with `clap`.
- Learn `async`/`await` + `tokio` when you have many concurrent I/O tasks.
- For C/C++ interop, look at `unsafe`, `extern "C"`, and `bindgen`.
- Keep `cargo fmt` and `cargo clippy` in your loop.

The goal of these 30 days is not to know everything. It is to make the compiler feel like a strict but fair colleague instead of an obstacle.
