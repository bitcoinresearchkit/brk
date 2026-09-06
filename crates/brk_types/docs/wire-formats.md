# Shared wire-format invariants

Block hashes use one fixed-size lowercase hex encoder for display, serialization
and vector JSON. Bitcoin's conventional byte order is preserved. Tests compare
native Bitcoin formatting, flags, JSON round trips and appended vector output.

Raw URPD prices are already unique and sorted by their source BTreeMap. Prepare
them directly instead of rebuilding a hash table and sorting again. Linear/log
aggregation retains its own grouping algorithm. Tests compare exact ordering,
supply and realized-cap arithmetic, including empty and maximum finite inputs.
