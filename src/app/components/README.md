# Reusable components

This directory is the application’s small, owned UI kit. Put a control here when
multiple pages use it or when its interaction and visual behavior should remain
consistent everywhere.

## Choose the right GPUI shape

- **Stateless presentation or controlled input:** implement `RenderOnce` and derive
  `IntoElement`, as `Button` and `Toggle` do.
- **Persistent local state, focus editing, subscriptions, or async ownership:** use
  an `Entity<T>` that implements `Render`.
- **Custom layout/paint behavior:** implement a GPUI `Element` only when normal
  elements are insufficient.

Do not add a generic application `Component` trait. GPUI already owns rendering,
entity lifetimes, actions, and element composition.

## Component checklist

1. Use semantic values from `Theme`; do not scatter raw colors through pages.
2. Accept a typed GPUI action when pointer and keyboard activation mean the same
   thing.
3. Give interactive controls a stable element ID, keyboard behavior, and visible
   focus treatment.
4. Remove disabled controls from tab traversal and activation handlers.
5. Keep product/domain state in the owning entity; controlled components receive
   their current value as input.
6. Document GPUI 0.2.2’s accessibility boundary in `docs/ACCESSIBILITY.md`; custom
   controls cannot currently expose semantic roles/states.
7. Export the component from `mod.rs` and demonstrate it on the Components page.

## Example

```rust
use crate::app::actions::Save;
use crate::app::components::{Button, ButtonVariant};

let save = Button::new("save-project", "Save", Save)
    .variant(ButtonVariant::Primary)
    .disabled(is_saving);
```

`Card`, `Badge`, `Button`, and `Toggle` are public through
`app::components`, so the library-shaped template can also reuse them from other
crate targets. Product-specific composites should generally remain in their page
module until a second caller proves that they are reusable.
