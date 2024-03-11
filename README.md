# xensieve

An implementation of the Xenakis Sieve, providing a Sieve interface from a string expression that filters integer sequences into sieve integers, Boolean states, or interval widths. Sieves are built from Residuals, defined as a modulus (M) and a shift (S), notated `M@S`. Sieve string expressions, and Sieve structs, support complementation, intersection, symmetric difference, and union operations on Residuals with operators `!`, `&`, `^` and `|`, respectively.

The Xenakis Sieve is tool for generating discrete interval patterns. Such patterns have boundless applications in creative domains: the Xenakis Sieve can be used to generate scales or multi-octave pitch sequences, rhythms and polyrhtyhms, and used to control countless other aspects of pictorial or architectural design.

This Rust implementation follows the Python implementation in Ariza (2005), with significant performance and interface enhancements: https://direct.mit.edu/comj/article/29/2/40/93957



# What is New in `xensieve`

## 0.3.0

Code cleanup and improved doc strings.

## 0.2.0

Implemented symmetric difference with the `^` operator on `Sieve`.

## 0.1.0

First release.
