# Rule

before editing a file, always explain why that code, why it's the most optimal one and wait for my feedback

# Types

To check types run:

```sh
npx --package typescript tsc --noEmit --pretty false | grep -v "modules/"
```

# Code

ALWAYS

- very fast
- light (memory)
- KISS
- DRY
- very well organized
- contained
- colocated
- idiomatic
- composed
- prefer one concept per file
- prefer more files and folders than big files
- reads like english
- easy to understand
- maintainability
- avoid defensive checks when the code itself guarantees correctness
- prefer "@type {const}" + infer to "@type {TYPE}" or "@type {import().type}"
