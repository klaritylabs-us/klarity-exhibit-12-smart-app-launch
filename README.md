# SMART app launch configuration — Rust

Small library and CLI that sanity-check **SMART confidential app launch** JSON before you register or deploy an app: `client_id`, `redirect_uri`, `scope`, and OIDC **`authorization_endpoint`** / **`token_endpoint`** — the fields EHR launch flows expect in `.well-known` and local config.

Synthetic fixtures only. Not ONC certification, not a substitute for SMART App Launch conformance testing or Inferno, and not legal/clinical advice.

Maintained by [Klarity Labs](https://klaritylabs.ai/) as open reference code for U.S. healthcare FHIR interoperability work.

**Exhibit 12** — [FHIR exhibit catalog](https://github.com/klaritylabs-us/klarity-fhir-exhibits) · [klaritylabs-us](https://github.com/klaritylabs-us)


## How this relates to other tools

| Tool class | Role |
|------------|------|
| **HL7 FHIR Validator**, **HAPI**, **Firely** | Full validation, servers, SDKs, IG packages |
| **Inferno**, **Touchstone**, vendor cert suites | **Conformance** against test plans |
| **This exhibit** | **Pre-flight construction** on synthetic fixtures—catch shape mistakes before wire traffic |

Conformance tools test whether your **API behaves**; exhibits focus on whether your **payloads and pipes are built** to survive real partners.

## Specifications cited

- [SMART App Launch](http://hl7.org/fhir/smart-app-launch/)

## What it checks

- Required string fields present and non-empty:
  - **`client_id`**
  - **`redirect_uri`**
  - **`scope`**
  - **`authorization_endpoint`**
  - **`token_endpoint`**

| Problem | What breaks |
|--------|-------------|
| **Missing redirect URI** | OAuth callback fails after user consent |
| **Empty scope** | Token missing patient/*.read or other required access |
| **Wrong token endpoint** | Code exchange succeeds in dev, fails in prod |
| **Stale auth URL** | Launch iframe never completes authorization |

## Run it

Needs [Rust](https://www.rust-lang.org/tools/install).

```bash
cargo test
cargo run --bin validate-smart-launch -- examples/valid-launch.json
cargo run --bin validate-smart-launch -- examples/invalid-launch.json
```

Exit `0` if no errors, `1` if validation failed, `2` for bad usage or JSON.

## Sample output

One command:

```bash
cargo run --bin validate-smart-launch -- examples/valid-launch.json
```

**Pass** (`examples/valid-launch.json`) — exit code `0`:

```json
[]
```

**Fail** (`examples/invalid-launch.json`) — exit code `1` (issues array; more entries may appear):

```json
[
  {
    "severity": "error",
    "path": "redirect_uri",
    "detail": "SMART confidential launch config field required",
    "rule_id": "launch-field"
  }
]
```


## Layout

- `src/validate.rs` — rules
- `examples/` — valid and broken launch config for tests and demos

## Inferno vs this repo

Inferno exercises **SMART launch and token exchange** against real servers. This repo is **static launch config** — the JSON your portal or deployment pipeline ships before users click “Connect”.

## License

MIT — see [LICENSE](./LICENSE).

**Need this on your payloads or API?** 20-minute FHIR Scan — sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)

## Contact

sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)
