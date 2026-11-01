use bevy::ecs::system::IntoObserverSystem;
use bevy::text::FontSmoothing;
use bevy::text::LineHeight;
use std::borrow::Cow;

use crate::animation::backup::Backup;
use crate::animation::offset::NodeOffset;
use crate::prelude::*;
use crate::util::rich_text::RichText;

/// A root UI node that fills the window and centers its content.
pub fn ui_root(name: impl Into<Cow<'static, str>>) -> impl Scene {
    let name = name.into();
    bsn! {
        Name::new(name)
        Node {
            position_type: PositionType::Absolute,
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: px(20),
        }
        // Don't block picking events for other UI roots.
        Pickable::IGNORE
    }
}
//
// /// A simple header label. Bigger than [`label`].
// pub fn header(text: impl Into<String>) -> impl Scene {
//     bsn! {
//         #Header
//         Text({ text.into() })
//         TextFont { font_size: px(40.0) }
//         TextColor(HEADER_TEXT)
//     }
// }
//
// /// A simple text label.
// pub fn label(text: impl Into<String>) -> impl Scene {
//     bsn! {
//         #Label
//         Text({ text.into() })
//         TextFont { font_size: px(24.0) }
//         TextColor(LABEL_TEXT)
//     }
// }
//
// /// A large rounded button with text and an action defined as an [`Observer`].
// pub fn button<E, M, I>(text: impl Into<String>, action: I) -> impl Scene
// where
//     E: EntityEvent,
//     I: IntoObserverSystem<E, M, ()> + Clone + Sync,
//     M: 'static,
// {
//     button_base(
//         text,
//         action,
//         bsn! {
//             Node {
//                 width: px(380),
//                 height: px(80),
//                 align_items: AlignItems::Center,
//                 justify_content: JustifyContent::Center,
//                 border_radius: BorderRadius::MAX,
//             }
//         },
//     )
// }
//
// /// A small square button with text and an action defined as an [`Observer`].
// pub fn button_small<E, M, I>(text: impl Into<String>, action: I) -> impl Scene
// where
//     E: EntityEvent,
//     I: IntoObserverSystem<E, M, ()> + Clone + Sync,
//     M: 'static,
// {
//     button_base(
//         text,
//         action,
//         bsn! {
//             Node {
//                 width: px(30),
//                 height: px(30),
//                 align_items: AlignItems::Center,
//                 justify_content: JustifyContent::Center,
//             }
//         },
//     )
// }
//
// /// A simple button with text and an action defined as an [`Observer`]. The button's layout is provided by `button_bundle`.
// fn button_base<E, M, I>(text: impl Into<String>, action: I, button_bundle: impl Scene) -> impl Scene
// where
//     E: EventPattern<Event: EntityEvent>,
//     I: IntoObserverSystem<E, M, ()> + Clone + Sync,
//     M: 'static,
// {
//     let text = text.into();
//
//     bsn! {
//         #Button
//         Node
//         Children [
//             #ButtonInner
//             // Button
//             BackgroundColor(BUTTON_BACKGROUND)
//             InteractionPalette {
//                 none: BUTTON_BACKGROUND,
//                 hovered: BUTTON_HOVERED_BACKGROUND,
//                 pressed: BUTTON_PRESSED_BACKGROUND,
//             }
//             Children [
//                 #ButtonText
//                 Text(text)
//                 TextFont { font_size: px(40.0) }
//                 TextColor(BUTTON_TEXT)
//                 // Don't bubble picking events from the text up to the button.
//                 Pickable::IGNORE
//             ]
//             @button_bundle
//             @on(action)
//         ]
//     }
// }

/// TODO ------------------------------------------------
/// TODO ------------------------------------------------
pub fn overlay(z: i32) -> impl Bundle {
    (
        Name::new("Overlay"),
        Node::DEFAULT.full_size().abs(),
        Pickable::IGNORE,
        GlobalZIndex(z),
    )
}

pub fn blocking_overlay(z: i32) -> impl Bundle {
    (
        Name::new("BlockingOverlay"),
        Node::DEFAULT.full_size().abs(),
        FocusPolicy::Block,
        GlobalZIndex(z),
    )
}

pub fn spacer(space: Val) -> impl Bundle {
    (Name::new("Spacer"), Node::DEFAULT.size(space, space))
}

pub fn root(children: impl Bundle) -> impl Bundle {
    (
        Name::new("Root"),
        Node {
            padding: UiRect::all(Vw(4.0)),
            ..Node::COLUMN.full_size()
        },
        children,
    )
}

pub fn header(children: impl Bundle) -> impl Bundle {
    (
        Name::new("Header"),
        Node {
            margin: UiRect::bottom(Vw(4.0)),
            ..Node::ROW.center().full_width()
        },
        children,
    )
}

pub fn body(children: impl Bundle) -> impl Bundle {
    (
        Name::new("Body"),
        Node::COLUMN.center_left().full_width().grow(),
        children,
    )
}

pub fn footer(children: impl Bundle) -> impl Bundle {
    (
        Name::new("Footer"),
        Node {
            margin: UiRect::top(Vw(4.0)),
            ..Node::ROW.center().full_width()
        },
        children,
    )
}

pub fn full_popup(children: impl Bundle) -> impl Bundle {
    // popup(Percent(100.0), Percent(100.0), children)
    popup(children)
}

pub fn popup(children: impl Bundle) -> impl Bundle {
    (
        Name::new("PopupContainer"),
        Node {
            padding: UiRect::all(Vw(5.0)),
            align_items: AlignItems::Center,
            width: percent(100),
            height: percent(100),
            ..default()
        },
        children![(
            Name::new("Popup"),
            Node {
                display: Display::Block,
                padding: UiRect::all(Vw(3.5)),
                border: UiRect::all(Px(1.0)),
                width: percent(100),
                height: percent(100),
                border_radius: BorderRadius::all(Vw(3.0)),
                ..default()
            },
            ThemeColor::Popup.set::<BackgroundColor>(),
            ThemeColor::BorderColor.set::<BorderColor>(),
            BoxShadow::from(ShadowStyle {
                color: Color::BLACK.with_alpha(0.5),
                x_offset: Val::ZERO,
                y_offset: Val::ZERO,
                spread_radius: Val::ZERO,
                blur_radius: Val::Vw(4.0),
            }),
            FocusPolicy::Block,
            children,
        )],
    )
}

pub fn stretch(children: impl Bundle) -> impl Bundle {
    (Name::new("Stretch"), Node::ROW.center().grow(), children)
}

pub fn center(children: impl Bundle) -> impl Bundle {
    (
        Name::new("Center"),
        Node::COLUMN.center().full_size(),
        children,
    )
}

pub fn column_of_buttons(children: impl Bundle) -> impl Bundle {
    (
        Name::new("ColumnOfButtons"),
        Node {
            row_gap: Vw(2.5),
            ..Node::COLUMN.center()
        },
        children,
    )
}

pub fn row_of_buttons(children: impl Bundle) -> impl Bundle {
    (
        Name::new("RowOfButtons"),
        Node {
            column_gap: Vw(2.5),
            ..Node::ROW.center()
        },
        children,
    )
}

pub fn h1(text: impl AsRef<str>) -> impl Bundle {
    label_base(Vw(5.0), 1.2, Justify::Center, ThemeColor::BodyText, text)
}

pub fn big_label(text: impl AsRef<str>) -> impl Bundle {
    label_base(Vw(5.0), 1.2, Justify::Left, ThemeColor::BodyText, text)
}

pub fn label(text: impl AsRef<str>) -> impl Bundle {
    label_base(Vw(3.5), 1.2, Justify::Left, ThemeColor::BodyText, text)
}

pub fn label_base(
    font_size: Val,
    line_height: f32,
    justify: Justify,
    text_color: ThemeColor,
    text: impl AsRef<str>,
) -> impl Bundle {
    let text = text.as_ref();
    let rich_text = RichText::from_sections(parse_rich(text))
        .with_justify(justify)
        .with_font_smoothing(FontSmoothing::None)
        .with_line_height(LineHeight::RelativeToFont(line_height));
    let text_colors =
        std::iter::repeat_n(text_color, rich_text.sections.len().max(1)).collect::<Vec<_>>();

    (
        Name::new(format!("Label(\"{text}\")")),
        rich_text,
        DynamicFontSize::new(font_size).with_step(8.0),
        ThemeColorForText(text_colors),
    )
}

pub fn small_button<E, M, I>(text: impl Into<String>, action: I) -> impl Bundle
where
    E: EntityEvent,
    // E: Event,
    // E: EventPattern<Event: EntityEvent>,
    // I: IntoObserverSystem<E, M, ()> + Clone + Sync,
    I: IntoObserverSystem<E, M, ()> + Sync,
    M: 'static,
{
    button_base(Vw(3.0), Vw(4.0), Vw(3.0), text, action)
}

pub fn button<E, M, I>(text: impl Into<String>, action: I) -> impl Bundle
where
    E: EntityEvent,
    // E: Event,
    // E: EventPattern<Event: EntityEvent>,
    // I: IntoObserverSystem<E, M, ()> + Clone + Sync,
    I: IntoObserverSystem<E, M, ()> + Sync,
    M: 'static,
{
    button_base(Vw(30.0), Vw(7.0), Vw(3.0), text, action)
}

pub fn wide_button<E, M, I>(text: impl Into<String>, action: I) -> impl Bundle
where
    // E: EventPattern<Event: EntityEvent>,
    // E: Event,
    // I: IntoObserverSystem<E, M, ()> + Clone + Sync,
    E: EntityEvent,
    I: IntoObserverSystem<E, M, ()> + Sync,
    M: 'static,
{
    button_base(Vw(38.0), Vw(7.0), Vw(3.0), text, action)
}

pub fn big_button<E, M, I>(text: impl Into<String>, action: I) -> impl Bundle
where
    // E: EventPattern<Event: EntityEvent>,
    // E: Event,
    // I: IntoObserverSystem<E, M, ()> + Clone + Sync,
    E: EntityEvent,
    I: IntoObserverSystem<E, M, ()> + Sync,
    M: 'static,
{
    button_base(Vw(38.0), Vw(10.0), Vw(4.0), text, action)
}

pub fn button_base<E, M, I>(
    width: Val,
    height: Val,
    font_size: Val,
    text: impl Into<String>,
    action: I,
) -> impl Bundle
where
    // E: Event,
    // E: EventPattern<Event: EntityEvent>,
    // I: IntoObserverSystem<E, M, ()> + Clone + Sync,
    E: EntityEvent,
    I: IntoObserverSystem<E, M, ()> + Sync,
    M: 'static,
{
    let text = text.into();
    let action = IntoObserverSystem::into_system(action);
    // (
    //     Name::new(format!("Button(\"{text}\")")),
    //     Node::ROW.center().size(width, height),
    //     BorderRadius::MAX,
    //     ThemeColor::default().set::<BackgroundColor>(),
    //     BoxShadow::from(ShadowStyle {
    //         color: Color::BLACK.with_alpha(0.5),
    //         x_offset: Val::ZERO,
    //         y_offset: Vw(0.7),
    //         spread_radius: Vw(0.5),
    //         blur_radius: Vw(0.5),
    //     }),
    //     Backup::<BoxShadow>::default(),
    //     InteractionTheme {
    //         none: ThemeColor::Primary.set::<BackgroundColor>(),
    //         hovered: ThemeColor::PrimaryHovered.set::<BackgroundColor>(),
    //         pressed: ThemeColor::PrimaryPressed.set::<BackgroundColor>(),
    //         disabled: ThemeColor::PrimaryDisabled.set::<BackgroundColor>(),
    //     },
    //     NodeOffset::default(),
    //     InteractionTheme {
    //         hovered: NodeOffset::new(Val::ZERO, Vw(-0.5)),
    //         pressed: NodeOffset::new(Val::ZERO, Vw(0.5)),
    //         ..default()
    //     },
    //     InteractionSfx,
    //     children![(
    //         label_base(
    //             font_size,
    //             1.2,
    //             Justify::Center,
    //             ThemeColor::PrimaryText,
    //             text
    //         ),
    //         Pickable::IGNORE,
    //     )],
    //     // Patch(|entity| {
    //     //     entity.observe(action);
    //     // }),
    // )
    (
        Name::new(format!("Button(\"{text}\")")),
        Node::ROW.center().size(width, height),
        Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
            parent
                .spawn((
                    // BorderRadius::MAX,
                    ThemeColor::default().set::<BackgroundColor>(),
                    BoxShadow::from(ShadowStyle {
                        color: Color::BLACK.with_alpha(0.5),
                        x_offset: Val::ZERO,
                        y_offset: Vw(0.7),
                        spread_radius: Vw(0.5),
                        blur_radius: Vw(0.5),
                    }),
                    Backup::<BoxShadow>::default(),
                    InteractionTheme {
                        none: ThemeColor::Primary.set::<BackgroundColor>(),
                        hovered: ThemeColor::PrimaryHovered.set::<BackgroundColor>(),
                        pressed: ThemeColor::PrimaryPressed.set::<BackgroundColor>(),
                        disabled: ThemeColor::PrimaryDisabled.set::<BackgroundColor>(),
                    },
                    NodeOffset::default(),
                    InteractionTheme {
                        hovered: NodeOffset::new(Val::ZERO, Vw(-0.5)),
                        pressed: NodeOffset::new(Val::ZERO, Vw(0.5)),
                        ..default()
                    },
                    InteractionSfx,
                    children![(
                        label_base(
                            font_size,
                            1.2,
                            Justify::Center,
                            ThemeColor::PrimaryText,
                            text
                        ),
                        Pickable::IGNORE,
                    )],
                ))
                .observe(action);
        })),
    )
}

pub fn selector<E1, M1, I1, C, E2, M2, I2>(
    marker: C,
    left_action: I1,
    right_action: I2,
) -> impl Bundle
where
    C: Component,
    E1: EntityEvent,
    // E1: EventPattern<Event: EntityEvent>,
    // E1: Event,
    I1: Sync + IntoObserverSystem<E1, M1, ()>,
    M1: 'static,
    E2: EntityEvent,
    // E2: EventPattern<Event: EntityEvent>,
    // E2: Event,
    I2: Sync + IntoObserverSystem<E2, M2, ()>,
    M2: 'static,
{
    (
        Name::new("Selector"),
        Node::ROW.width(Vw(35.0)),
        marker,
        children![
            (small_button("<", left_action), InteractionDisabled(false)),
            stretch(children![label("")]),
            (small_button(">", right_action), InteractionDisabled(false)),
        ],
    )
}

pub fn loading_bar<S: State + Clone + PartialEq + Eq + Hash + Debug>() -> impl Bundle {
    (
        Name::new("LoadingBar"),
        Node {
            padding: UiRect::all(Vw(0.5)),
            border: UiRect::all(Vw(0.5)),
            ..Node::DEFAULT.size(Percent(60.0), Vw(4.0))
        },
        ThemeColor::BodyText.set::<BorderColor>(),
        children![(
            Name::new("LoadingBarFill"),
            LoadingBarFill::<S>(PhantomData),
            Node::DEFAULT.full_height(),
            ThemeColor::Primary.set::<BackgroundColor>(),
        )],
    )
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct LoadingBarFill<S: State + Clone + PartialEq + Eq + Hash + Debug>(
    #[reflect(ignore)] PhantomData<S>,
);

impl<S: State + Clone + PartialEq + Eq + Hash + Debug + TypePath> Configure for LoadingBarFill<S> {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            update_loading_bar_fill::<S>.in_set(UpdateSystems::SyncLate),
        );
    }
}

fn update_loading_bar_fill<S: State + Clone + PartialEq + Eq + Hash + Debug>(
    progress: Res<ProgressTracker<BevyState<S>>>,
    mut fill_query: Query<&mut Node, With<LoadingBarFill<S>>>,
) {
    let Progress { done, total } = progress.get_global_combined_progress();

    for mut node in &mut fill_query {
        node.width = Percent(100.0 * done as f32 / total as f32);
    }
}
