# Solutions

Try the exercise first — the compiler and tests are the real teacher.

Each entry shows the body that replaces `todo!("implement me")`.

## Day 01 — Cargo and your first function

### `greet(name: &str) -> String`

`src/days/day01.rs`

```rust
format!("Hello, {name}!")
```


## Day 02 — Variables, mutability, shadowing

### `running_total(values: &[i32]) -> i32`

`src/days/day02.rs`

```rust
let mut total = 0;
for &value in values {
    total += value;
}
total
```


## Day 03 — Types, casts, and overflow

### `celsius_to_fahrenheit(celsius: f64) -> f64`

`src/days/day03.rs`

```rust
celsius * 9.0 / 5.0 + 32.0
```

### `wrapping_add(a: u8, b: u8) -> u8`

`src/days/day03.rs`

```rust
a.wrapping_add(b)
```


## Day 04 — Functions and expressions

### `clamp(value: i32, lo: i32, hi: i32) -> i32`

`src/days/day04.rs`

```rust
if value < lo {
    lo
} else if value > hi {
    hi
} else {
    value
}
```

### `sign(n: i32) -> &'static str`

`src/days/day04.rs`

```rust
if n < 0 {
    "negative"
} else if n > 0 {
    "positive"
} else {
    "zero"
}
```


## Day 05 — Control flow

### `collatz_steps(mut n: u64) -> u32`

`src/days/day05.rs`

```rust
let mut steps = 0;
while n != 1 {
    n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
    steps += 1;
}
steps
```


## Day 06 — Ownership, part 1: move vs copy

### `join_words(words: Vec<String>) -> String`

`src/days/day06.rs`

```rust
words.join(" ")
```


## Day 07 — Checkpoint: functions, loops, and `Vec`

### `collatz_sequence(n: u64) -> Vec<u64>`

`src/days/day07.rs`

```rust
let mut sequence = vec![n];
let mut current = n;
while current != 1 {
    current = if current % 2 == 0 {
        current / 2
    } else {
        3 * current + 1
    };
    sequence.push(current);
}
sequence
```


## Day 08 — Borrowing

### `append_len(dst: &mut String, src: &str) -> usize`

`src/days/day08.rs`

```rust
dst.push_str(src);
dst.len()
```


## Day 09 — Slices and strings

### `first_word(s: &str) -> &str`

`src/days/day09.rs`

```rust
match s.find(' ') {
    Some(index) => &s[..index],
    None => s,
}
```


## Day 10 — Structs and methods

### `area(&self) -> f64`

`src/days/day10.rs`

```rust
self.width * self.height
```

### `scale(&mut self, factor: f64)`

`src/days/day10.rs`

```rust
self.width *= factor;
self.height *= factor;
```

### `is_square(&self) -> bool`

`src/days/day10.rs`

```rust
(self.width - self.height).abs() < 1e-9
```


## Day 11 — Tuples, arrays, and `Vec`

### `stats(xs: &[i32]) -> (i32, i32, f64)`

`src/days/day11.rs`

```rust
if xs.is_empty() {
    return (0, 0, 0.0);
}

let mut min = xs[0];
let mut max = xs[0];
let mut sum: i64 = 0;

for &x in xs {
    min = min.min(x);
    max = max.max(x);
    sum += x as i64;
}

(min, max, sum as f64 / xs.len() as f64)
```


## Day 12 — Enums and `Option`

### `area(shape: &Shape) -> f64`

`src/days/day12.rs`

```rust
match shape {
    Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
    Shape::Rectangle { width, height } => width * height,
    Shape::Triangle { base, height } => 0.5 * base * height,
}
```

### `find_first_even(xs: &[i32]) -> Option<i32>`

`src/days/day12.rs`

```rust
for &x in xs {
    if x % 2 == 0 {
        return Some(x);
    }
}
None
```


## Day 13 — `match` and patterns

### `fizzbuzz(n: u32) -> String`

`src/days/day13.rs`

```rust
match (n % 3, n % 5) {
    (0, 0) => "FizzBuzz".to_string(),
    (0, _) => "Fizz".to_string(),
    (_, 0) => "Buzz".to_string(),
    _ => n.to_string(),
}
```

### `describe(n: i32) -> &'static str`

`src/days/day13.rs`

```rust
match n {
    0 => "zero",
    x if x < 0 => "negative",
    1..=9 => "single digit",
    10..=99 => "double digit",
    _ => "large",
}
```


## Day 14 — Checkpoint: RPN calculator

### `eval(tokens: &[&str]) -> Option<i64>`

`src/days/day14.rs`

```rust
let mut stack: Vec<i64> = Vec::new();

for token in tokens {
    match *token {
        "+" => {
            let b = stack.pop()?;
            let a = stack.pop()?;
            stack.push(a + b);
        }
        "-" => {
            let b = stack.pop()?;
            let a = stack.pop()?;
            stack.push(a - b);
        }
        "*" => {
            let b = stack.pop()?;
            let a = stack.pop()?;
            stack.push(a * b);
        }
        "/" => {
            let b = stack.pop()?;
            let a = stack.pop()?;
            if b == 0 {
                return None;
            }
            stack.push(a / b);
        }
        number => stack.push(number.parse().ok()?),
    }
}

if stack.len() == 1 { stack.pop() } else { None }
```


## Day 15 — `Result` and the `?` operator

### `parse_pair(input: &str) -> Result<(i32, i32), String>`

`src/days/day15.rs`

```rust
let (left, right) = input
    .split_once(',')
    .ok_or_else(|| format!("missing comma in {input:?}"))?;

let a = left
    .trim()
    .parse::<i32>()
    .map_err(|e| format!("bad left number: {e}"))?;
let b = right
    .trim()
    .parse::<i32>()
    .map_err(|e| format!("bad right number: {e}"))?;

Ok((a, b))
```


## Day 16 — Panic vs `Result`

### `average(xs: &[i64]) -> Result<f64, String>`

`src/days/day16.rs`

```rust
if xs.is_empty() {
    return Err("cannot average an empty slice".to_string());
}

let sum: i64 = xs.iter().sum();
Ok(sum as f64 / xs.len() as f64)
```


## Day 17 — Traits

### `summarize(&self) -> String`

`src/days/day17.rs`

```rust
format!("{} by {}", self.title, self.author)
```


## Day 18 — Generics

### `largest<T: PartialOrd + Copy>(list: &[T]) -> Option<T>`

`src/days/day18.rs`

```rust
let mut largest = *list.first()?;

for &item in list {
    if item > largest {
        largest = item;
    }
}

Some(largest)
```


## Day 19 — Lifetimes

### `longest<'a>(x: &'a str, y: &'a str) -> &'a str`

`src/days/day19.rs`

```rust
if x.len() >= y.len() { x } else { y }
```


## Day 20 — Modules and crates

### `gcd(mut a: u64, mut b: u64) -> u64`

`src/days/day20.rs`

```rust
while b != 0 {
    let remainder = a % b;
    a = b;
    b = remainder;
}
a
```


## Day 21 — Checkpoint: generic `Stack<T>`

### `push(&mut self, value: T)`

`src/days/day21.rs`

```rust
self.items.push(value);
```

### `pop(&mut self) -> Option<T>`

`src/days/day21.rs`

```rust
self.items.pop()
```

### `peek(&self) -> Option<&T>`

`src/days/day21.rs`

```rust
self.items.last()
```

### `len(&self) -> usize`

`src/days/day21.rs`

```rust
self.items.len()
```

### `is_empty(&self) -> bool`

`src/days/day21.rs`

```rust
self.items.is_empty()
```


## Day 22 — Closures

### `sort_by_abs(xs: &mut [i32])`

`src/days/day22.rs`

```rust
xs.sort_by_key(|x| x.abs());
```

### `apply_to_all(xs: &[i32], f: impl Fn(i32) -> i32) -> Vec<i32>`

`src/days/day22.rs`

```rust
xs.iter().map(|&x| f(x)).collect()
```


## Day 23 — Iterators

### `sum_of_even_squares(xs: &[i32]) -> i32`

`src/days/day23.rs`

```rust
xs.iter().filter(|&&x| x % 2 == 0).map(|&x| x * x).sum()
```

### `first_over(xs: &[i32], limit: i32) -> Option<i32>`

`src/days/day23.rs`

```rust
xs.iter().copied().find(|&x| x > limit)
```


## Day 24 — Collections

### `word_count(text: &str) -> HashMap<String, usize>`

`src/days/day24.rs`

```rust
let mut counts = HashMap::new();

for word in text.split_whitespace() {
    *counts.entry(word.to_lowercase()).or_insert(0) += 1;
}

counts
```


## Day 25 — Smart pointers: `Box`, `Rc`, `RefCell`, `Arc`

### `sum(list: &List) -> i32`

`src/days/day25.rs`

```rust
match list {
    List::Cons(value, next) => value + sum(next),
    List::Nil => 0,
}
```


## Day 26 — Threads and shared state

### `parallel_sum(xs: Vec<i64>) -> i64`

`src/days/day26.rs`

```rust
if xs.is_empty() {
    return 0;
}

let chunk_size = xs.len().div_ceil(4);

std::thread::scope(|scope| {
    let handles: Vec<_> = xs
        .chunks(chunk_size)
        .map(|chunk| scope.spawn(move || chunk.iter().sum::<i64>()))
        .collect();

    handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .sum()
})
```


## Day 27 — Testing, formatting, and lints

### `is_prime(n: u64) -> bool`

`src/days/day27.rs`

```rust
if n < 2 {
    return false;
}

let mut divisor = 2;
while divisor * divisor <= n {
    if n % divisor == 0 {
        return false;
    }
    divisor += 1;
}

true
```


## Day 28 — Checkpoint: word frequency

### `top_words(text: &str, n: usize) -> Vec<(String, usize)>`

`src/days/day28.rs`

```rust
let mut counts: HashMap<String, usize> = HashMap::new();

for word in text.split_whitespace() {
    *counts.entry(word.to_lowercase()).or_insert(0) += 1;
}

let mut pairs: Vec<(String, usize)> = counts.into_iter().collect();
pairs.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
pairs.truncate(n);
pairs
```


## Day 29 — Capstone I: matching lines

### `filter_lines<'a>(text: &'a str, needle: &str, ignore_case: bool) -> Vec<&'a str>`

`src/days/day29.rs`

```rust
if ignore_case {
    let needle = needle.to_lowercase();
    text.lines()
        .filter(|line| line.to_lowercase().contains(&needle))
        .collect()
} else {
    text.lines().filter(|line| line.contains(needle)).collect()
}
```


## Day 30 — Capstone II: argument parsing and next steps

### `parse_args(args: &[String]) -> Result<Args, String>`

`src/days/day30.rs`

```rust
let mut ignore_case = false;
let mut positional = Vec::new();

for arg in args.iter().skip(1) {
    match arg.as_str() {
        "-i" => ignore_case = true,
        flag if flag.starts_with('-') => return Err(format!("unknown flag: {flag}")),
        value => positional.push(value.to_string()),
    }
}

if positional.len() != 2 {
    return Err("usage: grep [-i] <pattern> <path>".to_string());
}

Ok(Args {
    pattern: positional.remove(0),
    path: positional.remove(0),
    ignore_case,
})
```

