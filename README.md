# Sorting Benchmarks [Draft]

![build status](https://github.com/jeremy-rifkin/Sorting-Benchmarks/workflows/build/badge.svg)
![tests status](https://github.com/jeremy-rifkin/Sorting-Benchmarks/workflows/tests/badge.svg)

Asymptotic complexity provides a lot of information about the behavior of an algorithm, however,
algorithms with the same complexity may perform very differently when it comes to physical hardware.
The canonical example is mergesort vs. heapsort (which has poor cache performance).

This goal of this project is to rigorously benchmarks the hardware performance of various
algorithms.

The applications of work like this would be in dialing in thresholds for hybrid algorithms (i.e. at
what point in introsort do you switch to insertionsort). At the moment this application only runs
algorithms on various input sizes. I would like to later add functionality to systematically alter
algorithm parameters.

Then again, who cares about performance-quibbling with sorting algorithms 🤷‍♂️

## Table of Contents
- [Benchmarking is Hard](#benchmarking-is-hard)
	- [Benchmarking Strategy](#benchmarking-strategy)
- [Algorithms Tested](#algorithms-tested)
- [Results](#results)
- [Findings](#findings)
- [Appendix](#appendix)
	- [Rust Performance](#rust-performance)
	- [More work](#more-work)

# Benchmarking is Hard

As Emery Berger points out in his talk ["Performance Matters"] there are a ton of complex hardware
and software factors that effect application performance. These include caching, branch prediction,
random layout of an executable's binary in memory.

These factors are difficult to address. Berger presented a tool called [Coz] that addresses some of
these challenges in the context of application profiling and identifying targets for performance
improvement. Berger also presented a tool called [Stabilizer] which modifies executable layout
during runtime to eliminate the effect binary layout in memory. The effects of code layout on
performance are largely due to cache / branch predictor conflict misses (pieces of code that execute
frequently and are used in conjunction conflict with each other). I don't know how substantially
this effects modern cpus (modern amd and intel processors use multi-way set-associative caches
[\[1\]][1][\[2\]][2][\[3\]][3][\[4\]][4]), however, it would explain some anomalies we've observed
while developing this. I'd like to be able to use stabilizer in this project, however, it is a
compile-time plugin for C/C++/Fortran and getting it to work with Rust is outside the scope of this
project.

Here's some of the issues we've encountered:
- In 200 runs with mean runtime ~2,200ns and a very tight distribution, there was an outlier of
  112,600ns. This result blew up the standard deviation calculation. This happened on a couple of
  algorithms on a couple of test-sizes every run but there was no pattern to which algorithms or
  when it would happen. Because of the extreme nature of these outliers and sporadic occurrence, the
  outliers are simply discarded (using Tukey's method).
- We'd get really tight distributions (98% CI == +/- 0%) for the performance of some algorithms
  however, on the next execution of the program, even without code changes, we'd get substantially
  different results. The p-value of a 2-sample t-test for these instances is 0.

Here are some of the factors contributing to benchmarking challenges:
- Cache
- Branch prediction
- Linker layout
- OS task scheduling
- CPU throttling

## Benchmarking Strategy

Here are some of the techniques used for mitigation:
- Instead of running all insertion sorts size=1,000 then all selection sorts size=1,000 etc. and
  everything sequentially, every single individual run for every algorithm and test size is setup
  and randomly shuffled. Then a thread pool begins performing benchmarks from the problem pool.
  This is an attempt to improve independence between individual runs.
- ~~OS calls are made to request preferential scheduling. This should improve consistency.~~
  Currently not performed for lack of effect.
- Threads sleep briefly between benchmark runs and only N_Cores / 2 threads are spun up. This is to
  help prevent thermal throttling and improve consistency of cache performance.

We experimented with running a cache buster between every benchmark execution (writing to a massive
block of memory to flush out the cache). However, this has been discarded because it was not highly
effective at addressing benchmarking issues, was very slow, and would be problematic in a
multi-threaded context.

# Algorithms Tested

We've implemented numerous algorithms, variations on algorithms, and hybrid algorithms to test.

Algorithm list: bubblesort, cocktail-shaker sort, selectionsort, insertionsort, shellsort (many
variations for different gap sequences), heapsort (top-down and bottom-up variations), mergesort,
quicksort, radixsort.

We've put effort into optimizing all these algorithms, however, there is room for improvement.
Rust's builtin `sort_unstable` substantially out-performs all of our algorithms (with the exception
of radixsort). It's ok, though, that our implementations aren't as fast as rust's builtin. We're
just interested in how our imperfectly implemented algorithms compare to each other.

# Results

# Findings

write more about results here...

# Appendix

## Rust Performance

// Rust performs boundary checks on every array access. This causes a substantial performance hit
// (and may effect branch prediction). The various sorting algorithms we've implemented are much
// slower than rust's built-in sorting algorithms. This is partially due to unsafe array accesses
// (rust's built-in algorithms use unsafe accesses to disable boundary checks), and we aim to have
// everything implemented with unsafe accesses. Rust's sorting algorithms are also faster because
// they are advanced hybrid sorting algorithms and have had much more work put into optimizing them
// than we've put into optimizing ours. We hope to get our algorithms to have comparable performance
// to rust's builtin, but it isn't strictly necessary for our program: we just want to look at how
// algorithms compare to each other on real hardware.

In pursuit of safety, rust inserts a number of runtime checks into code. These include
overflow/underflow checks (in debug mode) and array access boundary checks (in release and debug
modes), These checks are helpful for safety but are costly and result in a substantial performance
hit. The branch predictor will reduce cost, however, the checks are still slow, break up basic
blocks, and more branch instructions in the code may introduce conflicts in the cpu branch table.

To avoid array boundary checks, we have implemented all sorting algorithms with unchecked array
accesses.

Below is a comparison of insertionsort implemented with rust boundary checks, two rust insertionsort
implementations which use unsafe access to circumvent boundary checks, and an insertionsort
implementation in pure C.

**Intel 10th gen i7, Wandows, mingw-64 & rust llvm**
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

**Intel 4th gen i7, Ubuntu, gcc & rust llvm**
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

**ARM1176, Raspbian, gcc & rust llvm**
```
Insertion sorts:
+-------------------------------+----------------------------+----------------------------+----------------------------+------------------------------+---------+
|                               | 10                         | 100                        | 1,000                      | 10,000                       | 100,000 |
+-------------------------------+----------------------------+----------------------------+----------------------------+------------------------------+---------+
| algos::insertionsort          | 0.00942 ± 0.00019 (2%)     | 0.04698 ± 0.00040 (1%)     | 3.56845 ± 0.01596 (0%)     | -                            | -       |
+-------------------------------+----------------------------+----------------------------+----------------------------+------------------------------+---------+
| algos::insertionsort_unsafe   | 0.00900 ± 0.00000 (0%) s * | 0.04166 ± 0.00032 (1%)     | 3.06175 ± 0.01298 (0%)     | -                            | -       |
+-------------------------------+----------------------------+----------------------------+----------------------------+------------------------------+---------+
| algos::insertionsort_unsafe_2 | 0.00892 ± 0.00016 (2%) s * | 0.04151 ± 0.00035 (1%)     | 3.06357 ± 0.01368 (0%)     | -                            | -       |
+-------------------------------+----------------------------+----------------------------+----------------------------+------------------------------+---------+
| algos::insertionsort_c        | 0.01000 ± 0.00000 (0%)     | 0.03345 ± 0.00025 (1%) s * | 2.06649 ± 0.01074 (1%) s * | 255.24971 ± 1.00602 (0%) s * | -       |
+-------------------------------+----------------------------+----------------------------+----------------------------+------------------------------+---------+
└ Values in ms; 98% confidence interval displayed; s = statistically equal to fastest; * = within 5% of fastest
```

**Conclusion:** Using unchecked access, we're able to achieve performance on-par (and sometimes
faster) than pure C.

I'm not sure what allows the unchecked rust implementation to out-perform the C implementation. It's
on the backlog to investigate the generated assembly and better understand what's going on here.
Hypotheses:
- `rustc` optimizes better than `gcc`.
- C is actually faster but there's an overhead to offset due to how the method call works (e.g. due
  to linkage and lack of inlining). This hypothesis would explain the results from `Intel 4th gen
  i7, Ubuntu, gcc & rust llvm` where the rust code out-performs C but only to a point.
- There are [other factors](#benchmarking-is-hard) contributing to the code performance: e.g. the
  layout of the code in the final executable.

## More work

General Backlog:
- Investigate generated assembly for the various insertion sorts.
- Convert all rust algorithm implementations to use unsafe access.
- Add hybrid sorting algorithms to test. Introsort and timsort are of interest.
- Generate some graphs, not just tables.
- Performance of hash table implementations


["Performance Matters"]: https://www.youtube.com/watch?v=r-TLSBdHe1A
[Coz]: https://github.com/plasma-umass/coz
[Stabilizer]: https://github.com/ccurtsinger/stabilizer

[1]: https://stackoverflow.com/questions/20333547/cache-specifications-for-intel-core-i7
[2]: https://stackoverflow.com/questions/49092541/which-cache-mapping-technique-is-used-in-intel-core-i7-processor
[3]: https://en.wikichip.org/wiki/amd/microarchitectures/zen
[4]: https://en.wikichip.org/wiki/amd/microarchitectures/zen_2


