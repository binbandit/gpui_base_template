# Application shell

The shell is stable chrome shared by every route: sidebar navigation, the page
header, and notices. Shell modules receive state from `RootView` and
dispatch typed actions; they do not own product state. Notices remain visible
across navigation until dismissed or replaced by subsequent feedback.

Keep route-specific content in `pages/` and broadly reusable controls in
`components/`. Add shell modules only for UI that truly wraps or coordinates all
screens, such as command palettes, global overlays, or window-level status.

The sidebar uses embedded, theme-colored SVG icons. Compact navigation retains
hover labels and the same keyboard actions. The header stays a small breadcrumb
and appearance control; page headings belong to the page.
