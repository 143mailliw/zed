use gpui::ClickEvent;

use crate::{Divider, IconButtonShape, prelude::*};

#[derive(IntoElement, RegisterComponent)]
pub struct NumericStepper {
    id: ElementId,
    value: SharedString,
    on_decrement: Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>,
    on_increment: Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>,
    /// Whether to reserve space for the reset button.
    reserve_space_for_reset: bool,
    on_reset: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    border: bool,
}

impl NumericStepper {
    pub fn new(
        id: impl Into<ElementId>,
        value: impl Into<SharedString>,
        on_decrement: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
        on_increment: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            value: value.into(),
            on_decrement: Box::new(on_decrement),
            on_increment: Box::new(on_increment),
            border: false,
            reserve_space_for_reset: false,
            on_reset: None,
        }
    }

    pub fn reserve_space_for_reset(mut self, reserve_space_for_reset: bool) -> Self {
        self.reserve_space_for_reset = reserve_space_for_reset;
        self
    }

    pub fn on_reset(
        mut self,
        on_reset: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_reset = Some(Box::new(on_reset));
        self
    }

    pub fn border(mut self) -> Self {
        self.border = true;
        self
    }
}

impl RenderOnce for NumericStepper {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let shape = IconButtonShape::Square;
        let icon_size = IconSize::Small;

        h_flex()
            .id(self.id)
            .gap_1()
            .map(|element| {
                if let Some(on_reset) = self.on_reset {
                    element.child(
                        IconButton::new("reset", IconName::RotateCcw)
                            .shape(shape)
                            .icon_size(icon_size)
                            .on_click(on_reset),
                    )
                } else if self.reserve_space_for_reset {
                    element.child(
                        h_flex()
                            .size(icon_size.square(window, cx))
                            .flex_none()
                            .into_any_element(),
                    )
                } else {
                    element
                }
            })
            .child(
                h_flex()
                    .gap_1()
                    .when(self.border, |this| {
                        this.border_1().border_color(cx.theme().colors().border)
                    })
                    .px_1()
                    .rounded_sm()
                    .bg(cx.theme().colors().editor_background)
                    .child(
                        IconButton::new("decrement", IconName::Dash)
                            .shape(shape)
                            .icon_size(icon_size)
                            .on_click(self.on_decrement),
                    )
                    .when(self.border, |this| {
                        this.child(Divider::vertical().color(super::DividerColor::Border))
                    })
                    .child(Label::new(self.value))
                    .when(self.border, |this| {
                        this.child(Divider::vertical().color(super::DividerColor::Border))
                    })
                    .child(
                        IconButton::new("increment", IconName::Plus)
                            .shape(shape)
                            .icon_size(icon_size)
                            .on_click(self.on_increment),
                    ),
            )
    }
}

impl Component for NumericStepper {
    fn scope() -> ComponentScope {
        ComponentScope::Input
    }

    fn name() -> &'static str {
        "NumericStepper"
    }

    fn sort_name() -> &'static str {
        Self::name()
    }

    fn description() -> Option<&'static str> {
        Some("A button used to increment or decrement a numeric value. ")
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        Some(
            v_flex()
                .child(single_example(
                    "Borderless",
                    NumericStepper::new(
                        "numeric-stepper-component-preview",
                        "10",
                        move |_, _, _| {},
                        move |_, _, _| {},
                    )
                    .into_any_element(),
                ))
                .child(single_example(
                    "Border",
                    NumericStepper::new(
                        "numeric-stepper-with-border-component-preview",
                        "10",
                        move |_, _, _| {},
                        move |_, _, _| {},
                    )
                    .border()
                    .into_any_element(),
                ))
                .into_any_element(),
        )
    }
}
