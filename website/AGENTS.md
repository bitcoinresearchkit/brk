# Rule

before editing a file, always explain why that code, why it's the most optimal one and wait for my feedback

# Types

To check types run:

```sh
npx --package typescript tsc --noEmit --pretty false | grep -v "modules/"
```

# Code

ALWAYS

- fast
- KISS
- DRY
- very well organized
- contained
- colocated
- prefer one concept per file
- prefer more files and folders than big files
- reads like english
- very easy to understand
- very easy to maintain
- avoid defensive checks when the code itself guarantees correctness
