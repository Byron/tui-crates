# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 170 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release tui-react v0.17.0 ([`4523678`](https://github.com/Byron/tui-crates/commit/4523678efbc9c876e46325682861c27ee5e7fb02))
    - Use local tui-react crate ([`9281c8e`](https://github.com/Byron/tui-crates/commit/9281c8e65226ff3a34ade7b95ef96374c9a759ea))
</details>

## v0.8.1 (2021-08-05)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 1 calendar day.
 - 1 day passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.8.1 ([`5fe230b`](https://github.com/Byron/tui-crates/commit/5fe230b6d85eaace655fad10ddac9608d2e62072))
    - Fix apparently untested macr ([`c9ea9ac`](https://github.com/Byron/tui-crates/commit/c9ea9ac069ce7951252b3620052d611c925f6afd))
    - fix format ([`6323479`](https://github.com/Byron/tui-crates/commit/6323479162d69d22c55f4307be3eedd18c55f2af))
</details>

## v0.8.0 (2021-08-04)

* upgrade to TUI 0.16 and crossterm 0.20

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 93 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Upgrade to crossterm 0.20 ([`1d31acb`](https://github.com/Byron/tui-crates/commit/1d31acbfde475ae42ac928d0e23ff950e669e2e7))
    - Make CI work on windows ([`dc0e96f`](https://github.com/Byron/tui-crates/commit/dc0e96f982bf46c8b910ca316e92357c2091559b))
    - Use a workspace ([`ccc5add`](https://github.com/Byron/tui-crates/commit/ccc5add08476a1910d8d78d3bb94b70237f50958))
    - changes repository paths in tui crates manifests ([`ac5c6e6`](https://github.com/Byron/tui-crates/commit/ac5c6e62c86189a72e2305d06da176821f88b180))
    - Add tui crates ([`ccb6a24`](https://github.com/Byron/tui-crates/commit/ccb6a24315a7d881e50b24e98d4720406bff16d5))
</details>

## v0.7.0

* upgrade to TUI 0.15

## v0.4.0

* upgrade to TUI 0.12

## v0.3.2

* Remove `futures-util` dependency in favor of `futures-lite`

## v0.2.0

* remove native support for `flume` and `crossbeam-channel` for key input in favor of `std::sync::mpsc::Receiver`s :).

## v0.1.5

* support for simple cursor movements
* get terminal size
* color support to learn if color is allowed, and dynamically disable it
  if `ansi_term` is used.

## v0.1.4

* Fix precendence of imports in 'terminal' module.

## v0.1.3

* `Key` type conversions are now always compiled when possible, as they are not mutually exclusive

## v0.1.2

* Enable `flume/select` by default and allow selecting `flume/async` via the `flume-async` feature.

## v0.1.1

* Add support for 'input-thread-flume' for using flume channels instead of crossbeam ones. They are
  smaller and there is no unsafe code either, at the expense of lack of the 'select!()` capability.

## v0.1.0

Initial release.

