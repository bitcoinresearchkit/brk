# Bitview Plugin

The small compatibility contract shared by Bitview's built-in and external
plugins.

It provides stable plugin identity, root storage schema, and publication gates
for query-visible mutable data. A plugin declares one `PluginStorage`, which is
the source of truth for its `PluginId`, root schema version, `plugins/<id>`
directory, database opening, and database finalization. The directory may be
empty for an in-memory plugin. Component versions remain local and additive
when they describe a narrower stored or computed dependency.

Plugin import constructors receive a copyable `ImportContext`, which provides
the composition data root to `PluginStorage`. Computing plugins declare their
typed dependencies and output through `ComputePlugin`; its copyable
`UpdateContext` provides shared update control such as cancellation. Plugin
dependencies stay explicit and typed instead of being hidden in either
context.

The contexts are lightweight borrowed handles: the runner creates one of each
and passes them by value through the composition. The
runnable default composition lives in
[`bitview_default`](https://crates.io/crates/bitview_default), while
[`bitview`](https://crates.io/crates/bitview) runs any compatible composition.
Generic composition and update lifecycle traits live in
[`bitview_runtime`](https://crates.io/crates/bitview_runtime).

The plugin API remains experimental while the built-in Bitview modules are
extracted into independent crates.

## Publication-read allocation review

Multi-plugin acquisition inserts owned lock guards directly into its final vector.
Previously each gate allocated a one-element `PluginReadGuard` vector, which was
immediately appended and freed. The crate-private accessor removes that temporary
allocation per acquired gate without changing the public API, pointer ordering,
deduplication, deadline, partial-set release or writer exclusion. No retained
state or dependency was added.

Eight plugin tests pass, including deadline, partial-release and duplicate-gate
coverage. Three isolated collection-loop comparisons measured 44–47 ns before
versus 15–17 ns after for two gates, and 145–151 ns versus 32–33 ns for eight.
The ignored benchmark uses four warmup and 20 alternating batches of 10,000
acquire/drop cycles; it excludes sorting, waiting and HTTP dispatch. These are
small allocation savings, not an end-to-end server performance claim.

## License

MIT
