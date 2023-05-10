# tui-crates

Various crates related to handling terminal user interfaces.

# Maintenance Notes

## Upgrading Ratatui

1. upgrade `ratatui` in `tui-react`, increment mionr in `tui-react`  and `cargo publish`.
1. upgrade `ratatui` and `tui-react` in `crosstermion` and `cargo release`.

## Upgrading Crossterm

You can't - it comes as ratatui backend and thus we need ratatui to upgrade to the latest crossterm first.
