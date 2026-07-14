# Application shell

The shell is stable chrome shared by every route: sidebar navigation, the page
header, and transient notices. Shell modules receive state from `RootView` and
dispatch typed actions; they do not own product state.

Keep route-specific content in `pages/` and broadly reusable controls in
`components/`. Add shell modules only for UI that truly wraps or coordinates all
screens, such as command palettes, global overlays, or window-level status.
