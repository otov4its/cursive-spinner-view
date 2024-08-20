# Release History

## Unreleased

* Nothing

## 0.1.5 (2024-08-20)

### Added:

* Blueprint for SpinnerView

### Updated:

* cursive-core to 0.4
* cursive to 0.21
* ntest to 0.9

## 0.1.4 (2023-09-17)

### Changed:

* Cargo.lock in .gitignore (see https://doc.rust-lang.org/cargo/faq.html#why-do-binaries-have-cargolock-in-version-control-but-not-libraries)

## 0.1.3 (2023-09-14)

### Changed:

* Fixed compilation issue due to dependency updates
* Fixed panic on drop Cursive instance

### Added:

* flake.nix for dev shell `nix develop`
* Cargo.lock

## 0.1.2 (2022-01-21)

### Changed:

* Cargo.toml: cursive version

## 0.1.1 (2021-09-16)

### Changed:

* Cargo.toml: ntest dependency (wildcard * dependency constraints are not allowed on crates.io)
* CI.yml: removed Windows (ncurses doesn't support Windows)

## 0.1.0 (2021-09-16)

* Initial release
