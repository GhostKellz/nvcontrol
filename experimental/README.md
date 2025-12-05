# Experimental Features

This directory contains experimental integrations that are not yet ready for production use.

## Contents

### Source Code (`src/`)
- `bolt_integration.rs` - Bolt container runtime integration (pre-alpha)
- `nvbind_integration.rs` - nvbind GPU passthrough integration (pre-alpha)

### Documentation (`docs/`)
- `BOLT.md` - Bolt integration documentation
- `GHOSTWAVE.md` - ghostwave audio denoising integration
- `NVBIND.md` - nvbind integration documentation

## Status

These features are planned for re-integration in a future release after:
1. Upstream dependencies (bolt, nvbind) reach stable status
2. Full API compatibility is verified
3. Test coverage is complete

## Re-integration

To re-enable these features:
1. Move source files back to `src/`
2. Add module declarations to `lib.rs`
3. Restore CLI commands in `nvctl.rs`
4. Add dependencies to `Cargo.toml`

See the main project for current stable features.
