# Benchmarking IRI crates

To use, run `cargo bench`.

This compares performances of

* [iri-string]
* [iref]
* [oxiri]
* [sophia_iri]

[iref]: https://crates.io/crates/iref
[iri-string]: https://crates.io/crates/iri-string
[oxiri]: https://crates.io/crates/oxiri
[sophia_iri]: https://crates.io/crates/sophia_iri

## Analysis

The detailed results are available [here](https://pchampin.github.io/bench_iri/results/report/).

For **parsing (absolute) IRIs**, [sophia_iri] is by far the fastest, and [oxiri] is the slowest.
[oxiri] being slow is probably due to the fact that it does not only check that the underlying text complies with RFC 3987,
it also populates a data structure representing the different parts of the IRI.
As we will see, this pays off for IRI resolution,
but is needlessly costly for the simple cases where we only want to check that an IRI is valid
(arguably a much more common use-case in RDF).
[sophia_iri] uses a big [regular expression](https://github.com/pchampin/sophia_rs/blob/af11895647feaa980108d2584c04d77c46764069/iri/src/_regex.rs#L47)
and benefits from the very good performances of the [regex](https://crates.io/regex) crate.
Granted, such a huge regular expression is hard to maintain, but the specification of IRIs is not expected to change often.
So the maintenance cost is acceptable compared to the gain in performance.

For **parsing arbitrary IRI references** (which can therefore be absolute or relative),
we see a big difference when the input is an absolute IRI reference or a relative one.
In the former case, the performances are very similar to the case where an (absolute) IRI is expected (described above).
However, in the latter case, [sophia_iri] drops from being the fastest to being the slowest.
The reason seems to be that [sophia_iri] first tries to match a regular expression for absolute IRI references,
and if it fails, tries to match another regular expression for relative IRI references.
*Another experiment (not in the report) shows that using a `RegexSet` instead brings back [sophia_iri] on par with the other crates.*

For **resolving relative IRI references** against a fixed base, there are two dimensions along which the different crates differ:
* [iri-string], [oxiri] and [sophia_iri] provide a dedicated type for the base IRI, which pre-parses the base IRI once and for all
  (actually, [oxiri] uses only one type for all IRIs, but as we saw above, this type always stores the internal structure of the IRI).
  On the contrary, [iref] uses a standard `Iri` as the base, and must rediscover its internal structure at each resolution.
  As expected, [iref] is much slower than the others when resolving multiple relative IRI references against the same base.
* [iri-string] and [iref] forcibly allocate a new buffer for the resolved IRI,
  while [oxiri] and [sophia_iri] can write the result of the resolution in a pre-allocated buffer.
  This allows users to reuse the same buffer when resolving multiple relative IRI references, which also saves time.
  For a fair comparison, [oxiri] and [sophia_iri] have been tested once with a new buffer on each resolution,
  and once with a mutualized buffer (suffix `Mbuf`).
  The second set of tests shows significantly better performances.
*NB: the performances of [oxiri] and [sophia_iri] are the same, for the simple reason that [sophia_iri] is using [oxiri] under the hood for base IRIs.*


## Lessons learned

In RDF, the IRIs present in a graph are unlikely to serve as bases to resolve relative IRI references,
so "common-use" should only wrap the underlying text with the guarantee that it is a valid IRI (or IRI reference).

For such "lightweight" IRIs, the [regex](https://crates.io/regex) crates seems to provide very good performances for parsing.

For those IRIs that are destined to serve as base (typically, the IRI of a document from which triples or quads are parsed),
a dedicated type that stores the internal structure of the IRI should be provided
(all the more that, in these use-cases, the same base is expected to be used multiple times).

Resolving an IRI reference into a pre-allocated buffer should also be supported, to ensure good performances.
