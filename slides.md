---
marp: true
theme: default
desc: |
    A fast-paced primer for DBAs and analysts who know Python + SQL, introducing
    just enough systems concepts to appreciate why Rust (with Rayon) can be so much faster.
Key Idea: |
    To understand why Rust crushes Python for heavy loops, we need to cover:
    types, compilers vs interpreters, the GIL, parallelism, iterators,
    memory safety, and benchmarking.
paginate: true
---

<!-- _class: lead -->
# 🦀 Why Rust is So Fast
## (A 101 Primer for Python + SQL Folks)

<!-- Everyone, sign up for github, fork the project and
lets get going
-->
---

## What is Rust?

<!-- class: lead -->
![bg left:60%](https://miro.medium.com/v2/resize:fit:996/format:webp/1*EyBu3xyslFozyfa_UUlK0w.gif)

- Systems Programming
- Memory Safety
- High Performance

---
# Our Starting Point

- Goal: Understand **just enough systems concepts** to see:
  - Why Python can feel slow
  - Why Rust + Rayon can feel blistering in comparison
- Topics:
  1. Types  
  2. Compiler vs Interpreter  
  3. GIL  
  4. Parallelism  
  5. Iterators  
  6. Memory Safety  
  7. Benchmarks  

---

# 1. Types (SQL vs Python vs Rust)

- SQL: `INT`, `BIGINT`, `VARCHAR`
- Python: `int` (arbitrary precision, flexible but slow)
- Rust: `u32`, `u64`, `String` (fixed size, predictable, fast)

💡 **Analogy:**  
SQL column types let the engine optimize.  
Rust’s fixed types let the compiler optimize.

---

# 2. Compilers vs Interpreters

- **Python:** Interpreted  
  Runs your code line by line at runtime  
  Like a live translator reading every sentence aloud.

- **Rust:** Compiled  
  Translates once into machine code before execution  
  Like having the whole book pre-translated → you read at native speed.

---

# 3. The GIL (Global Interpreter Lock)

- Python has a **lock** around execution
  - Only one thread can run Python bytecode at a time
- Threads don’t give true parallelism for CPU work
- Rust: **no GIL**, so all cores can run at once

💡 Great for *data crunching* tasks.

---

# 4. Parallelism vs Concurrency

- **Concurrency:** juggling many tasks (good for waiting on I/O)
- **Parallelism:** doing tasks *simultaneously* on multiple CPU cores
- Our histogram problem is **embarrassingly parallel**  
  → each element can be processed independently

---

# 5. Iterators and Map/Reduce

- SQL:  
  `SELECT bin, COUNT(*) FROM data GROUP BY bin;`
- Python: explicit `for` loops
- Rust: iterators → expressive pipelines
  - `iter().map(...).reduce(...)`
  - Rayon: `.par_iter()` for multi-core map/reduce

---

# 6. Memory Safety & Data Races

- C/Java: programmer must avoid races → hard and risky
- Python: GIL prevents races, but blocks speed
- Rust: compiler **proves at compile time**  
  → no two threads mutate the same data  
  → safe parallelism without locks
  → Roughly 70% of bugs in code at microsoft are memory related
<!--
SRC: https://www.zdnet.com/article/microsoft-70-percent-of-all-security-bugs-are-memory-safety-issues/

-->

---

# 7. Benchmarks & Measurement

- Why “felt fast on my laptop” is misleading
- Need **wall-clock time** and repeated trials
- Python: `time.perf_counter()`

---

# If it's so good, why isn't _everyone_ doing it?

They are!

- Ruff
- UV
- Polars

Are all written in rust! Other popular tools like
**numpy** and **duckdb** also rely on other system prgoramming languages to support their computation speed!

---

# Rust Syntax

- Quick tour: variables, control flow, and functions.
- Immutable by default; use `mut` to opt into mutation.

---

## Variables — immutable

```rust
let x = 10; // immutable binding
println!("{}", x);
```

---

## Variables — mutable

```rust
let mut y = 20;
y += 5;
println!("{}", y);
```

---

## Shadowing

```rust
let x = 5;
let x = x + 1; // new binding shadows old one
println!("{}", x);
```

---

## If as an expression

```rust
let n = 3;
let parity = if n % 2 == 0 { "even" } else { "odd" };
println!("{}", parity);
```

---

## For loop (range)

```rust
for i in 0..3 { // 0,1,2
  println!("{}", i);
}
```

---

## While loop

```rust
let mut i = 0;
while i < 3 { i += 1; }
```

---

## Functions — signature & return

```rust
fn add(a: i32, b: i32) -> i32 {
  a + b // last expression is returned
}
let s = add(2, 3);
```

---

## Iterators (brief)

```rust
let v = vec![1,2,3];
let doubled: Vec<_> = v.iter().map(|x| x * 2).collect();
```

---

## Parallel hint

- To parallelize similar iterator chains use Rayon:
  - `.iter()` -> `.par_iter()` (add `rayon` crate)
  - Keep data ownership / borrowing rules in mind

---
