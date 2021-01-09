# Sorting Benchmarks

![build status](https://github.com/jeremy-rifkin/Sorting-Benchmarks/workflows/build/badge.svg)
![tests status](https://github.com/jeremy-rifkin/Sorting-Benchmarks/workflows/tests/badge.svg)

Asymptotic complexity provides a lot of information about the behavior of an algorithm, however,
algorithms with the same complexity may perform very differently when it comes to physical hardware.
The canonical example is mergesort vs. heapsort (which has poor cache performance).

This project experimentally and rigorously benchmarks the hardware performance of various sorting
algorithms.
