# Guidelines

## Parser

- Avoid floats as much as possible
  - Use structs like `Amount` and `Price` for calculations
  - **Only** use `Amount.to_btc()` when inserting or computing inside a dataset. It is **very** expensive.
- No `Arc`, `Rc`, `Mutex` even from third party libraries, they're slower
