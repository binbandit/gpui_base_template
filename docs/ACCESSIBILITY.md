# Accessibility boundary and release audit

This starter treats accessibility as a release requirement, not a styling extra.
It also documents what the pinned dependency can and cannot currently provide.

## GPUI 0.2.2 limitation

Published GPUI 0.2.2 exposes focus handles, tab stops, keyboard events, and focused
styles. It does **not** expose a public API for assigning assistive-technology
roles, names, values, checked/selected states, or live-region announcements to
custom elements.

The showcase therefore implements the parts this API line can support:

- every enabled custom control participates in keyboard traversal;
- Enter and Space dispatch the same typed actions as pointer clicks;
- disabled buttons are removed from the tab order;
- a high-contrast focus ring is visible in both themes;
- normal-sized semantic text pairs are guarded by WCAG AA contrast tests;
- narrow layouts stack dense content instead of only squeezing columns;
- shortcut labels render Command glyphs on macOS and Ctrl labels elsewhere.

The `Button`, `Toggle`, navigation, and status notice are still custom `div`
elements. A screen reader cannot reliably learn their button/switch/navigation/
status semantics on GPUI 0.2.2. Do not describe the generated application as
accessible or compliant until your product has resolved this limitation and
passed a platform-specific audit.

## Before each release

Test with a keyboard in both themes:

1. Tab and Shift-Tab through every route and control.
2. Confirm focus never disappears and follows a logical order.
3. Activate controls with Enter and Space.
4. Confirm disabled controls are skipped.
5. Resize from the default size down to the 720 × 520 minimum.
6. Verify content stacks, remains readable, and scrolls without clipping.
7. Trigger settings corruption, save failure, async completion, and clipboard
   feedback; verify each message is visible and actionable.
8. Verify shortcut labels and behavior on every supported operating system.

Then test with the platform screen reader and accessibility inspector. On the
pinned API, expect the semantic-control checks to fail; that is a known release
blocker for products requiring assistive-technology support, not a reason to
silence the audit.

## During a GPUI upgrade

Follow `docs/GPUI_VERSIONING.md`, then inspect the new release for public APIs
covering:

- role and accessible-name assignment;
- checked, selected, expanded, disabled, and value state;
- navigation/landmark structure;
- live status and alert announcements;
- focus-visible rather than focus-only styling.

When those APIs exist, wire them into `src/app/components/` and navigation first,
add interaction/accessibility tests, update this document, and only then change
the accessibility claim. Do not infer support from newer Zed `main` code while the
project remains pinned to the crates.io 0.2.2 API line.
