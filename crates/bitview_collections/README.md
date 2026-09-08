# bitview_collections

Typed field groups shared by Bitview: resolutions, windows, percentiles,
distribution statistics, and investment periods. These shapes contain values;
they do not import vectors or choose cache policy.

The default build has no storage integration. The optional `storage` feature
adds traversal and column identifiers. Cohort membership and cohort-specific
groups live in `bitview_cohort`; query protocol types live in `bitview_types`.
