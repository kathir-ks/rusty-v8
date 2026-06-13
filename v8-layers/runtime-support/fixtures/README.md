# runtime-support fixtures

Golden fixtures are `<name>.in` (input) paired with `<name>.expected` (expected
output, `stdout-text` format). None yet: this layer is currently validated by the
transpile + build + no-`todo!()` cycle (built-only), pending a bigint driver crate
that can execute transpiled functions against numeric inputs.
