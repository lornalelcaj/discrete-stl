# Discrete STL-GT Syntactic Separation

This project implements the discrete-time STL-GT semantics and the global/local syntactic-separation transformations developed in my master's thesis.

## Mathematical setting

A discrete-time signal is evaluated over

$$
\mathbb{N} = \{0,1,2,\dots\}.
$$

The main Until operator is

$$
\varphi U_{I_d}^{K_d} \psi,
$$

where:

- $I_d$ is the local timing set and constrains the relative delay:

$$
j-k \in I_d
$$

- $K_d$ is the global timing set and constrains the absolute witness position:

$$
j \in K_d
$$

The Until formula holds at position $k$ if there exists a witness $j$ such that

$$
j \ge k,
\qquad
j-k \in I_d,
\qquad
j \in K_d,
$$

$$
\psi \text{ holds at } j,
$$

and

$$
\varphi \text{ holds at every position from } k \text{ through } j.
$$

## Project structure

### `interval.rs`

Implements the discrete temporal-set operations used by the semantics and separation rules.

Main concepts:

- interval membership:

$$
n \in I_d
$$

- intersection:

$$
I_d \cap J_d
$$

- discrete left shift:

$$
I_d \ominus_{\mathbb{N}} \tau
$$

The shift is needed in local separation because, after moving the evaluation point from $k$ to $k+\tau$,

$$
j-(k+\tau) = (j-k)-\tau.
$$

### `formula.rs`

Defines the syntax of the discrete STL-GT formulas.

Supported formulas include

$$
\top,
\qquad
\neg\varphi,
\qquad
\varphi \land \psi,
\qquad
\varphi \lor \psi,
$$

and the temporal operators

$$
F_{I_d}^{K_d}\varphi,
\qquad
G_{I_d}^{K_d}\varphi,
\qquad
\varphi U_{I_d}^{K_d}\psi.
$$

This file only represents formulas. It does not evaluate them.

### `trace.rs`

Stores finite Boolean traces used for semantic validation.

For example,

$$
p=[T,T,F,T],
$$

$$
q=[F,F,T,F].
$$

This is a Boolean abstraction of the underlying discrete signal.

The trace also defines a finite horizon $H$.

### `semantics.rs`

Implements the discrete satisfaction relation

$$
\sigma,k \models_d \varphi.
$$

For Until, the implementation checks

$$
j-k \in I_d,
$$

$$
j \in K_d,
$$

$$
\sigma,j \models_d \psi,
$$

and

$$
\forall m \in [k,j]\cap\mathbb{N}:
\qquad
\sigma,m \models_d \varphi.
$$

### `separation.rs`

Implements the global and local syntactic-separation transformations.

For global separation,

$$
K_d^- =
K_d \cap [0,\tau-1]_{\mathbb{N}},
$$

$$
K_d^0 =
K_d \cap \{\tau\},
$$

$$
K_d^+ =
K_d \cap [\tau+1,\infty)_{\mathbb{N}}.
$$

The witness is therefore classified as

$$
j<\tau,
\qquad
j=\tau,
\qquad
j>\tau.
$$

For local separation, the local interval is split according to

$$
j-k<\tau,
\qquad
j-k=\tau,
\qquad
j-k>\tau.
$$

The after-$\tau$ branch uses the shifted interval

$$
I_d^+ \ominus_{\mathbb{N}} \tau.
$$

At the moment, $\tau$ is provided explicitly to the separation functions. Automatic derivation of relevant separation points is a future implementation step.

## Validation

The implementation contains unit tests for the discrete semantics and for the separation transformations.

In addition, exhaustive bounded tests enumerate:

- Boolean traces for $p$ and $q$,
- evaluation positions $k$,
- local intervals $I_d$,
- global intervals $K_d$,
- separation points $\tau$,

and compare the result of the original Until formula with the corresponding separated formula.

The current implementation has no mismatches in the tested bounded cases.

These tests are computational validation and do not replace the mathematical proofs.

## Current status

Implemented:

- discrete interval representation
- Boolean trace representation
- discrete STL-GT formula representation
- direct semantics
- Eventually / Globally / Until
- global separation
- local separation
- exhaustive bounded validation

Next steps:

- Restricted Until
- edge-case cleanup
- automatic separation-point derivation
- SMT encoding
- benchmarking