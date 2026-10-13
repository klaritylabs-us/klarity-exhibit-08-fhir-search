# FHIR R4 REST search — Rust

Small library and CLI that sanity-check **FHIR search request lines** (`Resource?param=value&...`) before they hit your server or test harness: resource path, non-empty query, well-formed pairs, **`:missing`** modifiers, and **date prefixes**.

Synthetic fixtures only. Not a FHIR server, not ONC certification or Inferno, and not legal/clinical advice.

Maintained by [Klarity Labs](https://klaritylabs.ai/) as open reference code for U.S. healthcare FHIR interoperability work.

**Exhibit 08** — [FHIR exhibit catalog](https://github.com/klaritylabs-us/klarity-fhir-exhibits) · [klaritylabs-us](https://github.com/klaritylabs-us)


## How this relates to other tools

| Tool class | Role |
|------------|------|
| **HL7 FHIR Validator**, **HAPI**, **Firely** | Full validation, servers, SDKs, IG packages |
| **Inferno**, **Touchstone**, vendor cert suites | **Conformance** against test plans |
| **This exhibit** | **Pre-flight construction** on synthetic fixtures—catch shape mistakes before wire traffic |

Conformance tools test whether your **API behaves**; exhibits focus on whether your **payloads and pipes are built** to survive real partners.

## Specifications cited

- [FHIR R4 RESTful API](http://hl7.org/fhir/R4/http.html)
- [Search](http://hl7.org/fhir/R4/search.html)

## What it checks

- Line format `Resource?...` (optional leading `/`)
- At least one `name=value` pair; no empty names or values
- `:missing` modifier values are `true` or `false`
- Warnings on `birthdate` / `date` without `eq|gt|lt|ge|le|...` prefix
- Focus examples on **Patient** and **Observation** search (warnings for other resource types)

| Problem | What breaks |
|--------|-------------|
| **Malformed pairs** | `&birthdate` with no `=` — 400s or silent ignore depending on server |
| **Wrong `:missing`** | Server rejects or mis-interprets token search |
| **Date without prefix** | Assumes `eq` when you meant `ge` for cohort queries |
| **Empty query string** | Client bugs after URL encoding |

## Run it

Needs [Rust](https://www.rust-lang.org/tools/install).

```bash
cargo test
cargo run --bin validate-search -- examples/valid-patient-search.txt
cargo run --bin validate-search -- examples/invalid-no-query.txt
```

Exit `0` if no errors, `1` if validation failed, `2` for bad usage or input.

## Sample output

One command:

```bash
cargo run --bin validate-search -- examples/valid-patient-search.txt
```

**Pass** (`examples/valid-patient-search.txt`) — exit code `0`:

```json
[]
```

**Fail** (`examples/invalid-no-query.txt`) — exit code `1` (issues array; more entries may appear):

```json
[
  {
    "severity": "error",
    "path": "request",
    "detail": "expected Resource?param=value format",
    "rule_id": "missing-query"
  }
]
```


## Layout

- `src/validate.rs` — rules
- `examples/` — valid and broken search lines for tests and demos

## Inferno vs this repo

Inferno exercises **search behavior** on a live endpoint. This repo is **client-side query construction** — the strings your SDK generates before the request leaves middleware.

## License

MIT — see [LICENSE](./LICENSE).

**Need this on your payloads or API?** 20-minute FHIR Scan — sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)

## Contact

sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)
