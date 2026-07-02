# bratch

For agents that risk regressing prior work. bratch reads a revision pair (the change about to be made vs the prior baseline) against a regression-signature taxonomy and emits a tri-state verdict: HISTORICAL-BUG (this change matches a pattern that broke things before), CLEAN, or MALFORMED. The signature taxonomy is closed (9 variants at v0.1); the prompt library evolves continuously via empirical-lift evaluation, so the same `bratch verify` invocation gets stricter at catching regression-signatures as the corpus matures.


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
bratch compare --signature null-check-removed --diff ./PR-feature-x.diff
# stdout: REGRESSION-DETECTED: null-check-removed. ...
# exit: 1
```

Optional flags: `--diff <path>`, `--history <path-or-name>`, `--manifest <path>`, `--json`, `--quiet`, `--reason <text>`. Subcommands consume the same flag set; defaults are sane.

## Regression-signature taxonomy

Closed `RegressionSignature` enum. The taxonomy is fixed at this version; widening lands in a later version.

| Category | Variants |
|---|---|
| Engineering | `null-check-removed`, `error-handling-narrowed`, `test-assertion-weakened`, `scope-boundary-broken`, `invariant-violated` |
| CMS-context | `brand-voice-loosened`, `banned-term-re-introduced`, `disclosure-line-deleted`, `approved-fact-replaced` |

`bratch signatures` prints the full list.

The v0 corpus shipped with this release is hand-authored fixture material. An empirically evolved corpus ships in a later cycle; upgrading is handled automatically via `bratch update` once the signed-manifest endpoint is live.

## Configuration

| Environment variable | Purpose | Default |
|---|---|---|
| `BSUITE_UPDATE_BASE_URL` | Base URL for the `bratch update` manifest fetch | `https://updates.example.invalid/bratch/v1` (deliberately non-resolving; production URL ships with the first signed-manifest release) |
| `BSUITE_TRANSCRIPT_DIR` | Override the per-OS default transcript directory | OS default (Linux: `$XDG_STATE_HOME`; macOS: `~/Library/Application Support`; Windows: `%LOCALAPPDATA%`) |
| `BSUITE_TRANSCRIPT_RETENTION_DAYS` | Days of transcript history to retain | `90` |

## License

MIT.
