# xensieve

An implementation of the Xenakis Sieve, providing a Sieve interface from a string expression that filters integer sequences into sieve integers, Boolean states, or interval widths. Sieves are built from Residuals, defined as a modulus (M) and a shift (S), notated `M@S`. Sieve string expressions, and Sieve structs, support complementation, intersection, symmetric difference, and union operations on Residuals with operators `!`, `&`, `^` and `|`, respectively.

The Xenakis Sieve is tool for generating discrete interval patterns. Such patterns have boundless applications in creative domains: the Xenakis Sieve can be used to generate scales or multi-octave pitch sequences, rhythms and polyrhtyhms, and used to control countless other aspects of pictorial or architectural design.

This Rust implementation follows the Python implementation in Ariza (2005), with significant performance and interface enhancements: https://direct.mit.edu/comj/article/29/2/40/93957

# Strategies for Creating Sieves

First, we can examine the output of Sieves built from a single Residual. As shown above, a Residual is defined as a modulus (M) and a shift (S), notated `M@S`. In the diagram below, three Residuals are shown: `5@0`, `4@2`, and `30@10`. As can be seen, for every M units, a value is articulated at the shift S. The final example shows an application of the unary inversion operator `!30@10`.

![Residual diagram](https://raw.githubusercontent.com/flexatone/xensieve-sandbox/default/images/residual-a.svg)

Complex Sieves combine Residuals with logical operators such as complementation, intersection, symmetric difference, and union. In the example below, Residuals `5@0` and `4@2` are combined by union with the expression `5@0|4@2`. Combining many Residuals by union is a practical approach to building sequences. The final example, `(5@0|4@2)&!30@10`, shows "removing" selected values from these unioned components by intersecting them with an inverted Residual (`!30@10`)

![Sieve diagram](https://github.com/flexatone/xensieve-sandbox/blob/default/images/sieve-a.svg)

While all Sieves are, by definition, periodic, combinations of Residuals can result in sequences and with great local complexity and inner patterning.


# What is New in `xensieve`

## 0.3.0

Code cleanup and improved doc strings.

## 0.2.0

Implemented symmetric difference with the `^` operator on `Sieve`.

## 0.1.0

First release.
