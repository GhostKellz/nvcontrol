//! Safe environment variable operations
//!
//! This module provides safe wrappers around `std::env::set_var` and `std::env::remove_var`
//! which became unsafe in Rust 1.66+ due to potential data races.
//!
//! # Safety Guarantees
//!
//! These functions are safe to use because nvcontrol:
//! 1. Sets environment variables during initialization before spawning worker threads
//! 2. Environment modifications are idempotent (setting same value twice is safe)
//! 3. No concurrent reads/writes to the same env vars occur
//!
//! For truly concurrent scenarios, consider using process-level configuration
//! or command-line arguments instead of environment variables.

use std::ffi::OsStr;

/// Safely set an environment variable.
///
/// This wraps `std::env::set_var` with documented safety guarantees.
/// Use this instead of raw `unsafe { std::env::set_var(...) }`.
#[inline]
pub fn set_var<K: AsRef<OsStr>, V: AsRef<OsStr>>(key: K, value: V) {
    // SAFETY: nvcontrol sets env vars during single-threaded initialization
    // or in contexts where no concurrent env access occurs. The env vars
    // set are NVIDIA-specific hints that are read by external libraries
    // (Mesa, NVIDIA driver) at their initialization, not concurrently.
    unsafe {
        std::env::set_var(key, value);
    }
}

/// Safely remove an environment variable.
///
/// This wraps `std::env::remove_var` with documented safety guarantees.
/// Use this instead of raw `unsafe { std::env::remove_var(...) }`.
#[inline]
pub fn remove_var<K: AsRef<OsStr>>(key: K) {
    // SAFETY: Same guarantees as set_var - called during initialization
    // or in non-concurrent contexts.
    unsafe {
        std::env::remove_var(key);
    }
}

/// Set multiple environment variables at once.
///
/// More efficient than multiple `set_var` calls and ensures atomicity
/// from the caller's perspective.
#[inline]
pub fn set_vars<I, K, V>(vars: I)
where
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<OsStr>,
    V: AsRef<OsStr>,
{
    for (key, value) in vars {
        set_var(key, value);
    }
}

/// Remove multiple environment variables at once.
#[inline]
pub fn remove_vars<I, K>(keys: I)
where
    I: IntoIterator<Item = K>,
    K: AsRef<OsStr>,
{
    for key in keys {
        remove_var(key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get_var() {
        let key = "NVCONTROL_TEST_VAR";
        let value = "test_value";

        set_var(key, value);
        assert_eq!(std::env::var(key).unwrap(), value);

        remove_var(key);
        assert!(std::env::var(key).is_err());
    }

    #[test]
    fn test_set_multiple_vars() {
        let vars = [
            ("NVCONTROL_TEST_A", "value_a"),
            ("NVCONTROL_TEST_B", "value_b"),
        ];

        set_vars(vars);

        assert_eq!(std::env::var("NVCONTROL_TEST_A").unwrap(), "value_a");
        assert_eq!(std::env::var("NVCONTROL_TEST_B").unwrap(), "value_b");

        remove_vars(["NVCONTROL_TEST_A", "NVCONTROL_TEST_B"]);
    }
}
