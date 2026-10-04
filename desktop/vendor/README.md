# Linux desktop glib backport

`glib-0.18.5` is the published crates.io source, with the two-line fix from
[gtk-rs/gtk-rs-core#1343](https://github.com/gtk-rs/gtk-rs-core/pull/1343)
(merge `05dff0ee696f9bcd8617cd48c4b812d046d440cb`) applied to
`src/variant_iter.rs`. The original licenses and version are preserved.
`Cargo.lock` was added solely to lock the upstream regression-test dependencies.

The original `.crate` SHA-256 is
`233daaf6e83ae6a12a52055f568f9d7cf4671dabb78ff9560ab6da230ce00ee5`.
`scripts/check_glib_backport.py` checks that archive digest and compares every
vendored source file against it, permitting only the upstream correction.

This fixes [RUSTSEC-2024-0429](https://rustsec.org/advisories/RUSTSEC-2024-0429.html):
the C output argument must refer to a mutable pointer. The unpatched optimized
upstream iterator tests crashed with SIGSEGV on the review machine. Run the same
tests against the backport with:

```sh
cargo test --release --locked --manifest-path desktop/vendor/glib-0.18.5/Cargo.toml --lib variant_iter::tests
```

This is a desktop dependency, not part of the minimal decoder, CLI, WebAssembly
reviewer or isolated signer. GTK3 currently requires the 0.18 series. Replacing
it with 0.20 is not a compatible lockfile update. Remove this backport when the
desktop stack supports an upstream-fixed release. Version-only advisory tools
may continue to flag 0.18.5; do not rename the package or broadly ignore the
advisory to make those tools appear green. This backport addresses this advisory,
not every possible defect in GTK, glib or the desktop stack.
