# Benchmarking IRI crates

To use, run `cargo bench`.

This compares performances of

* [iref](https://crates.io/crates/iref)
* [oxiri](https://crates.io/crates/oxiri)
* [sophia_iri](https://crates.io/crates/sophia_iri)

in parsing absolute and relative IRIs,
and in resolving relative IRIs against a base.
