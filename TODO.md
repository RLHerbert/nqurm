# TODO

## urm
- `execute` returns whole register
- `execute` takes reference instead of using `into_iter`
- Implement:
  - `max_register`

## encode
- Hook up to `urm`
- Implement:
  - `length` (directly from `urm`?)
  - `instruction_at_index`/`a(i,x)` (directly from `urm`?)

## Misc.
- Separate tests?
- Create binary/CLI app for simulating URM programs, encoding, etc.

## Maybe
- Implement optional feature `primitive_recursive` which turns all functions into primitive recursive functions.