use super::PageContext;
use crate::app::actions::{IncrementCounter, RunSync, ToggleCompactSidebar, ToggleTheme};
use crate::app::components::{Badge, BadgeTone, Button, ButtonSize, ButtonVariant, Card, Toggle};
use gpui::{AnyElement, IntoElement, div, prelude::*};

pub(super) fn render(context: &PageContext<'_>) -> AnyElement {
    let theme = context.theme;

    div()
        .flex()
        .flex_col()
        .gap_6()
        .child(
            div()
                .flex()
                .flex_col()
                .gap_2()
                .pt_3()
                .child(
                    div()
                        .text_3xl()
                        .font_weight(gpui::FontWeight::SEMIBOLD)
                        .child("Small pieces. Real interactions."),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(theme.text_muted)
                        .child("A focused set of controls for the screens you’ll build."),
                ),
        )
        .child(
            Card::new()
                .title("Actions")
                .child(
                    div()
                        .text_sm()
                        .text_color(theme.text_muted)
                        .child("Try a button. The counter is shared with Overview."),
                )
                .child(
                    div()
                        .flex()
                        .flex_wrap()
                        .items_center()
                        .gap_3()
                        .child(
                            Button::new("primary-demo", "Add one", IncrementCounter)
                                .variant(ButtonVariant::Primary),
                        )
                        .child(
                            Button::new("secondary-demo", "Run demo sync", RunSync)
                                .disabled(context.sync.is_running()),
                        )
                        .child(
                            Button::new("ghost-demo", "Switch theme", ToggleTheme)
                                .variant(ButtonVariant::Ghost),
                        )
                        .child(
                            Button::new("disabled-demo", "Unavailable", IncrementCounter)
                                .disabled(true),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .gap_4()
                        .pt_3()
                        .border_t_1()
                        .border_color(theme.border)
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_3()
                                .child(
                                    div()
                                        .text_2xl()
                                        .font_weight(gpui::FontWeight::SEMIBOLD)
                                        .child(context.counter.to_string()),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_muted)
                                        .child("counter value"),
                                ),
                        )
                        .child(
                            Button::new("small-demo", "+1", IncrementCounter)
                                .size(ButtonSize::Small),
                        ),
                ),
        )
        .child(
            Card::new()
                .title("Status")
                .child(
                    div()
                        .text_sm()
                        .text_color(theme.text_muted)
                        .child("Use a label alongside color to communicate state."),
                )
                .child(
                    div()
                        .flex()
                        .flex_wrap()
                        .gap_2()
                        .child(Badge::new("Draft"))
                        .child(Badge::new("In progress").tone(BadgeTone::Accent))
                        .child(Badge::new("Complete").tone(BadgeTone::Success))
                        .child(Badge::new("Needs attention").tone(BadgeTone::Warning)),
                ),
        )
        .child(Card::new().title("Preferences").child(Toggle::new(
            "sidebar-demo",
            "Compact navigation",
            "Show icons in the sidebar to make more room for your work.",
            context.settings.compact_sidebar,
            ToggleCompactSidebar,
        )))
        .child(
            div()
                .text_xs()
                .text_color(theme.text_faint)
                .child("Keyboard friendly: Tab to move, Enter or Space to activate."),
        )
        .into_any_element()
}
