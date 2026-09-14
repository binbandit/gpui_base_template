# Pages

Each route-level screen has one module in this directory. `mod.rs` owns the
`Route` → page mapping and passes an immutable `PageContext` snapshot into the
selected screen.

Pages should:

- compose controls from `app::components`;
- render from the supplied state without mutating `RootView` directly;
- dispatch typed actions for user intent;
- keep page-only helper views private;
- move a helper into `components/` only after it is genuinely reused.

To add a screen, add its `Route` variant and actions, create a page module, extend
the mapping in `pages/mod.rs`, and add its navigation entry in
`shell/sidebar.rs`. State transitions and async ownership remain in `root.rs` (or
a dedicated entity/service once they outgrow it). Navigation restores root focus
before removing the old page. Keep the route-specific scroll-container identity
in `root.rs` so pages do not inherit one another’s scroll offset.

The starter has three intentionally small screens: Overview shows a simulated
background job and shared counter, Components lets you try the controls, and
Settings changes theme/navigation preferences. Keep sample data visibly labeled;
replace these screens with your own product instead of growing a showcase framework.
