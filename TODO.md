# TODO — Rust in 30 Days

**Start date:** __________
**Goal date:** __________
**Completed:** ☐☐☐☐☐☐☐☐☐☐☐☐☐☐☐☐☐☐☐☐☐☐☐☐☐☐☐☐☐☐ / 30

> **Done** = `cargo test dayNN` passes **and** you can explain the idea in one sentence.
> One day = 5 minutes. Checkpoints (14, 21, 28) and the capstone (29–30) may take 10–15.
> Missed a day? Continue where you left off; do not double up.

## Week 1 — Syntax with C++ reflexes

Streak: ☐ ☐ ☐ ☐ ☐ ☐ ☐

- [x] **Day 01 — Cargo and your first function** · `cargo test day01`
  - Stretch: `cargo fmt`, then `cargo doc --open`.
- [x] **Day 02 — Variables, mutability, shadowing** · `cargo test day02`
  - Stretch: add a `const LIMIT: i32 = 100;` and use it.
- [ ] **Day 03 — Types, casts, and overflow** · `cargo test day03`
  - Stretch: run `cargo test --release` and explain the overflow difference.
- [ ] **Day 04 — Functions and expressions** · `cargo test day04`
  - Stretch: rewrite `clamp` with early `return`s.
- [ ] **Day 05 — Control flow** · `cargo test day05`
  - Stretch: rewrite `collatz_steps` with `loop` + `break`.
- [ ] **Day 06 — Ownership I: move vs copy** · `cargo test day06`
  - Stretch: `rustc --explain E0382`.
- [ ] **Day 07 — Checkpoint: functions, loops, `Vec`** · `cargo test day07`
  - Stretch: make a `src/bin/collatz.rs` that reads `env::args()`.

**Week 1 takeaway:** ______________________________________________________

## Week 2 — Borrowing and data modeling

Streak: ☐ ☐ ☐ ☐ ☐ ☐ ☐

- [ ] **Day 08 — Borrowing** · `cargo test day08`
  - Stretch: do the compiler gym in `scratch.rs`, then fix it two ways.
- [ ] **Day 09 — Slices and strings** · `cargo test day09`
  - Stretch: rewrite `first_word` with `split_once(' ')`.
- [ ] **Day 10 — Structs and methods** · `cargo test day10`
  - Stretch: add a `translate` method.
- [ ] **Day 11 — Tuples, arrays, and `Vec`** · `cargo test day11`
  - Stretch: add a `median` helper using `Vec` sorting.
- [ ] **Day 12 — Enums and `Option`** · `cargo test day12`
  - Stretch: add `Shape::Square(f64)` and handle it.
- [ ] **Day 13 — `match` and patterns** · `cargo test day13`
  - Stretch: add a two-word command parser with `match`.
- [ ] **Day 14 — Checkpoint: RPN calculator** · `cargo test day14`
  - Stretch: add `%` and `^` operators.

**Week 2 takeaway:** ______________________________________________________

## Week 3 — Abstraction, errors, lifetimes

Streak: ☐ ☐ ☐ ☐ ☐ ☐ ☐

- [ ] **Day 15 — `Result` and `?`** · `cargo test day15`
  - Stretch: replace `String` errors with a custom error enum.
- [ ] **Day 16 — Panic vs `Result`** · `cargo test day16`
  - Stretch: write a `main` that reads a file with `?`.
- [ ] **Day 17 — Traits** · `cargo test day17`
  - Stretch: implement `Summary` for a `Tweet` and use `dyn Summary`.
- [ ] **Day 18 — Generics** · `cargo test day18`
  - Stretch: add a `Pair<T>` struct with a `larger` method.
- [ ] **Day 19 — Lifetimes** · `cargo test day19`
  - Stretch: add `first_line` with an elided lifetime.
- [ ] **Day 20 — Modules and crates** · `cargo test day20`
  - Stretch: call `math::gcd` from `day19` and add `pub(crate)`.
- [ ] **Day 21 — Checkpoint: generic `Stack<T>`** · `cargo test day21`
  - Stretch: add `iter()` and an `IntoIterator` impl.

**Week 3 takeaway:** ______________________________________________________

## Week 4 — Idiomatic Rust

Streak: ☐ ☐ ☐ ☐ ☐ ☐ ☐

- [ ] **Day 22 — Closures** · `cargo test day22`
  - Stretch: return a closure that captures a `String` with `move`.
- [ ] **Day 23 — Iterators** · `cargo test day23`
  - Stretch: rewrite one `for` loop from an earlier day as an iterator chain.
- [ ] **Day 24 — Collections** · `cargo test day24`
  - Stretch: repeat the word count with `BTreeMap` and compare ordering.
- [ ] **Day 25 — Smart pointers** · `cargo test day25`
  - Stretch: build an `Rc<RefCell<i32>>` counter.
- [ ] **Day 26 — Threads and shared state** · `cargo test day26`
  - Stretch: replace scoped threads with `Arc<Mutex>` or an `mpsc` channel.
- [ ] **Day 27 — Testing, formatting, lints** · `cargo test day27` + `cargo test --doc`
  - Stretch: add a `#[should_panic]` test and an `#[ignore]` test.
- [ ] **Day 28 — Checkpoint: word frequency** · `cargo test day28`
  - Stretch: read the file path from `env::args()`.

**Week 4 takeaway:** ______________________________________________________

## Week 5 — Capstone

Streak: ☐ ☐

- [ ] **Day 29 — Capstone I: matching lines** · `cargo test day29`
  - Stretch: add `-v` (invert match) with a test.
- [ ] **Day 30 — Capstone II: arguments and next steps** · `cargo test day30`
  - Stretch: add a `--count` flag to the CLI.
- [ ] **Capstone end-to-end** · `cargo run --bin grep -- -i rust README.md`
  - Expect matching lines and exit code 0. Try a missing pattern (exit 1) and a missing file (exit 2).

**Course complete** ☐

## After the course

Pick one:

- [ ] Read *The Rust Programming Language* chapters 4, 10, and 13.
- [ ] Finish `rustlings`.
- [ ] Build a JSON CLI with `serde` + `clap`.
- [ ] Build a small web service with `axum`.
- [ ] Try FFI: call a C function from Rust with `extern "C"`.

## Journal

Write one line per week — what clicked, what is still fuzzy:

- Week 1: ____________________________________________________________
- Week 2: ____________________________________________________________
- Week 3: ____________________________________________________________
- Week 4: ____________________________________________________________
- Week 5: ____________________________________________________________
