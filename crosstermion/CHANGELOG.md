# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.13.0 (2023-12-26)

### New Features (BREAKING)

 - <csr-id-7b49dd81a3f7c209279e90c1110de00dfd6a0701/> upgrade to `ratatui` v0.25
 - <csr-id-2df671a3b4e33e3c960c91e6c10f9e8fe6c94a11/> Remove support for termion; use `crossterm` events directly.
   Also remove custom Key/Event abstractions and use the ones provided
   by `crossterm` instead. This is very much a breaking change,
   but requires only mechanical adjustments.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 19 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release tui-react v0.22.0, safety bump crosstermion v0.13.0 ([`b7283a5`](https://github.com/Byron/tui-crates/commit/b7283a511abadfdbbb6cfbb2832ee1d84c06815c))
    - Merge branch 'remove-termion' ([`c69813f`](https://github.com/Byron/tui-crates/commit/c69813f00e114313b206bceaa838e2fe0954f37b))
    - Upgrade to `ratatui` v0.25 ([`7b49dd8`](https://github.com/Byron/tui-crates/commit/7b49dd81a3f7c209279e90c1110de00dfd6a0701))
    - Remove support for termion; use `crossterm` events directly. ([`2df671a`](https://github.com/Byron/tui-crates/commit/2df671a3b4e33e3c960c91e6c10f9e8fe6c94a11))
</details>

## 0.12.1 (2023-12-07)

### New Features

 - <csr-id-5533fe6db074b0ba132ea01df41bde45182a4933/> replace `ansi-term` with `ansiterm`.
   One is maintained, the other one is not.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release crosstermion v0.12.1 ([`7e7a324`](https://github.com/Byron/tui-crates/commit/7e7a324e094c121508adb1514b4aa39575f539aa))
    - Replace `ansi-term` with `ansiterm`. ([`5533fe6`](https://github.com/Byron/tui-crates/commit/5533fe6db074b0ba132ea01df41bde45182a4933))
</details>

## 0.12.0 (2023-12-07)

<csr-id-d337c866f48dda83166d8cafaaa97bbf24d69216/>
<csr-id-a371272a96da49e8db82221eaf23d3aab38514d0/>

### Chore

 - <csr-id-d337c866f48dda83166d8cafaaa97bbf24d69216/> upgrade all packages as long as they don't break the API

### Chore (BREAKING)

 - <csr-id-a371272a96da49e8db82221eaf23d3aab38514d0/> upgrade `ratatui` to 0.24 and `crossterm` to v0.27.
   Note that these two work together, and depend on each other.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 210 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release tui-react v0.21.0, crosstermion v0.12.0, safety bump crosstermion v0.12.0 ([`942f0b9`](https://github.com/Byron/tui-crates/commit/942f0b97a4b86b91e26530cca914906645f87ac4))
    - Merge branch 'upgrades' ([`c6265c8`](https://github.com/Byron/tui-crates/commit/c6265c8773829f54e4acb1106d56ef2239edcab7))
    - Upgrade `ratatui` to 0.24 and `crossterm` to v0.27. ([`a371272`](https://github.com/Byron/tui-crates/commit/a371272a96da49e8db82221eaf23d3aab38514d0))
    - Upgrade all packages as long as they don't break the API ([`d337c86`](https://github.com/Byron/tui-crates/commit/d337c866f48dda83166d8cafaaa97bbf24d69216))
</details>

## 0.11.0 (2023-05-11)

<csr-id-073005de8c9177248528a4cc9be58d6cee525394/>

### Chore (BREAKING)

 - <csr-id-073005de8c9177248528a4cc9be58d6cee525394/> switch to 'ratatui' in a fashion that leaves imports alone.
   However would still be a breaking change for anyone leaving `tui` in
   their dependency tree.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 241 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release tui-react v0.20.0, crosstermion v0.11.0, safety bump crosstermion v0.11.0 ([`6cc6346`](https://github.com/Byron/tui-crates/commit/6cc634618e1549fcece860b7b43f5d7ee5d1d259))
    - Merge branch 'ratatui-2' ([`948b539`](https://github.com/Byron/tui-crates/commit/948b5397c0b8317e857f79515165c04314b6838a))
    - Further updates to `crossterm` and `termion` ([`3157b3b`](https://github.com/Byron/tui-crates/commit/3157b3bf392b18f0f6be7e2c04d0b8782d76314e))
    - Switch to 'ratatui' in a fashion that leaves imports alone. ([`073005d`](https://github.com/Byron/tui-crates/commit/073005de8c9177248528a4cc9be58d6cee525394))
</details>

## 0.10.1 (2022-09-12)

### New Features

 - <csr-id-509c64c4918d7e1fbd462de5ec3ae83bf3329785/> Support for reseize events via the new `input_channel()` function.
   Note that it only exists in blocking mode.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release crosstermion v0.10.1 ([`b1775e8`](https://github.com/Byron/tui-crates/commit/b1775e8da35f3df4f9e10af8f95b0931c1ee8d2b))
    - Support for reseize events via the new `input_channel()` function. ([`509c64c`](https://github.com/Byron/tui-crates/commit/509c64c4918d7e1fbd462de5ec3ae83bf3329785))
    - Cargo fmt ([`d31a4bd`](https://github.com/Byron/tui-crates/commit/d31a4bd53dde9db3160236ac132237f76513ad12))
</details>

## 0.10.0 (2022-09-12)

Upgrade to `tui` 0.19 and `crossterm` 0.25.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 231 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release crosstermion v0.10.0 ([`420d808`](https://github.com/Byron/tui-crates/commit/420d8087f49a87985d3331a384287481d43f1217))
    - Update changelog prior to release ([`7e1330d`](https://github.com/Byron/tui-crates/commit/7e1330dfc070cd8c383d8b273622709e67a0c985))
    - Upgrade to crossterm 25 and tui 19 ([`ec53e20`](https://github.com/Byron/tui-crates/commit/ec53e202e7fb1809b94de0b26a010f61704c6a74))
    - Allow for the Resize event to happen ([`e49a08a`](https://github.com/Byron/tui-crates/commit/e49a08a9143e9d4684ffdf387face6cdb7b24367))
</details>

## 0.9.0 (2022-01-23)

### New Features (BREAKING)

 - <csr-id-a3223b3d39cb71adb1b866653ee0c984924b90ca/> upgrade to tui 0.17

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 170 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release crosstermion v0.9.0 ([`c5fc690`](https://github.com/Byron/tui-crates/commit/c5fc69022161e0abb2dd6c1402a606381ac2cce9))
    - Upgrade to tui 0.17 ([`a3223b3`](https://github.com/Byron/tui-crates/commit/a3223b3d39cb71adb1b866653ee0c984924b90ca))
    - Release tui-react v0.17.0 ([`4523678`](https://github.com/Byron/tui-crates/commit/4523678efbc9c876e46325682861c27ee5e7fb02))
    - Use local tui-react crate ([`9281c8e`](https://github.com/Byron/tui-crates/commit/9281c8e65226ff3a34ade7b95ef96374c9a759ea))
</details>

## v0.8.1 (2021-08-05)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 1 calendar day.
 - 1 day passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.8.1 ([`5fe230b`](https://github.com/Byron/tui-crates/commit/5fe230b6d85eaace655fad10ddac9608d2e62072))
    - Fix apparently untested macr ([`c9ea9ac`](https://github.com/Byron/tui-crates/commit/c9ea9ac069ce7951252b3620052d611c925f6afd))
    - Fix format ([`6323479`](https://github.com/Byron/tui-crates/commit/6323479162d69d22c55f4307be3eedd18c55f2af))
</details>

## v0.8.0 (2021-08-04)

* upgrade to TUI 0.16 and crossterm 0.20

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 93 calendar days.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Upgrade to crossterm 0.20 ([`1d31acb`](https://github.com/Byron/tui-crates/commit/1d31acbfde475ae42ac928d0e23ff950e669e2e7))
    - Make CI work on windows ([`dc0e96f`](https://github.com/Byron/tui-crates/commit/dc0e96f982bf46c8b910ca316e92357c2091559b))
    - Use a workspace ([`ccc5add`](https://github.com/Byron/tui-crates/commit/ccc5add08476a1910d8d78d3bb94b70237f50958))
    - Changes repository paths in tui crates manifests ([`ac5c6e6`](https://github.com/Byron/tui-crates/commit/ac5c6e62c86189a72e2305d06da176821f88b180))
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

