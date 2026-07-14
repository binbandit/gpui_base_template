use super::PageContext;
use crate::app::actions::{
    IncrementCounter, Quit, RunSync, ToggleAnimations, ToggleCompactSidebar, ToggleTheme,
};
use crate::app::components::{Badge, BadgeTone, Button, ButtonVariant, Card, Toggle};
use gpui::{AnyElement, IntoElement, div, prelude::*};

pub(super) fn render(context: &PageContext<'_>) -> AnyElement {
    let theme = context.theme;

    div()
        .flex()
        .flex_col()
        .gap_5()
        .child(
            Card::new()
                .title("Buttons")
                .child(
                    div()
                        .flex()
                        .flex_wrap()
                        .items_center()
                        .gap_3()
                        .child(
                            Button::new("primary-demo", "Increment counter", IncrementCounter)
                                .variant(ButtonVariant::Primary),
                        )
                        .child(
                            Button::new("secondary-demo", "Run task", RunSync)
                                .variant(ButtonVariant::Secondary)
                                .disabled(context.sync.is_running()),
                        )
                        .child(
                            Button::new("ghost-demo", "Toggle theme", ToggleTheme)
                                .variant(ButtonVariant::Ghost),
                        )
                        .child(
                            Button::new("danger-demo", "Quit app", Quit)
                                .variant(ButtonVariant::Danger),
                        ),
                )
                .child(div().text_xs().text_color(theme.text_muted).child(format!(
                    "Counter: {} — every control dispatches a typed GPUI action.",
                    context.counter
                ))),
        )
        .child(
            Card::new().title("Badges").child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_2()
                    .child(Badge::new("Neutral"))
                    .child(Badge::new("Accent").tone(BadgeTone::Accent))
                    .child(Badge::new("Healthy").tone(BadgeTone::Success))
                    .child(Badge::new("Attention").tone(BadgeTone::Warning)),
            ),
        )
        .child(
            Card::new()
                .title("Interactive settings")
                .child(Toggle::new(
                    "animations-demo",
                    "Motion",
                    "Respect this flag before adding decorative animation.",
                    context.settings.animations,
                    ToggleAnimations,
                ))
                .child(Toggle::new(
                    "sidebar-demo",
                    "Compact navigation",
                    "Useful for narrow or information-dense applications.",
                    context.settings.compact_sidebar,
                    ToggleCompactSidebar,
                )),
        )
        .into_any_element()
}
