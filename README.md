```sh
fuelup show
```

results in

```sh
active toolchain
-----------------
latest-x86_64-unknown-linux-gnu (default)
  forc : 0.35.1
    - forc-client
      - forc-deploy : 0.35.1
      - forc-run : 0.35.1
    - forc-doc : 0.35.1
    - forc-explore : 0.28.1
    - forc-fmt : 0.35.1
    - forc-index : 0.2.3
    - forc-lsp : 0.35.1
    - forc-tx : Error getting version string
    - forc-wallet : 0.1.3
  fuel-core : 0.17.2
  fuel-indexer : 0.2.3
```

Result when running the test.

r1 and r2 are the same. Shouldn't `produce_blocks`, given the time, shift the timestamp?

```sh
running 1 test
F
failures:

---- can_get_contract_id stdout ----
r1: ["4611686020104040332"]
r2: ["4611686020104040332"]
thread 'can_get_contract_id' panicked at 'assertion failed: false', tests/harness.rs:61:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    can_get_contract_id

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.24s

error: test failed, to rerun pass `--test tests`
```
