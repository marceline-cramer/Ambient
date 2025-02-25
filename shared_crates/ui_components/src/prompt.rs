use ambient_cb::{cb, Cb};
use ambient_element::{element_component, Element, ElementComponentExt, Hooks};
use ambient_guest_bridge::{
    components::layout::{align_vertical_center, space_between_items},
    ecs::World,
};

use crate::{
    button::{Button, ButtonStyle},
    default_theme::{StylesExt, STREET},
    editor::{Editor, TextEditor},
    layout::{FlowColumn, FlowRow},
    screens::DialogScreen,
    scroll_area::ScrollArea,
    text::Text,
};

#[element_component]
pub fn Alert(
    _hooks: &mut Hooks,
    title: String,
    set_screen: Cb<dyn Fn(Option<Element>) + Sync + Send>,
    on_ok: Option<Cb<dyn Fn(&mut World) + Sync + Send>>,
    on_cancel: Option<Cb<dyn Fn(&mut World) + Sync + Send>>,
) -> Element {
    DialogScreen(
        FlowColumn::el([
            Text::el(title).header_style(),
            FlowRow::el([
                if let Some(on_ok) = on_ok.clone() {
                    let set_screen = set_screen.clone();
                    Button::new("Ok", move |world| {
                        set_screen(None);
                        on_ok(world);
                    })
                    .style(ButtonStyle::Primary)
                    .el()
                } else {
                    Element::new()
                },
                if let Some(on_cancel) = on_cancel.clone() {
                    let set_screen = set_screen.clone();
                    Button::new("Cancel", move |world| {
                        set_screen(None);
                        on_cancel(world);
                    })
                    .style(ButtonStyle::Primary)
                    .el()
                } else {
                    Element::new()
                },
            ])
            .with(space_between_items(), STREET),
        ])
        .with(space_between_items(), STREET),
    )
    .el()
}
impl Alert {
    /// Creates a new `Alert`. At least one of `on_ok` or `on_cancel` must be specified.
    ///
    /// If `on_ok` or `on_cancel` are `Some`, the respective button will exist.
    pub fn new(
        title: impl Into<String>,
        set_screen: Cb<dyn Fn(Option<Element>) + Sync + Send>,
        on_ok: Option<Cb<dyn Fn(&mut World) + Sync + Send>>,
        on_cancel: Option<Cb<dyn Fn(&mut World) + Sync + Send>>,
    ) -> Self {
        assert!(on_ok.is_some() || on_cancel.is_some());
        Self { title: title.into(), set_screen, on_ok, on_cancel }
    }
}

#[element_component]
pub fn Prompt(
    hooks: &mut Hooks,
    title: String,
    placeholder: Option<String>,
    on_ok: Cb<dyn Fn(&mut World, String) + Sync + Send>,
    on_cancel: Option<Cb<dyn Fn(&mut World) + Sync + Send>>,
) -> Element {
    let (value, set_value) = hooks.use_state("".to_string());
    DialogScreen(
        FlowColumn::el([
            Text::el(title).header_style(),
            TextEditor::new(value.clone(), set_value).placeholder(placeholder.or(Some("Enter value".to_string()))).el(),
            FlowRow::el([
                Button::new("Ok", move |world| {
                    on_ok(world, value.clone());
                })
                .style(ButtonStyle::Primary)
                .el(),
                if let Some(on_cancel) = on_cancel {
                    Button::new("Cancel", move |world| {
                        on_cancel(world);
                    })
                    .style(ButtonStyle::Flat)
                    .el()
                } else {
                    Element::new()
                },
            ])
            .with(space_between_items(), STREET)
            .with_default(align_vertical_center()),
        ])
        .with(space_between_items(), STREET),
    )
    .el()
}

impl Prompt {
    pub fn new(
        title: impl Into<String>,
        set_screen: Cb<dyn Fn(Option<Element>) + Sync + Send>,
        on_ok: impl Fn(&mut World, String) + Sync + Send + 'static,
    ) -> Self {
        Self {
            title: title.into(),
            placeholder: None,
            on_ok: cb(move |world, value| {
                on_ok(world, value);
                set_screen(None);
            }),
            on_cancel: None,
        }
    }

    pub fn new_cancelable(
        title: impl Into<String>,
        set_screen: Cb<dyn Fn(Option<Element>) + Sync + Send>,
        on_ok: impl Fn(&mut World, String) + Sync + Send + 'static,
    ) -> Self {
        Self {
            title: title.into(),
            placeholder: None,
            on_ok: cb({
                let set_screen = set_screen.clone();
                move |world, value| {
                    on_ok(world, value);
                    set_screen(None);
                }
            }),
            on_cancel: Some(cb(move |_| set_screen(None))),
        }
    }
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }
}

#[element_component]
pub fn EditorPrompt<T: Editor + std::fmt::Debug + Clone + Sync + Send + 'static>(
    hooks: &mut Hooks,
    title: String,
    value: T,
    set_screen: Cb<dyn Fn(Option<Element>) + Sync + Send>,
    on_ok: Cb<dyn Fn(&mut World, T) + Sync + Send>,
    on_cancel: Option<Cb<dyn Fn(&mut World) + Sync + Send>>,
    validator: Option<Cb<dyn Fn(&T) -> bool + Sync + Send>>,
) -> Element {
    let (value, set_value) = hooks.use_state(value);
    DialogScreen(
        ScrollArea(
            FlowColumn::el([
                Text::el(title).header_style(),
                value.clone().editor(set_value, Default::default()),
                FlowRow(vec![
                    Button::new("Ok", {
                        let set_screen = set_screen.clone();
                        let value = value.clone();
                        move |world| {
                            set_screen(None);
                            on_ok(world, value.clone());
                        }
                    })
                    .disabled(validator.map(|vv| !vv(&value)).unwrap_or(false))
                    .style(ButtonStyle::Primary)
                    .el(),
                    if let Some(on_cancel) = on_cancel {
                        Button::new("Cancel", move |world| {
                            set_screen(None);
                            on_cancel(world);
                        })
                        .style(ButtonStyle::Flat)
                        .el()
                    } else {
                        Element::new()
                    },
                ])
                .el()
                .with(space_between_items(), STREET)
                .with_default(align_vertical_center()),
            ])
            .with(space_between_items(), STREET),
        )
        .el(),
    )
    .el()
}

impl<T: Editor + std::fmt::Debug + Clone + Sync + Send + 'static> EditorPrompt<T> {
    pub fn new(
        title: impl Into<String>,
        value: T,
        set_screen: Cb<dyn Fn(Option<Element>) + Sync + Send>,
        on_ok: impl Fn(&mut World, T) + Sync + Send + 'static,
    ) -> Self {
        Self { title: title.into(), value, set_screen, on_ok: cb(on_ok), on_cancel: None, validator: None }
    }
}
