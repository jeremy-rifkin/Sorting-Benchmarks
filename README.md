# Sorting Benchmarks

![build status](https://github.com/jeremy-rifkin/Sorting-Benchmarks/workflows/build/badge.svg)
![tests status](https://github.com/jeremy-rifkin/Sorting-Benchmarks/workflows/tests/badge.svg)
[![codecov](https://codecov.io/gh/jeremy-rifkin/Sorting-Benchmarks/branch/master/graph/badge.svg?token=GIMYUWGT2H)](https://codecov.io/gh/jeremy-rifkin/Sorting-Benchmarks)

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
	- [Future Work](#future-work)
- [Appendix](#appendix)
	- [Rust Performance](#rust-performance)

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

Algorithms are tested on a series of array sizes. For each array and each test size, 200 tests with
randomly generated arrays are performed. Every algorithm is tested with the same 200 randomly
generated arrays.

Instead of running all insertion sorts size=1,000 then all selection sorts size=1,000 etc. and
everything sequentially, every single individual run for every algorithm and test size is setup and
randomly shuffled. Then a thread pool begins performing benchmarks from the problem pool. This is an
attempt to improve independence between individual runs.

Threads sleep briefly between benchmark runs and only N_Cores / 2 threads are spun up. This is to
help prevent thermal throttling and improve consistency of cache performance.

We experimented with running a cache buster between every benchmark execution (writing to a massive
block of memory to flush out the cache). However, this has been discarded because it was not highly
effective at addressing benchmarking issues, was very slow, and would be problematic in a
multi-threaded context.

# Algorithms Tested

We've implemented numerous algorithms, variations on algorithms, and hybrid algorithms to test.

Algorithm list: bubblesort, cocktail-shaker sort, selectionsort, insertionsort, shellsort, heapsort,
mergesort, quicksort, introsort, timsort, and radixsort. Some of these have many variations: for
example heapsort has top-down and bottom-up implementations and shellsort has several variations for
various gap sequences.

We've put effort into optimizing all these algorithms, however, there is room for improvement.
Rust's builtin `sort_unstable` substantially out-performs all of our algorithms (with the exception
of radixsort). It's ok, though, that our implementations aren't as fast as rust's builtin. We're
just interested in how our imperfectly implemented algorithms compare to each other.

# Results

We've ran tests on four different systems:
- Intel 4th gen i7, Ubuntu, gcc & rust llvm
- Intel 10th gen i7, Wandows, mingw-64 & rust llvm
- ARM11, Raspbian, gcc & rust llvm (raspberry pi zero)
- ARMv7, Raspbian, gcc & rust llvm (raspberry pi 3 B+)

The raw results can be found [here](results/).

## Findings

### Heapsort

```
Heap sorts:
+---------------------------+----------------------------+----------------------------+----------------------------+----------------------------+----------------------------+------------------------------+
|                           | 10                         | 100                        | 1,000                      | 10,000                     | 100,000                    | 1,000,000                    |
+---------------------------+----------------------------+----------------------------+----------------------------+----------------------------+----------------------------+------------------------------+
| algos::heapsort_bottom_up | 0.00085 ± 0.00005 (6%) s * | 0.00458 ± 0.00011 (2%) s * | 0.06013 ± 0.00274 (5%) s * | 0.68644 ± 0.01199 (2%) s * | 8.78124 ± 0.12426 (1%) s * | 115.52760 ± 2.13309 (2%) s * |
+---------------------------+----------------------------+----------------------------+----------------------------+----------------------------+----------------------------+------------------------------+
| algos::heapsort_top_down  | 0.00086 ± 0.00005 (6%) s * | 0.00458 ± 0.00013 (3%) s * | 0.06315 ± 0.00322 (5%) s   | 0.71443 ± 0.01505 (2%)   * | 8.86477 ± 0.13188 (1%) s * | 119.27459 ± 3.09581 (3%) s * |
+---------------------------+----------------------------+----------------------------+----------------------------+----------------------------+----------------------------+------------------------------+
└ Values in ms; 98% confidence interval displayed; s = statistically equal to fastest; * = within 5% of fastest
```
(performance on **Intel 10th gen i7, Wandows, mingw-64 & rust llvm**)

Bottom-up heap construction is often touted as the "best" way to go about heap construction. This is
because asymptotically, bottom-up construction can be done in `O(n)` time while top-down requires
`O(n log n)` time. On all systems tested and almost every input size we observed no statistical
difference between the performance of the two heap construction techniques. We never observed a
substantial difference between the two methods (i.e. >5% diff) and in one case where there was
statistical difference, top-down construction out-performed bottom-up construction (test_size =
`100,000` on arm11).

*Conclusion:* The only advantage of using bottom-up construction over top-down construction is that
it requires only a sink method and not a swim method.

Why isn't there a performance difference here? The asymptotic complexity says there should be! Two
hypothesis:
- The runtime of heapsort is dominated by the sort-down step.
- Any performance improvement by bottom-up construction is completely dwarfed by heapsort's terrible
  cache performance.

### Shellsort

```
Shell sorts:
+----------------------------------------+----------------------------+----------------------------+----------------------------+-----------------------------+-----------------------------+------------------------------+
|                                        | 10                         | 100                        | 1,000                      | 10,000                      | 100,000                     | 1,000,000                    |
+----------------------------------------+----------------------------+----------------------------+----------------------------+-----------------------------+-----------------------------+------------------------------+
| algos::insertionsort                   | 0.00066 ± 0.00005 (7%) s * | 0.00268 ± 0.00009 (4%) s * | 0.11508 ± 0.00514 (4%)     | 8.67249 ± 0.12000 (1%)      | -                           | -                            |
+----------------------------------------+----------------------------+----------------------------+----------------------------+-----------------------------+-----------------------------+------------------------------+
| algos::shellsort_knuth                 | 0.00166 ± 0.00007 (4%)     | 0.00424 ± 0.00012 (3%)     | 0.05342 ± 0.00263 (5%) s   | 0.70777 ± 0.01299 (2%)      | 9.59906 ± 0.12913 (1%)      | 122.95488 ± 2.42832 (2%)     |
+----------------------------------------+----------------------------+----------------------------+----------------------------+-----------------------------+-----------------------------+------------------------------+
| algos::shellsort_sedgewick82           | 0.00079 ± 0.00006 (8%)     | 0.00326 ± 0.00008 (2%)     | 0.05015 ± 0.00244 (5%) s * | 0.65840 ± 0.01350 (2%) s *  | 8.24177 ± 0.09567 (1%) s *  | 101.51249 ± 1.61576 (2%) s * |
+----------------------------------------+----------------------------+----------------------------+----------------------------+-----------------------------+-----------------------------+------------------------------+
| algos::shellsort_sedgewick86           | 0.00083 ± 0.00005 (6%)     | 0.00360 ± 0.00010 (3%)     | 0.05477 ± 0.00268 (5%) s   | 0.70330 ± 0.01358 (2%)      | 9.27153 ± 0.12953 (1%)      | 116.72985 ± 2.52380 (2%)     |
+----------------------------------------+----------------------------+----------------------------+----------------------------+-----------------------------+-----------------------------+------------------------------+
| algos::shellsort_gonnet_baeza          | 0.00077 ± 0.00006 (7%)     | 0.00366 ± 0.00009 (2%)     | 0.06059 ± 0.00329 (5%)     | 0.77024 ± 0.01354 (2%)      | 10.01428 ± 0.12831 (1%)     | 125.30122 ± 3.15616 (3%)     |
+----------------------------------------+----------------------------+----------------------------+----------------------------+-----------------------------+-----------------------------+------------------------------+
| algos::shellsort_tokuda                | 0.00224 ± 0.00009 (4%)     | 0.00568 ± 0.00014 (3%)     | 0.06359 ± 0.00351 (6%)     | 0.75365 ± 0.01407 (2%)      | 9.53897 ± 0.10989 (1%)      | 118.84112 ± 2.55185 (2%)     |
+----------------------------------------+----------------------------+----------------------------+----------------------------+-----------------------------+-----------------------------+------------------------------+
| algos::shellsort_ciura                 | 0.00077 ± 0.00005 (6%)     | 0.00358 ± 0.00011 (3%)     | 0.05681 ± 0.00279 (5%)     | 0.71683 ± 0.01398 (2%)      | 9.35106 ± 0.12323 (1%)      | 112.16648 ± 2.40597 (2%)     |
+----------------------------------------+----------------------------+----------------------------+----------------------------+-----------------------------+-----------------------------+------------------------------+
└ Values in ms; 98% confidence interval displayed; s = statistically equal to fastest; * = within 5% of fastest
```
(performance on **Intel 10th gen i7, Wandows, mingw-64 & rust llvm**)

We tested numerous different gap sequences with shellsort (see the [wikipedia page][shell_wiki] for
details).

Straightforward insertionsort substantially out-performs shellsort on small arrays (10 - 100) and
shellsort doesn't begin to shine until moderately sized arrays (somewhere between 100 and 1000). Of
the sequences we tested, Sedgewick's sequence published in 1982
(<img src="https://render.githubusercontent.com/render/math?math=4%5Ek%20%2B%203%20%5Ccdot%202%5E%7Bk%20-%201%7D%20%2B%201">) <!-- 4^k + 3 \cdot 2^{k - 1} + 1 -->
consistently out-performed other sequences by a large margin.

## Future Work

### Other algorithm performance factors

As mentioned in the introduction, at the moment we run algorithms on a series of input sizes. I
would like to later add functionality to systematically alter algorithm parameters. For example,
testing various cutoffs for the value of shellsort's
<img src="https://render.githubusercontent.com/render/math?math=h_{max}"> or finding the best
threshold for a hybrid algorithm to switch to insertionsort.

### Test methodology

We benchmarked using completely random arrays. This is not always the case in a real program:
sometimes an array may be almost sorted. We did not attempt to generate random "almost sorted"
arrays.

### Shellsort

Unless the wikipedia page's table indicated an upper bound for the value of
<img src="https://render.githubusercontent.com/render/math?math=h">, we set
<img src="https://render.githubusercontent.com/render/math?math=h_{max}"> to half the array's
length.

I am not sure the effect of this on performance. On one hand the shell sequences are exponential so
any performance hit by setting
<img src="https://render.githubusercontent.com/render/math?math=h_{max}"> too large should be
relatively insignificant, however, cache performance during an iteration with a large
<img src="https://render.githubusercontent.com/render/math?math=h"> value may be a significant
factor. It's on the backlog to explore this more.

### Our implementations

We've put work into optimizing our implementations, however, they aren't perfect. One of the notable
areas for improvement os in our quicksort implementations, where our fastest quicksort is still
slower than our fastest mergesort. I'm still puzzled how rust's buildin unstable sort is able to
out-perform our implementations by such a significant margin.

### General Backlog
- Investigate generated assembly for the various insertion sorts.
- Convert all rust algorithm implementations to use unsafe access.
- Generate some graphs, not just tables.
- Performance effect of different h_max values.
- Performance of hash table implementations.
- Test on arduinos and other embedded systems.

# Appendix

## Rust Performance

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
+---------------------------------------+----------------------------+----------------------------+----------------------------+-----------------------------+---------+-----------+
|                                       | 10                         | 100                        | 1,000                      | 10,000                      | 100,000 | 1,000,000 |
+---------------------------------------+----------------------------+----------------------------+----------------------------+-----------------------------+---------+-----------+
| algos::insertionsort                  | 0.00066 ± 0.00005 (7%) s * | 0.00268 ± 0.00009 (4%) s * | 0.11508 ± 0.00514 (4%) s * | 8.67249 ± 0.12000 (1%)   *  | -       | -         |
+---------------------------------------+----------------------------+----------------------------+----------------------------+-----------------------------+---------+-----------+
| algos::insertionsort_c                | 0.00080 ± 0.00004 (5%)     | 0.00302 ± 0.00009 (3%)     | 0.11136 ± 0.00574 (5%) s * | 8.28161 ± 0.10286 (1%) s *  | -       | -         |
+---------------------------------------+----------------------------+----------------------------+----------------------------+-----------------------------+---------+-----------+
| algos::insertionsort_boundary_checked | 0.00067 ± 0.00006 (9%) s * | 0.00307 ± 0.00013 (4%)     | 0.16885 ± 0.00659 (4%)     | 14.69654 ± 0.18541 (1%)     | -       | -         |
+---------------------------------------+----------------------------+----------------------------+----------------------------+-----------------------------+---------+-----------+
└ Values in ms; 98% confidence interval displayed; s = statistically equal to fastest; * = within 5% of fastest
```

**Intel 4th gen i7, Ubuntu, gcc & rust llvm**
```
Insertion sorts:
+---------------------------------------+----------------------------+----------------------------+----------------------------+-----------------------------+---------+-----------+
|                                       | 10                         | 100                        | 1,000                      | 10,000                      | 100,000 | 1,000,000 |
+---------------------------------------+----------------------------+----------------------------+----------------------------+-----------------------------+---------+-----------+
| algos::insertionsort                  | 0.00044 ± 0.00003 (6%)     | 0.00308 ± 0.00008 (3%) s * | 0.14240 ± 0.00459 (3%)     | 11.92179 ± 0.26666 (2%)     | -       | -         |
+---------------------------------------+----------------------------+----------------------------+----------------------------+-----------------------------+---------+-----------+
| algos::insertionsort_c                | 0.00059 ± 0.00003 (4%)     | 0.00329 ± 0.00008 (3%)     | 0.11799 ± 0.00348 (3%) s * | 8.81012 ± 0.04275 (0%) s *  | -       | -         |
+---------------------------------------+----------------------------+----------------------------+----------------------------+-----------------------------+---------+-----------+
| algos::insertionsort_boundary_checked | 0.00037 ± 0.00002 (6%) s * | 0.00327 ± 0.00008 (3%)     | 0.17175 ± 0.00533 (3%)     | 13.41049 ± 0.04915 (0%)     | -       | -         |
+---------------------------------------+----------------------------+----------------------------+----------------------------+-----------------------------+---------+-----------+
└ Values in ms; 98% confidence interval displayed; s = statistically equal to fastest; * = within 5% of fastest
```

**ARM11, Raspbian, gcc & rust llvm (raspberry pi zero)**
```
Insertion sorts:
+---------------------------------------+----------------------------+----------------------------+----------------------------+------------------------------+---------+
|                                       | 10                         | 100                        | 1,000                      | 10,000                       | 100,000 |
+---------------------------------------+----------------------------+----------------------------+----------------------------+------------------------------+---------+
| algos::insertionsort                  | 0.00509 ± 0.00022 (4%) s * | 0.02846 ± 0.00054 (2%) s * | 2.04610 ± 0.01179 (1%)     | 249.47356 ± 0.97038 (0%)     | -       |
+---------------------------------------+----------------------------+----------------------------+----------------------------+------------------------------+---------+
| algos::insertionsort_c                | 0.00702 ± 0.00025 (4%)     | 0.02864 ± 0.00060 (2%) s * | 1.67797 ± 0.00801 (0%) s * | 216.95768 ± 0.75235 (0%) s * | -       |
+---------------------------------------+----------------------------+----------------------------+----------------------------+------------------------------+---------+
| algos::insertionsort_boundary_checked | 0.00521 ± 0.00022 (4%) s * | 0.03423 ± 0.00066 (2%)     | 2.56015 ± 0.01676 (1%)     | 300.33886 ± 1.02493 (0%)     | -       |
+---------------------------------------+----------------------------+----------------------------+----------------------------+------------------------------+---------+
└ Values in ms; 98% confidence interval displayed; s = statistically equal to fastest; * = within 5% of fastest
```

**ARMv7, Raspbian, gcc & rust llvm (raspberry pi 3 B+)**
```
Insertion sorts:
+---------------------------------------+-----------------------------+----------------------------+----------------------------+------------------------------+---------+-----------+
|                                       | 10                          | 100                        | 1,000                      | 10,000                       | 100,000 | 1,000,000 |
+---------------------------------------+-----------------------------+----------------------------+----------------------------+------------------------------+---------+-----------+
| algos::insertionsort                  | 0.00132 ± 0.00014 (10%) s * | 0.01625 ± 0.00034 (2%)     | 1.40078 ± 0.02127 (2%)     | 147.26608 ± 0.36139 (0%)     | -       | -         |
+---------------------------------------+-----------------------------+----------------------------+----------------------------+------------------------------+---------+-----------+
| algos::insertionsort_c                | 0.00224 ± 0.00015 (7%)      | 0.01261 ± 0.00028 (2%) s * | 0.80043 ± 0.01146 (1%) s * | 81.94229 ± 0.24065 (0%) s *  | -       | -         |
+---------------------------------------+-----------------------------+----------------------------+----------------------------+------------------------------+---------+-----------+
| algos::insertionsort_boundary_checked | 0.00146 ± 0.00014 (9%) s    | 0.01474 ± 0.00030 (2%)     | 1.22301 ± 0.01775 (1%)     | 126.31177 ± 0.26702 (0%)     | -       | -         |
+---------------------------------------+-----------------------------+----------------------------+----------------------------+------------------------------+---------+-----------+
└ Values in ms; 98% confidence interval displayed; s = statistically equal to fastest; * = within 5% of fastest
```

With small arrays (10 - 100 elements), an insertionsort implementation in native rust using
unchecked array accesses is able to perform very well (on-par or better than C). However, the C
implementation out-performs rust substantially on larger arrays.

Why does rust out-perform on tiny arrays but under-perform on larger arrays? I think it may have to
do with how the function call works with foreign linkage (e.g. lack of inlining or other
optimizations on the call itself).

It's on the backlog to investigate the generated assembly better to understand what's going on here.

["Performance Matters"]: https://www.youtube.com/watch?v=r-TLSBdHe1A
[Coz]: https://github.com/plasma-umass/coz
[Stabilizer]: https://github.com/ccurtsinger/stabilizer
[shell_wiki]: https://en.wikipedia.org/wiki/Shellsort#Gap_sequences

[1]: https://stackoverflow.com/questions/20333547/cache-specifications-for-intel-core-i7
[2]: https://stackoverflow.com/questions/49092541/which-cache-mapping-technique-is-used-in-intel-core-i7-processor
[3]: https://en.wikichip.org/wiki/amd/microarchitectures/zen
[4]: https://en.wikichip.org/wiki/amd/microarchitectures/zen_2
