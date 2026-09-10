# Experimental Features

This directory contains experimental integrations that are not yet ready for production use.

## Contents

### Source Code (`src/`)
- `ghostwave.rs` - ghostwave audio denoising integration (pre-alpha)

### Documentation (`docs/`)
- `GHOSTWAVE.md` - ghostwave audio denoising integration

## Status

These features are planned for re-integration in a future release after:
1. The integration has a clear product fit for nvcontrol
2. Full API compatibility is verified
3. Test coverage is complete

## Re-integration

To re-enable these features:
1. Move source files back to `src/`
2. Add module declarations to `lib.rs`
3. Restore CLI commands in `nvctl.rs`
4. Add dependencies to `Cargo.toml`

See the main project for current stable features.
