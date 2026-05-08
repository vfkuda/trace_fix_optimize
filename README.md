# Into

Модуль про инструментирование, профилирование и оптимизацию Rust-кода. Требует демонстрации умения проведения динамического анализа с отладчиком, профилировщиком и другими инструментами. 

## Задача
- найти и починить ошибки, 
- подтвердить отсутствие UB, и
- ускорить критичные участки.

# Toolkit
- rust nightly
```
rustup toolchain install nightly
rustup update nightly
```
- better testing
```
cargo install cargo-nextest 
```
- test coverage
```
rustup component add llvm-tools-preview
cargo install cargo-llvm-cov
```
- miri
```
rustup component add miri --toolchain nightly
cargo +nightly miri setup
```
- valgrind 

> [!NOTE] 
> valgrind is unavailable/unstable on Apple Silicon. UB detection of interest 
> - Use-after-free
> - Memory leaks
> - Data races
> - etc
> can be made by ASan + TSan sanitizers.

- debugger
  
  Use CodeLLDB vscode extention
> [!TIP]
> Additional tips for better debugging. Add to launch.json for
> - ability to step into Rust std library 
> - better Rust-specific variables printing
> ```
>      "preRunCommands": [
>        "settings clear target.process.thread.step-avoid-regexp",
>        "command script import ~/[rust_prettifier_for_lldb.py](https://github.com>/cmrschwarz/rust-prettifier-for-lldb)"
>      ]
> ```  
# Actions Log

## 1. try cargo check
| Diagnostic | Result | Action |
|---|---|---|
| `cargo c` | code warnings | fix, [commit "fix warnings"](https://github.com/vfkuda/trace_fix_optimize/commit/1c8fc5abde9a3a650121de6f236245205bc671f8) |
| `cargo c` | passed | |

## 2. try cargo test
| Diagnostic | Result | Action |
|---|---|---|
| `cargo t` | panic | fix, [commit "fix panic"](https://github.com/vfkuda/trace_fix_optimize/commit/e438f9340a2a822eb8ec1b43e03bfc8b2dcddad6)|

- panic message
```
thread 'sums_even_numbers' (2174288) panicked at src/lib.rs:11:29:
unsafe precondition(s) violated: slice::get_unchecked requires that the index is within the slice
```

| Diagnostic | Result | Action |
|---|---|---|
| `cargo t` | test assert issue | fix, [commit "fix averages_only_positive"](https://github.com/vfkuda/trace_fix_optimize/commit/8fdd9fd186b0f1d5a351f540b65493c9230021a4)|

- asset message:
```
---- averages_only_positive stdout ----

thread 'averages_only_positive' (2177315) panicked at tests/integration.rs:36:5:
assertion failed: (broken_app::average_positive(&nums) - 10.0).abs() < f64::EPSILON
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

| Diagnostic | Result | Action |
|---|---|---|
| `cargo t` | passed | |

## 3. check test coverage
| Diagnostic | Result | Action |
|---|---|---|
| ```./scripts/coverage.sh``` | [./artifacts/llvm-cov/*](artifacts/llvm-cov/html/index.html) | added tests, [commit "add tests"](https://github.com/vfkuda/trace_fix_optimize/commit/ca836f579081c7ce4f6cb0a73732c8cb3e8844d1)

## 4. try diagnostics for new tests
### Diagnostic
```
PHASE=4
mkdir ./artifacts/$PHASE
scripts/test.sh   2> ./artifacts/$PHASE/testrun.txt
scripts/miri.sh   2> ./artifacts/$PHASE/miri.txt
scripts/asn.sh    2> ./artifacts/$PHASE/asn.txt
scripts/tsn.sh    2> ./artifacts/$PHASE/tsn.txt
``` 

### Results 
* Artifacts:
  - [artifacts/4/*](./artifacts/4/)
* Findings:
  - UB: data race @ `concurrency::race_increment_test` 
  - UB: memory access failed @ `broken_app::use_after_free`
  - UB: memory leak @ `broken_app::leak_buffer`

### Action
  fix UBs, [commit "fix UBs"](https://github.com/vfkuda/trace_fix_optimize/commit/3d327f00f3e66ac68d1aa1cedd588d015f2621c7)

## 5. try diagnostics after UBs fix
### Diagnostic
```
PHASE=5
mkdir ./artifacts/$PHASE
scripts/test.sh   2> ./artifacts/$PHASE/testrun.txt
scripts/miri.sh   2> ./artifacts/$PHASE/miri.txt
scripts/asn.sh    2> ./artifacts/$PHASE/asn.txt
scripts/tsn.sh    2> ./artifacts/$PHASE/tsn.txt
``` 
### Results 
no issues

## 6. try profiling
### Diagnostic
``` 
scripts/benchbase.sh > artifacts/baseline_before.txt
scripts/profile.sh -o artifacts/flamegraph.6.svg 
```
### Result
- [artifacts/baseline_before.txt](./artifacts/baseline_before.txt)
- [artifacts/flamegraph.6.svg](./artifacts/flamegraph.6.svg)
- decision to optimize `slow_fib`

### Action
1. pre benchmarking ```cargo bench --bench criterion -- slow_fib_broken```
2. optimize, ```cargo t``` -> passed 
3. post benchmarking ```cargo bench --bench criterion -- slow_fib_broken```
4. [commit "fix slow_fib"](https://github.com/vfkuda/trace_fix_optimize/commit/c99df0b890b8b3a65d338e72cf46d9585184d2f0)

## 7. repeat profiling
### Diagnostic
``` 
scripts/profile.sh -o artifacts/flamegraph.7.svg 
```
### Result
Decision to optimize 
- `broken_app::slow_dedup`
- `broken_app::leak_buffer`
- `broken_app::normalize`

### 7.1 slow_dedup : Action 
1. pre benchmarking ```cargo bench --bench criterion -- slow_dedup_broken```
2. optimize, ```cargo t``` -> passed 
3. post benchmarking ```cargo bench --bench criterion -- slow_dedup_broken```
4. [commit "fix slow_dedup"](https://github.com/vfkuda/trace_fix_optimize/commit/7a2676b5096a5fa644faf06503949b49678db6bf)

### 7.2 leak_buffer : Action 
1. add benchmark
2. pre benchmarking ```cargo bench --bench criterion -- leak_buffer_broken```
3. optimize, ```cargo t``` -> passed 
4. post benchmarking ```cargo bench --bench criterion -- leak_buffer_broken```
5. [commit "fix slow leak_buffer"](https://github.com/vfkuda/trace_fix_optimize/commit/6556ee923aa8f354bb23487593602a2245c9aa62)

### 7.3 normalize : Action 
1. add benchmark
2. pre benchmarking ```cargo bench --bench criterion -- slow_normalize```
3. optimize, ```cargo t``` -> passed 
4. post benchmarking ```cargo bench --bench criterion -- slow_normalize```
5. [commit "fix slow normalize"](https://github.com/vfkuda/trace_fix_optimize/commit/5d7807b51ca834b684c46a147d1deaf577188e1a)

## 8. final check
### Diagnostic
```
# tests
PHASE=8
mkdir ./artifacts/$PHASE
scripts/test.sh   2> ./artifacts/$PHASE/testrun.txt
scripts/miri.sh   2> ./artifacts/$PHASE/miri.txt
scripts/asn.sh    2> ./artifacts/$PHASE/asn.txt
scripts/tsn.sh    2> ./artifacts/$PHASE/tsn.txt

# profile
scripts/profile.sh -o artifacts/flamegraph.8.svg 

# baseline after optimizations
scripts/benchbase.sh > artifacts/baseline_after.txt
```

### Result
- [`artifacts/8/*`](./artifacts/8/) final test results - all passed successfully
- [`artifacts/flamegraph.8.svg`](./artifacts/flamegraph.8.svg) - final flamegraph
- [`artifacts/baseline_after.txt`](./artifacts/baseline_after.txt) - final performance results, compare to [`artifacts/baseline_before.txt`](./artifacts/baseline_before.txt)
- [`artifacts/criterion`](./artifacts/criterion/report/index.html) - criterion benhmark results report



