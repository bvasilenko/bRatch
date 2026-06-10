# bratch

Prompt lookup tool. Agent names a regression signature from a fixed list; bratch returns the prompt for that regression signature. The prompt tells the agent how to check the diff against a baseline for that regression.

Built for agentic loops. Reads a diff buffer, matches against a closed regression-signature taxonomy, writes a verdict directive on stdout, exits with a discriminating code so the calling agent can branch on whether a regression was found.

```
bratch compare       check a diff against a baseline for regression signatures; exit 0 / 1 / 2 / 64
bratch signatures    list the supported regression-signature identifiers
bratch init          scaffold a manifest in the current directory
bratch update        self-update to the latest published version
bratch tail          stream recent verdict transcripts
bratch explain       print taxonomy and exit-code reference
```

Exit code contract: `0` no regression, `1` regression detected, `2` internal error, `64` malformed input.

## Install

```sh
cargo install --git https://github.com/bvasilenko/bRatch
```

## Use

```sh
bratch compare --diff ./PR-feature-x.diff
# stdout: [bratch placeholder directive - pre-corpus output] ...
# exit: 1

bratch compare --diff ./PR-feature-x.diff --history main
# stdout: [bratch placeholder directive - pre-corpus output] ...
# exit: 1
```

Optional flags: `--diff <path>`, `--history <path-or-name>`, `--manifest <path>`, `--json`, `--quiet`, `--reason <text>`. Subcommands consume the same flag set; defaults are sane.

> **WARNING - pre-corpus build**: In this release `bratch compare` exits with code `1`
> regardless of the actual diff content. The directive on stdout is a placeholder.
> Do not wire `bratch compare` into a gating CI step until the corpus-backed release lands.
> Every invocation will appear as a regression finding.

## Regression-signature taxonomy

Closed `RegressionSignature` enum. The taxonomy is fixed at this version; widening lands in a later version.

| Category | Variants |
|---|---|
| Engineering | `null-check-removed`, `error-handling-narrowed`, `test-assertion-weakened`, `scope-boundary-broken`, `invariant-violated` |
| CMS-context | `brand-voice-loosened`, `banned-term-re-introduced`, `disclosure-line-deleted`, `approved-fact-replaced` |

`bratch signatures` prints the full list.

## License

MIT.
