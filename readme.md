# spectral

This is a library for **fast, approximate algebraic computations on tensors**. It specifically exploits advanced SIMD instructions and extensions which are architecture-specific. It will fall back to generic SIMD if nothing else, though it will _not_ fall back to 'scalar' code (SISD).

## Motivation

In theory, SIMD should yield speedups of several times the speed, based on the assumption that auto-vectorisation would optimise existing code. This visibly didn't happen.

In practice, the guarantee of preserving existing semantics means that it founders due to minute semantic details that it can't guarantee the author didn't intend (the problem with optimisation in a nutshell).

### Aims

- **Default to performance.** For instance, code is ruthlessly monomorphised, even over 'const generics' like string values. Binary size, memory usage, and all other metrics will be traded off - within reason - for _reduced work_ (i.e. CPU cycles).

- **As few dependencies as possible.** Dependencies multiply exponentially, and a low-level lib should not contain many. We aim to minimise _total transitive dependencies_.

    - Where possible, 'nice-to-have' functionality which depends on other libs will be feature-gated.

'Performance' is defined as using a minimum of clock cycles; memory will be kept within reason, but as a rule we'll always trade off increased memory for reduced CPU work.

### Is this for me?

You should use this iff:

- You are targeting Intel, AMD, or Apple hardware.
- Your bottleneck is pure computation on vectors or matrices.
- You have some understanding of vector processing (e.g. SIMD).

One use case is low-level machine learning code. Computer graphics or game development is another. If this is for you, you are likely aware of SIMD already, and perhaps looking to exploit capabilities beyond regular 'portable' SIMD.

## Usage

### Hardware

Currently, we optimise for the following above all:

- **Mac ARM64 chips**: These support not only the ARM-standard NEON registers and operations[0], but also the AMX coprocessor: a vector processor with a 64x64 u8 matrix register (plus 2 8x64 ones)[1].

- **Intel/AMD x86 chips**: These support 256x1/512x1 f32/64 registers, with broadcasting instructions that can be used to emulate ops like matrix multiplication (but at the expense of chunk-per-cycle serial processing[2]).

We fall back to portable SIMD if required. We **do not** fall back to SISD, since this represents a failure to do our one job, which should be an error condition.

### Coding

Some tips for performance and safety:

- **Measure, measure, measure.** Poor use of these tools [can of course *worsen* performance](https://blog.cloudflare.com/on-the-dangers-of-intels-frequency-scaling/).

- **Locality is paramount.** Any relevant functions will be amply documented to indicate how you can  cache locality.

- All assertions are disabled in release builds. Use `-C debug-assertions` if you want to retain them.

---

[0] See ARM's [NEON Developers Guide](https://developer.arm.com/documentation/102159/0400/Load-and-store---data-structures).

[1] See [this excellent reverse-engineered breakdown](https://gist.github.com/dougallj/7a75a3be1ec69ca550e7c36dc75e0d6f).

[2] This would also be true of the AMX vector processor, 