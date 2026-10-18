# mpp-rs: continue the reviewed ChargeClient contribution

This is a portable project handoff, not an instruction to publish a PR. Read the
repository's `AGENTS.md` first. The evidence files record completed work on
18 October 2026; their old local paths and uncommitted-state descriptions are
historical. The code is now committed and pushed as specified below.

## Current Git state

- Fork: `https://github.com/AkshatGada/mpp-rs.git`
- Upstream: `https://github.com/tempoxyz/mpp-rs.git`
- Reviewed contribution branch: `codex/owned-charge-client`
- Reviewed code commit: `44018e58bd2706277fc231b4dd51ae868e049af2`
- Base: `473b29962bd1393076c146a411e5a6ea9e3b6ac7`
- Handoff branch: `codex/owned-charge-client-handoff`, based on that code commit.
  Its additional files are migration/review evidence for the fork, not part of
  the intended upstream contribution. Prepare any upstream PR from the code
  branch so the handoff directory is excluded.
- No upstream PR has been opened. No CLA has been accepted. Final publication
  requires the user's authorization. Do not infer it from this migration.

The user authorized committing/pushing the reviewed patch and then pushing the
context necessary to continue on another machine. Preserve the reviewed work;
do not restart implementation or broaden it without a concrete reason. Keep
live payments, production edits, outreach, and unrelated projects out of scope.

## Recreate the working environment

1. Clone this handoff branch. Copy `handoff/owned-charge-client` somewhere outside
   the checkout before switching branches, so its evidence remains available:

   ```sh
   git clone --branch codex/owned-charge-client-handoff https://github.com/AkshatGada/mpp-rs.git
   cd mpp-rs
   cp -R handoff/owned-charge-client ../mpp-rs-handoff
   git switch --track origin/codex/owned-charge-client
   git rev-parse HEAD
   ```

   Confirm HEAD is `44018e58bd2706277fc231b4dd51ae868e049af2` before making changes.

2. Install Git, a native C compiler/linker, and the platform TLS build
   prerequisites. Use Rust **1.98.0** to match the broad validation. With rustup:

   ```sh
   rustup toolchain install 1.98.0 --profile minimal --component clippy,rustfmt
   rustup override set 1.98.0
   cp ../mpp-rs-handoff/Cargo.lock.reviewed Cargo.lock
   cargo fetch --locked
   ```

   The root intentionally ignores `Cargo.lock`; this copy preserves the exact
   CI-resolved graph used for the completed checks. Tempo 1.11.0 dependencies
   declare Rust 1.95.0; earlier Rust 1.94.1 minimal-feature tests did not establish
   Tempo compatibility. Broad checks passed with 1.98; exact 1.95 was not run.
   Adapt shell/native prerequisites to the destination OS without changing the
   implementation merely to match the old machine's filesystem paths.

3. Add upstream, refresh public state, and begin with a focused mock test run:

   ```sh
   git remote add upstream https://github.com/tempoxyz/mpp-rs.git
   git fetch upstream
   cargo test --locked --no-default-features --features client,middleware,utils --lib client:: -- --test-threads=2
   ```

   There are 11 ChargeClient regression tests. Historical aggregate test counts
   below were measured on macOS and may differ with platform-specific tests.
   No real wallet, Stripe key, API credential, or payment is required for these
   mock tests. Set up destination Git identity/GitHub authentication separately
   if later needed; no credentials are included in this handoff.

## Why this contribution exists

Endpoints already using API/Bearer Authorization can advertise
`header="Payment-Authorization"`. Legacy Fetch and arbitrary reqwest middleware
deliberately reject that custom header because the SDK cannot prove their
caller-owned clients' redirect policy. This is an intentional guard, not a new
vulnerability allegation. Removing it would defeat the original fix.

The bounded contribution is an SDK-owned **charge-only** client. Its private
reqwest client is constructed from `ClientBuilder` with `Policy::none()` applied
last. Both initial and paid requests actually execute through that retained
client. There is no conversion/assertion constructor from a built arbitrary
Client. A foreign RequestBuilder's client is discarded.

Payments require valid request-level `Accept-Payment`; client defaults do not
authorize payment. Missing, malformed, repeated, and all-zero preferences fail
closed. Method `q=0` preferences, body cloning, retry caps, challenge ranking,
provider selection and fallback remain effective. ChargeClient filters sessions
before preparation and rechecks selected/prepared offers for transport limits,
provider support, preferences and expiry. It returns redirects without following
Location, including same-origin redirects. Ordinary API Authorization survives
custom-header payment. No session-management client is supplied.

Legacy Fetch, middleware and direct session-management custom-header guards
remain. Session top-up, voucher and close support are excluded. No dependency
was added, and payment provider/session implementations were not edited.

## Architecture and exact contribution scope

- `src/protocol`: challenges/credentials, method/intent types and Accept-Payment
  parsing/ranking. `src/client/provider.rs`: provider hooks and pending lifecycle.
- `src/client/flow.rs`: canonical HTTP payment flow used by Fetch/middleware;
  now has internal transport admission checks and prepared-offer rechecks.
- `src/client/charge_client.rs`: new private-client API; exported by
  `src/client/mod.rs` under `client`.
- `src/client/charge_client/tests.rs`: loopback regressions.
- `src/client/tempo` / `stripe`: existing provider implementations; untouched.
- `src/server`, WebSocket/MCP modules and `crates/alloy-transport-mpp`: related
  integrations and compatibility surfaces, not new feature scope.

The reviewed commit changes exactly these six handwritten files:

```text
.changelog/redirect-safe-charge-client.md
README.md
src/client/charge_client.rs
src/client/charge_client/tests.rs
src/client/flow.rs
src/client/mod.rs
```

Contributions retain MIT OR Apache-2.0 terms. CODEOWNERS at the inspected base:
`@mattsse`, `@onbjerg`, `@brendanjryan`. Maintainer acceptance is not implied.

## Independent review and changes made

A fresh reviewer `/root/independent_review` read the actual diff, source, tests,
protocol binding, selector, provider lifecycle, legacy/session guards and
reqwest execution code. It performed read-only source review, not test runs.

It found one P2 defect: case-sensitive admission rejected protocol-valid
lowercase/mixed-case Payment-Authorization spelling. The fix uses
`eq_ignore_ascii_case` without rewriting the challenge's bound spelling/HMAC.
New tests cover initial and prepared casing while asserting echo preservation.
The two suggested coverage cases were also added: ordinary Authorization
success and provider-requested refresh with an unfollowed redirect.

The reviewer reread the final code/tests and reported no outstanding actionable
findings. This is independent agent review, not maintainer review.
See `evidence/mpp-rs-independent-review.txt`.

## Completed compatibility gates

The repository CI dependency-resolution script was run before validation. The
exact resulting lockfile is `Cargo.lock.reviewed`. All subsequent gates used
`--offline` after dependencies had been downloaded. Results on Rust 1.98:

- Strict `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  passed with no dead-code allowance.
- CI-style `cargo hack check --feature-powerset --depth 2 --no-dev-deps --skip
  integration,integration-stripe,integration-ws`: all **127** configurations
  passed. This is a depth-two matrix, not the full mathematical powerset.
- Workspace tests/doctests: **1,612 passed**, zero failed, 48 ignored, two live
  Stripe tests explicitly filtered. Includes **1,461** main-library tests,
  **64** main-crate doctests, transport tests and mock integrations.
- Final code combined with frozen #574: **1,467** library tests passed.
- Strict all-feature main-crate docs, normal workspace example/transport check,
  Ring-only transport graph, formatting and diff checks: passed.

All 11 new test groups passed: 20 same/cross-origin initial/paid redirect cases
across 301/302/303/307/308; preserved Bearer auth/body; ordinary Authorization;
valid header casing; unsupported/session filtering/fallback; prepared-offer
constraints; preference opt-outs/defaults; paid retry caps; provider refresh;
and noncloneable-body handling. Redirect targets were never contacted.

For commands, counts and limitations, read
`evidence/mpp-rs-compatibility-review-2026-10-18.txt`; raw logs are alongside it.
The `.txt` reports contain original machine paths as historical provenance;
use local paths to these files instead of trying to recreate those directories.

To rerun the broad runtime gate, explicitly exclude real Stripe tests and leave
the on-chain `integration` feature disabled:

```sh
env -u STRIPE_SECRET_KEY -u TEMPO_RPC_URL cargo test --locked --workspace --features tempo,stripe,ws,server,client,axum,middleware,tower,utils,sqlite,integration-stripe,integration-ws -- --skip test_live_stripe --test-threads=2
```

Cargo-hack 0.6.45 was used. Its `--no-dev-deps` mode temporarily modifies the
manifest: run that matrix on an isolated source copy, as was done previously.
No real payments or production edits are authorized by reproducing these checks.

## Upstream overlap and #574 reproduction

Refresh these facts before new publication; they are an 18 October snapshot:

- Upstream main matched base `473b29962bd1393076c146a411e5a6ea9e3b6ac7`.
- [#560](https://github.com/tempoxyz/mpp-rs/pull/560) was merged on 7 October and
  deliberately introduced the custom-header guard. Preserve it.
- [#573](https://github.com/tempoxyz/mpp-rs/pull/573) was merged and carries
  caller-configured policy into session management; it does not prove redirect
  safety or duplicate this client.
- [#574](https://github.com/tempoxyz/mpp-rs/pull/574) was open at
  `46a566bba7b3f2df910d100808ed227618a9f602`. It changes explicit payment policy,
  atomic channel updates and reconciliation. Coordinate integration with it;
  do not duplicate its implementation or infer it is merged.
- [#305](https://github.com/tempoxyz/mpp-rs/pull/305) was protocol-only native EVM
  foundation work, not an owned client. No competing public implementation was
  found in the inspected open work; private plans are unknown.

`upstream/pr-574-at-46a566b.diff` freezes the exact reviewed PR source diff.
To reproduce the combined-source test, use an isolated checkout at base
`473b299`, apply that frozen diff, then apply
`git diff --binary 473b299..44018e5` from this repository. Both patches previously
applied cleanly in that order. Copy `Cargo.lock.reviewed` and use:

```sh
env -u STRIPE_SECRET_KEY -u TEMPO_RPC_URL cargo test --locked --lib --features tempo,stripe,ws,server,client,axum,middleware,tower,utils,sqlite -- --test-threads=2
```

## Remaining work and boundaries

1. Confirm the destination clone and source hashes, restore toolchain/dependency
   graph, and run the focused tests before changing code.
2. Refresh upstream/#574 and inspect any new overlap or conflicts. Preserve
   unrelated user changes and keep subsequent changes bounded and reviewed.
3. The local PR title/body draft is `evidence/mpp-rs-pr-draft.txt`. It clearly
   separates the PR body from local preparation metadata; do not copy that
   metadata into an upstream PR description. Update validation if source changes.
4. Ask for publication authorization before opening an upstream PR. None exists
   yet. Do not accept a CLA or contact maintainers without authorization.

Exact Rust 1.95, other operating systems/targets, live settlement, physical
Node/Rust SQLite handoff, fuzz campaigns, external dependency scanners and
GitHub Actions were not validated. Local tests and independent source review
are not evidence of maintainer approval, merge or production success.

All necessary source/context is on the fork once this handoff commit is pushed.
Build targets, Cargo caches and temporary compatibility checkouts are
reproducible and are not needed from the original machine.
