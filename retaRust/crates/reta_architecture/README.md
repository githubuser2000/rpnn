# reta_architecture

Typed Rust transcompilation front for the modular `py reta arch` architecture.

This crate is intentionally shared by `rreta` and the split `rretaPrompt` crates.
It starts with the structural layers that should not change visible output:

- execution network / dataflow: FIFO, LIFO, priority, semaphores, half/full duplex channels
- topology: symbolic open Reta contexts and refinements
- category theory: categories, morphisms, functors, natural transformations
- presheaves/sheaves: local sections and deterministic gluing
- universal constructions: deterministic merge and bucket normalization

The crate is not a renderer replacement yet.  It gives Rust the same typed
architecture spine that `py reta arch` already has, so later table/prompt modules
can be ported without copying the old Python monolith shape.
