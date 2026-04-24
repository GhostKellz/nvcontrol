# 0.8.7 Release Checklist

Before shipping `v0.8.7`, verify:

```bash
cargo audit
cargo build
cargo test

nvctl driver diagnose-release
nvctl driver check
nvctl driver support-bundle --tarball --redact-paths --redact-ids --log-tail 80 --output ~/.local/state/nvcontrol/support/support.tar.gz
nvctl doctor --support --output ~/.local/state/nvcontrol/support/doctor-support.tar.gz
nvctl companion notify-test
```

## Documentation Checks

- README support workflow is current
- driver command docs reflect the latest flags
- issue-reporting docs reflect the latest support bundle workflow
- release diagnostics interpretation doc is linked from the docs index

## Expected Artifacts

- support tarball created successfully
- support metadata JSON created or packaged successfully
- no failing tests
- no outstanding cargo audit advisories
