# Layer fixture & expected-output formats

Every fixture is a pair of sibling files in a layer's `fixtures/` directory:

```
<name>.in          # the input artifact
<name>.expected    # the known-correct expected output
<name>.provenance.yaml   # optional per-fixture provenance (see below)
```

The harness pairs them by stem and judges `<name>.expected` against the runner's
output for `<name>.in` (after the layer's declared normalization). Each layer
declares its `output_format` in its `manifest.yaml`; the formats are:

| Layer            | `.in` format                     | `.expected` (`output_format`) |
|------------------|----------------------------------|-------------------------------|
| source-intake    | raw JS source text (UTF-8)       | `char-stream` — one normalized code point or chunk per line |
| scanner          | JS source text                   | `token-stream` — one `TOKEN  "lexeme"` per line, in scan order |
| parser           | JS source text                   | `ast-sexpr` — parenthesized s-expression of the parse tree |
| ast              | `ast-sexpr` (a parse tree)       | `ast-sexpr` — canonicalized/visited tree |
| bytecode-gen     | JS source text                   | `bytecode-listing` — one `OFFSET  Opcode operands` per line |
| interpreter      | `bytecode-listing`               | `stdout-text` — the program's observable result |
| runtime-support  | function inputs (layer-specific) | `stdout-text` — function output |

## Token-stream format (scanner)

One token per line: the V8 token name, two spaces, then the lexeme in double
quotes. End-of-input is the final `EOS` line. Example:

```
LET  "let"
IDENTIFIER  "x"
ASSIGN  "="
NUMBER  "1"
SEMICOLON  ";"
EOS  ""
```

## AST s-expression format (parser / ast)

A parenthesized prefix tree, one node per `(NodeType ...children)`. Whitespace is
insignificant (the `sort-lines` rule is NOT applied; structure is significant).

```
(FunctionLiteral "f"
  (Block
    (Return (NumberLiteral 1))))
```

## Authoring rules

- Prefer real V8 inputs/outputs (see provenance below); author only when reuse is
  not viable for the layer.
- Keep one behavior per fixture so a failure localizes.
- Declare any incidental nondeterminism via the manifest `oracle.normalize` rules
  rather than editing expected files to match.
