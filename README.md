# Sorting Benchmarks

![build status](https://github.com/jeremy-rifkin/Sorting-Benchmarks/workflows/build/badge.svg)
![tests status](https://github.com/jeremy-rifkin/Sorting-Benchmarks/workflows/tests/badge.svg)

Asymptotic complexity provides a lot of information about the behavior of an algorithm, however,
algorithms with the same complexity may perform very differently when it comes to physical hardware.
The canonical example is mergesort vs. heapsort (which has poor cache performance).

This project experimentally and rigorously benchmarks the hardware performance of various sorting
algorithms.

- [Sorting Benchmarks](#sorting-benchmarks)
- [Results](#results)
- [Benchmarking is Hard](#benchmarking-is-hard)
- [Benchmarking Strategy](#benchmarking-strategy)
- [Rust Performance](#rust-performance)
- [Significance and Planned Work](#significance-and-planned-work)

# Results

# Benchmarking is Hard

# Benchmarking Strategy

# Rust Performance

In pursuit of safety, rust inserts a number of runtime checks into code. These include
overflow/underflow checks (in debug mode) and array access boundary checks (in release and debug
modes), These checks are helpful for safety but are costly. The branch predictor will reduce cost
substantially, however, the checks are still not free and break up basic blocks. Below is a
comparison of insertionsort implemented with rust boundary checks, two rust insertionsort
implementations which use unsafe access to circumvent boundary checks, and an insertionsort
implementation in pure c.

Intel 4th gen i7, Ubuntu, gcc & rust llvm
```
Insertion sorts:
+-------------------------------+----------------------------+----------------------------+----------------------------+-----------------------------+---------+
|                               | 10                         | 100                        | 1,000                      | 10,000                      | 100,000 |
+-------------------------------+----------------------------+----------------------------+----------------------------+-----------------------------+---------+
| algos::insertionsort          | 0.00040 ± 0.00002 (5%)     | 0.00564 ± 0.00013 (2%)     | 0.43633 ± 0.00937 (2%)     | 37.29179 ± 0.10475 (0%)     | -       |
+-------------------------------+----------------------------+----------------------------+----------------------------+-----------------------------+---------+
| algos::insertionsort_unsafe   | 0.00037 ± 0.00002 (4%) s * | 0.00307 ± 0.00007 (2%) s * | 0.20391 ± 0.00541 (3%) s * | 17.69103 ± 0.06510 (0%)     | -       |
+-------------------------------+----------------------------+----------------------------+----------------------------+-----------------------------+---------+
| algos::insertionsort_unsafe_2 | 0.00036 ± 0.00002 (4%) s * | 0.00312 ± 0.00008 (3%) s * | 0.20370 ± 0.00500 (2%) s * | 17.62089 ± 0.04694 (0%)     | -       |
+-------------------------------+----------------------------+----------------------------+----------------------------+-----------------------------+---------+
| algos::insertionsort_c        | 0.00058 ± 0.00002 (4%)     | 0.00436 ± 0.00010 (2%)     | 0.19990 ± 0.00462 (2%) s * | 15.86337 ± 0.05298 (0%) s * | -       |
+-------------------------------+----------------------------+----------------------------+----------------------------+-----------------------------+---------+
└ Values in ms; 98% confidence interval displayed; s = statistically equal to fastest; * = within 5% of fastest
```

Intel 10th gen i7, Wandows, mingw-64 & rust llvm
```
Insertion sorts:
+-------------------------------+----------------------------+----------------------------+----------------------------+-----------------------------+---------+
|                               | 10                         | 100                        | 1,000                      | 10,000                      | 100,000 |
+-------------------------------+----------------------------+----------------------------+----------------------------+-----------------------------+---------+
| algos::insertionsort          | 0.00088 ± 0.00004 (5%) s * | 0.00434 ± 0.00007 (2%)     | 0.28608 ± 0.00142 (0%)     | 28.71438 ± 0.12226 (0%)     | -       |
+-------------------------------+----------------------------+----------------------------+----------------------------+-----------------------------+---------+
| algos::insertionsort_unsafe   | 0.00086 ± 0.00004 (5%) s * | 0.00294 ± 0.00009 (3%) s * | 0.13345 ± 0.00090 (1%) s * | 13.41843 ± 0.08444 (1%) s * | -       |
+-------------------------------+----------------------------+----------------------------+----------------------------+-----------------------------+---------+
| algos::insertionsort_unsafe_2 | 0.00089 ± 0.00005 (5%) s * | 0.00290 ± 0.00008 (3%) s * | 0.13480 ± 0.00075 (1%) s * | 13.90153 ± 0.11604 (1%)   * | -       |
+-------------------------------+----------------------------+----------------------------+----------------------------+-----------------------------+---------+
| algos::insertionsort_c        | 0.00104 ± 0.00005 (5%)     | 0.00395 ± 0.00009 (2%)     | 0.14623 ± 0.00065 (0%)     | 13.73471 ± 0.11514 (1%)   * | -       |
+-------------------------------+----------------------------+----------------------------+----------------------------+-----------------------------+---------+
└ Values in ms; 98% confidence interval displayed; s = statistically equal to fastest; * = within 5% of fastest
```

ARM1176, Raspbian, gcc & rust llvm
```
Insertion sorts:
+-------------------------------+----------------------------+----------------------------+----------------------------+------------------------------+
|                               | 10                         | 100                        | 1,000                      | 10,000                       |
+-------------------------------+----------------------------+----------------------------+----------------------------+------------------------------+
| algos::cocktail_shaker        | 0.01002 ± 0.00017 (2%)     | 0.06346 ± 0.00051 (1%)     | 5.25817 ± 0.02313 (0%)     | -                            |
+-------------------------------+----------------------------+----------------------------+----------------------------+------------------------------+
| algos::cocktail_shaker_unsafe | 0.00949 ± 0.00017 (2%)     | 0.05602 ± 0.00045 (1%)     | 4.49514 ± 0.01654 (0%)     | -                            |
+-------------------------------+----------------------------+----------------------------+----------------------------+------------------------------+
| algos::selectionsort          | 0.00900 ± 0.00000 (0%)   * | 0.05229 ± 0.00014 (0%)     | 4.07790 ± 0.00671 (0%)     | -                            |
+-------------------------------+----------------------------+----------------------------+----------------------------+------------------------------+
| algos::insertionsort          | 0.00942 ± 0.00019 (2%)     | 0.04698 ± 0.00040 (1%)     | 3.56845 ± 0.01596 (0%)     | -                            |
+-------------------------------+----------------------------+----------------------------+----------------------------+------------------------------+
| algos::insertionsort_unsafe   | 0.00900 ± 0.00000 (0%) s * | 0.04166 ± 0.00032 (1%)     | 3.06175 ± 0.01298 (0%)     | -                            |
+-------------------------------+----------------------------+----------------------------+----------------------------+------------------------------+
| algos::insertionsort_unsafe_2 | 0.00892 ± 0.00016 (2%) s * | 0.04151 ± 0.00035 (1%)     | 3.06357 ± 0.01368 (0%)     | -                            |
+-------------------------------+----------------------------+----------------------------+----------------------------+------------------------------+
| algos::insertionsort_c        | 0.01000 ± 0.00000 (0%)     | 0.03345 ± 0.00025 (1%) s * | 2.06649 ± 0.01074 (1%) s * | 255.24971 ± 1.00602 (0%) s * |
+-------------------------------+----------------------------+----------------------------+----------------------------+------------------------------+
└ Values in ms; 98% confidence interval displayed; s = statistically equal to fastest; * = within 5% of fastest
```

Using unsafe access, we're able to achieve performance on-par (and sometimes faster) than pure c.

I'm not sure what allows the unsafe rust implementation to out-perform the c implementation. It's on
the backlog to investigate the assembly to better understand that. Hypothesis:
- Rust optimizes better
- C is actually faster but there's an overhead to offset due to how the method call works (e.g. due
  to linkage and lack of inlining). This hypothesis would explain the results from `Intel 4th gen
  i7, Ubuntu, gcc & rust llvm` where the rust code out-performs c but only to a point.
- There are other factors contributing to the code performance: e.g. the layout of the code in the
  final executable. (As described by Emery Berger).

# Significance and Planned Work
