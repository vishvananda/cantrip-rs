# Source producer integration regressions

This separate workspace preserves the backend regression tests extracted from
MTGish. The test adapter invokes the pinned producer's export boundary before
calling the standalone generator. Source-only suites remain in MTGish; mixed
historical regressions are retained here as cross-component checks.

The normal generator has no Source dependency. Only this integration workspace
fetches the public Git-pinned lowerer. It never enables its former backend.
Producer crates and this generator retain an unoptimized development profile;
optimizing the large Source Serde implementations makes cold test builds costly
without improving the production generator, which never includes them.

Check out the revision in Cargo.toml as a fixture checkout, then run:

```sh
export MTGISH_SOURCE_ROOT=/path/to/pinned/mtgish
RUST_MIN_STACK=67108864 cargo test --locked --manifest-path tools/oracle-codegen/source-tests/Cargo.toml
```

The checkout must match the dependency revision; tests read its frozen corpus and
producer inputs, while local test fixtures travel with the tests. The build
script requires the explicit fixture path. Update the Git pin, fixture checkout
and golden evidence together when adopting a producer change. Ignored native
compile/runtime cases still require explicit `MTGISH_CANTRIP_DIR` and a separate
native target; they do not silently use an arbitrary neighboring clone.
