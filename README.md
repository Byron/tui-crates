# tui-crates

Various crates related to handling terminal user interfaces.

# Maintenance Notes

## Upgrading TUI

1. upgrade `tui` in `tui-react`, increment minor in `tui-react`  and `cargo publish`.
1. uprgade `tui` and `tui-react` in `crosstermion` and `cargo release`.


## Upgrading Crossterm

You can't - it comes as tui backend and thus we need tui to upgrade to the latest crossterm first.
