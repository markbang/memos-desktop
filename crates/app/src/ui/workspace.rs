use std::collections::BTreeMap;

use chrono::{DateTime, Local, Utc};
use gpui::{
    AnyElement, Context, InteractiveElement as _, IntoElement, ParentElement as _,
    StatefulInteractiveElement as _, Styled, Window, div, prelude::FluentBuilder as _, px,
};
use gpui_component::{
    Disableable, Icon, IconName, Sizable, StyledExt, WindowExt as _,
    button::{Button, ButtonVariants as _},
    h_flex,
    input::Input,
    scroll::ScrollableElement,
    text::TextView,
    v_flex,
};
use memos_api::types::{Memo, MemoState, MemoVisibility};

use super::{DetailTab, MemosDesktop, QuickFilter, Route};
use crate::theme;

impl MemosDesktop {
    pub(super) fn render_workspace(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let show_inspector = matches!(self.route, Route::Timeline | Route::Archive)
            && self.selected_memo().is_some();

        h_flex()
            .id("workspace")
            .size_full()
            .bg(theme::paper())
            .text_color(theme::ink())
            .child(self.render_nav_rail(cx))
            .child(self.render_context_panel(cx))
            .child(self.render_content(window, cx))
            .when(show_inspector, |layout| {
                layout.child(self.render_inspector(window, cx))
            })
            .into_any_element()
    }

    fn render_nav_rail(&self, cx: &mut Context<Self>) -> AnyElement {
        v_flex()
            .id("nav-rail")
            .w(px(theme::NAV_WIDTH))
            .h_full()
            .flex_shrink_0()
            .items_center()
            .justify_between()
            .py_4()
            .bg(theme::nav())
            .border_r_1()
            .border_color(theme::color(0x30343b))
            .child(
                v_flex()
                    .items_center()
                    .gap_5()
                    .child(
                        div()
                            .relative()
                            .size_9()
                            .rounded(px(4.0))
                            .bg(theme::color(0xf1f2ee))
                            .text_color(theme::nav())
                            .font_semibold()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child("M")
                            .child(
                                div()
                                    .absolute()
                                    .right_0()
                                    .bottom_0()
                                    .size_2()
                                    .bg(theme::vermilion()),
                            ),
                    )
                    .child(
                        v_flex()
                            .items_center()
                            .gap_2()
                            .child(self.nav_button(
                                Route::Timeline,
                                IconName::GalleryVerticalEnd,
                                cx,
                            ))
                            .child(self.nav_button(Route::Explore, IconName::Globe, cx))
                            .child(self.nav_button(Route::Views, IconName::LayoutDashboard, cx))
                            .child(self.nav_button(Route::Archive, IconName::FolderClosed, cx))
                            .child(self.nav_button(Route::Attachments, IconName::File, cx))
                            .child(self.nav_button(Route::Inbox, IconName::Bell, cx)),
                    ),
            )
            .child(
                v_flex()
                    .items_center()
                    .gap_3()
                    .child(self.nav_button(Route::Settings, IconName::Settings, cx))
                    .child(
                        div()
                            .size_8()
                            .rounded(px(4.0))
                            .bg(theme::cobalt())
                            .text_color(theme::color(0xffffff))
                            .text_xs()
                            .font_semibold()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(self.user_initials()),
                    ),
            )
            .into_any_element()
    }

    fn nav_button(&self, route: Route, icon: IconName, cx: &mut Context<Self>) -> impl IntoElement {
        let selected = self.route == route;
        let view = cx.entity().clone();
        Button::new(gpui::SharedString::from(format!("nav-{:?}", route)))
            .ghost()
            .large()
            .icon(Icon::new(icon).size_4())
            .tooltip(route.title())
            .when(selected, |button| button.primary())
            .on_click(move |_, _, cx| {
                view.update(cx, |this, cx| this.navigate(route, cx));
            })
    }

    fn render_context_panel(&self, cx: &mut Context<Self>) -> AnyElement {
        let visible = self.visible_memos();
        let tags = tag_counts(&self.memos);
        let version = self
            .instance
            .as_ref()
            .and_then(|profile| profile.version.as_deref())
            .unwrap_or("unknown");
        let server = self
            .session
            .as_ref()
            .map(ApiSessionLabel::label)
            .or_else(|| {
                self.instance
                    .as_ref()
                    .and_then(|profile| profile.instance_url.clone())
            })
            .unwrap_or_else(|| "Local preview".into());

        v_flex()
            .id("context-panel")
            .w(px(theme::CONTEXT_WIDTH))
            .h_full()
            .flex_shrink_0()
            .border_r_1()
            .border_color(theme::line())
            .bg(theme::surface())
            .child(
                v_flex()
                    .gap_1()
                    .px_4()
                    .pt_4()
                    .pb_3()
                    .border_b_1()
                    .border_color(theme::line())
                    .child(
                        h_flex()
                            .items_center()
                            .justify_between()
                            .child(div().text_sm().font_semibold().child("MEMOS"))
                            .child(
                                div()
                                    .font_family(theme::mono_family())
                                    .text_xs()
                                    .text_color(theme::graphite())
                                    .child(format!("v{version}")),
                            ),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme::graphite())
                            .overflow_hidden()
                            .text_ellipsis()
                            .child(server),
                    ),
            )
            .child(
                v_flex()
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scrollbar()
                    .p_3()
                    .gap_5()
                    .child(
                        Input::new(&self.search_input)
                            .prefix(Icon::new(IconName::Search).size_4())
                            .cleanable(true),
                    )
                    .child(
                        v_flex()
                            .gap_1()
                            .child(panel_label("QUICK FILTERS"))
                            .children(
                                [
                                    QuickFilter::All,
                                    QuickFilter::Pinned,
                                    QuickFilter::Tasks,
                                    QuickFilter::Links,
                                    QuickFilter::Code,
                                ]
                                .into_iter()
                                .map(|filter| {
                                    let count = self
                                        .memos
                                        .iter()
                                        .filter(|memo| filter.matches(memo))
                                        .count();
                                    self.filter_row(filter, count, cx)
                                }),
                            ),
                    )
                    .child(
                        v_flex()
                            .gap_1()
                            .child(panel_label("TAGS"))
                            .when(tags.is_empty(), |panel| {
                                panel.child(
                                    div()
                                        .px_2()
                                        .py_2()
                                        .text_xs()
                                        .text_color(theme::graphite())
                                        .child("No tags"),
                                )
                            })
                            .children(tags.into_iter().take(12).map(|(tag, count)| {
                                h_flex()
                                    .px_2()
                                    .py_1p5()
                                    .items_center()
                                    .justify_between()
                                    .text_sm()
                                    .child(format!("#{tag}"))
                                    .child(
                                        div()
                                            .font_family(theme::mono_family())
                                            .text_xs()
                                            .text_color(theme::graphite())
                                            .child(count.to_string()),
                                    )
                            })),
                    )
                    .child(
                        v_flex().gap_1().child(panel_label("CURRENT RESULT")).child(
                            h_flex()
                                .px_2()
                                .py_2()
                                .justify_between()
                                .text_sm()
                                .child("Visible memos")
                                .child(
                                    div()
                                        .font_family(theme::mono_family())
                                        .text_color(theme::cobalt())
                                        .child(visible.len().to_string()),
                                ),
                        ),
                    ),
            )
            .into_any_element()
    }

    fn filter_row(&self, filter: QuickFilter, count: usize, cx: &mut Context<Self>) -> AnyElement {
        let selected = self.quick_filter == filter;
        let view = cx.entity().clone();
        h_flex()
            .id(gpui::SharedString::from(format!("filter-{:?}", filter)))
            .w_full()
            .h(px(34.0))
            .px_2()
            .items_center()
            .justify_between()
            .rounded(px(3.0))
            .cursor_pointer()
            .when(selected, |row| {
                row.bg(theme::pale_cobalt())
                    .text_color(theme::cobalt_dark())
            })
            .when(!selected, |row| {
                row.hover(|style| style.bg(theme::color(0xf1f3ef)))
            })
            .on_click(move |_, _, cx| {
                view.update(cx, |this, cx| this.set_quick_filter(filter, cx));
            })
            .child(div().text_sm().child(filter.label()))
            .child(
                div()
                    .font_family(theme::mono_family())
                    .text_xs()
                    .text_color(if selected {
                        theme::cobalt_dark()
                    } else {
                        theme::graphite()
                    })
                    .child(count.to_string()),
            )
            .into_any_element()
    }

    fn render_content(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        match self.route {
            Route::Timeline | Route::Archive | Route::Explore => {
                self.render_timeline_content(window, cx)
            }
            Route::Views => self.render_views_page(cx),
            Route::Attachments => self.render_attachments_page(cx),
            Route::Inbox => self.render_inbox_page(cx),
            Route::Settings => self.render_settings_page(cx),
        }
    }

    fn render_timeline_content(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let memos = self.visible_memos();
        let count = memos.len();
        let title = self.route.title();
        let subtitle = match self.route {
            Route::Timeline => "Private working stream",
            Route::Archive => "Memos outside the active stream",
            Route::Explore => "Visible memos from this instance",
            _ => "",
        };
        let is_timeline = self.route == Route::Timeline;
        let can_create = is_timeline && self.current_user.is_some();
        let loading = self.loading;
        let error = self.error.clone();

        v_flex()
            .id("content")
            .flex_1()
            .min_w_0()
            .h_full()
            .child(
                h_flex()
                    .h(px(64.0))
                    .flex_shrink_0()
                    .px_6()
                    .items_center()
                    .justify_between()
                    .border_b_1()
                    .border_color(theme::line())
                    .bg(theme::paper())
                    .child(
                        v_flex()
                            .gap_0p5()
                            .child(div().text_lg().font_semibold().child(title))
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme::graphite())
                                    .child(subtitle),
                            ),
                    )
                    .child(
                        h_flex()
                            .items_center()
                            .gap_3()
                            .child(
                                div()
                                    .font_family(theme::mono_family())
                                    .text_xs()
                                    .text_color(theme::graphite())
                                    .child(format!("{count:02} ITEMS")),
                            )
                            .child(
                                Button::new("refresh")
                                    .ghost()
                                    .icon(IconName::Redo2)
                                    .tooltip("Refresh")
                                    .loading(loading)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.reload_memos(cx);
                                    })),
                            ),
                    ),
            )
            .when(can_create, |content| {
                content.child(self.render_composer(cx))
            })
            .when_some(error, |content, error| {
                content.child(
                    h_flex()
                        .mx_6()
                        .mt_4()
                        .p_3()
                        .gap_2()
                        .border_1()
                        .border_color(theme::color(0xe1b6b1))
                        .bg(theme::color(0xfff4f2))
                        .text_sm()
                        .text_color(theme::color(0x9c3028))
                        .child(Icon::new(IconName::TriangleAlert).size_4())
                        .child(error),
                )
            })
            .child(
                v_flex()
                    .id("timeline-scroll")
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scrollbar()
                    .when(memos.is_empty() && !loading, |list| {
                        list.child(empty_state(
                            IconName::BookOpen,
                            "No memos in this view",
                            "Change the filter or capture a new memo.",
                        ))
                    })
                    .children(memos.into_iter().map(|memo| self.render_memo_row(memo, cx))),
            )
            .into_any_element()
    }

    fn render_composer(&self, cx: &mut Context<Self>) -> AnyElement {
        let saving = self.saving;
        v_flex()
            .mx_6()
            .mt_5()
            .mb_3()
            .border_1()
            .border_color(theme::line_strong())
            .bg(theme::surface())
            .rounded(px(4.0))
            .child(
                Input::new(&self.composer_input)
                    .appearance(false)
                    .bordered(false)
                    .focus_bordered(false)
                    .h(px(116.0)),
            )
            .child(
                h_flex()
                    .min_h(px(44.0))
                    .px_3()
                    .items_center()
                    .justify_between()
                    .border_t_1()
                    .border_color(theme::line())
                    .child(
                        h_flex()
                            .gap_1()
                            .child(self.visibility_button(
                                MemoVisibility::Private,
                                IconName::EyeOff,
                                "Private",
                                cx,
                            ))
                            .child(self.visibility_button(
                                MemoVisibility::Protected,
                                IconName::User,
                                "Protected",
                                cx,
                            ))
                            .child(self.visibility_button(
                                MemoVisibility::Public,
                                IconName::Globe,
                                "Public",
                                cx,
                            )),
                    )
                    .child(
                        h_flex()
                            .gap_3()
                            .items_center()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme::graphite())
                                    .child("Ctrl+Enter"),
                            )
                            .child(
                                Button::new("save-memo")
                                    .primary()
                                    .small()
                                    .icon(IconName::Plus)
                                    .label("Save")
                                    .loading(saving)
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.save_memo(window, cx);
                                    })),
                            ),
                    ),
            )
            .into_any_element()
    }

    fn visibility_button(
        &self,
        visibility: MemoVisibility,
        icon: IconName,
        label: &'static str,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let selected = self.visibility == visibility;
        let view = cx.entity().clone();
        Button::new(gpui::SharedString::from(format!("visibility-{visibility}")))
            .xsmall()
            .ghost()
            .icon(icon)
            .tooltip(label)
            .when(selected, |button| button.primary())
            .on_click(move |_, _, cx| {
                view.update(cx, |this, cx| this.set_visibility(visibility, cx));
            })
    }

    fn render_memo_row(&self, memo: Memo, cx: &mut Context<Self>) -> AnyElement {
        let name = memo.name.clone().unwrap_or_else(|| "memos/unknown".into());
        let selected = self.selected_memo_name.as_deref() == Some(name.as_str());
        let view = cx.entity().clone();
        let title = memo
            .property
            .as_ref()
            .and_then(|property| property.title.clone());
        let excerpt = memo_excerpt(&memo.content, title.as_deref());
        let visibility = visibility_label(memo.visibility);
        let timestamp = memo.create_time;

        h_flex()
            .id(gpui::SharedString::from(format!("memo-{name}")))
            .w_full()
            .border_b_1()
            .border_color(theme::line())
            .cursor_pointer()
            .when(selected, |row| row.bg(theme::color(0xf0f4ff)))
            .when(!selected, |row| {
                row.hover(|style| style.bg(theme::color(0xf9faf7)))
            })
            .on_click(move |_, _, cx| {
                view.update(cx, |this, cx| {
                    this.select_memo(Some(name.clone()), cx);
                });
            })
            .child(
                v_flex()
                    .w(px(96.0))
                    .flex_shrink_0()
                    .items_end()
                    .pt_5()
                    .pr_4()
                    .border_r_1()
                    .border_color(theme::line())
                    .child(
                        div()
                            .font_family(theme::mono_family())
                            .text_xs()
                            .text_color(theme::graphite())
                            .child(timestamp.map(relative_time).unwrap_or_else(|| "now".into())),
                    )
                    .when(memo.pinned.unwrap_or(false), |column| {
                        column.child(div().mt_2().size_1p5().bg(theme::vermilion()))
                    }),
            )
            .child(
                v_flex()
                    .flex_1()
                    .min_w_0()
                    .gap_3()
                    .px_5()
                    .py_5()
                    .when_some(title, |body, title| {
                        body.child(div().text_base().font_semibold().child(title))
                    })
                    .child(
                        div()
                            .text_sm()
                            .line_height(px(22.0))
                            .text_color(theme::color(0x30343a))
                            .whitespace_normal()
                            .child(excerpt),
                    )
                    .when(!memo.tags.is_empty(), |body| {
                        body.child(h_flex().gap_3().flex_wrap().children(memo.tags.iter().map(
                            |tag| {
                                div()
                                    .text_xs()
                                    .text_color(theme::cobalt_dark())
                                    .child(format!("#{tag}"))
                            },
                        )))
                    })
                    .child(
                        h_flex()
                            .items_center()
                            .gap_3()
                            .text_xs()
                            .text_color(theme::graphite())
                            .child(visibility)
                            .when(!memo.attachments.is_empty(), |metadata| {
                                metadata.child(format!("{} attachments", memo.attachments.len()))
                            })
                            .when(!memo.reactions.is_empty(), |metadata| {
                                metadata.child(format!("{} reactions", memo.reactions.len()))
                            }),
                    ),
            )
            .into_any_element()
    }

    fn render_inspector(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let Some(memo) = self.selected_memo().cloned() else {
            return div().into_any_element();
        };
        let name = memo.name.clone().unwrap_or_else(|| "memos/unknown".into());
        let timestamp = memo
            .create_time
            .map(|time| {
                time.with_timezone(&Local)
                    .format("%Y-%m-%d %H:%M")
                    .to_string()
            })
            .unwrap_or_else(|| "Unknown time".into());
        let view = cx.entity().clone();
        let pin_name = name.clone();
        let archive_name = name.clone();
        let delete_name = name.clone();
        let is_archived = memo.state == MemoState::Archived;
        let is_pinned = memo.pinned.unwrap_or(false);

        v_flex()
            .id("inspector")
            .w(px(theme::INSPECTOR_WIDTH))
            .h_full()
            .flex_shrink_0()
            .border_l_1()
            .border_color(theme::line())
            .bg(theme::surface())
            .child(
                h_flex()
                    .h(px(64.0))
                    .flex_shrink_0()
                    .px_4()
                    .items_center()
                    .justify_between()
                    .border_b_1()
                    .border_color(theme::line())
                    .child(
                        v_flex()
                            .gap_0p5()
                            .child(div().text_sm().font_semibold().child("Memo detail"))
                            .child(
                                div()
                                    .font_family(theme::mono_family())
                                    .text_xs()
                                    .text_color(theme::graphite())
                                    .child(resource_id(&name)),
                            ),
                    )
                    .child(
                        Button::new("close-inspector")
                            .ghost()
                            .icon(IconName::PanelRightClose)
                            .tooltip("Close detail")
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.select_memo(None, cx);
                            })),
                    ),
            )
            .child(
                v_flex()
                    .flex_1()
                    .min_h_0()
                    .p_4()
                    .gap_4()
                    .child(
                        h_flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme::graphite())
                                    .child(timestamp),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .font_semibold()
                                    .text_color(visibility_color(memo.visibility))
                                    .child(visibility_label(memo.visibility)),
                            ),
                    )
                    .child(self.render_detail_panel(window, cx, memo.clone(), name.clone()))
                    .when(!memo.tags.is_empty(), |panel| {
                        panel.child(h_flex().gap_2().flex_wrap().children(memo.tags.iter().map(
                            |tag| {
                                div()
                                    .px_2()
                                    .py_1()
                                    .border_1()
                                    .border_color(theme::line())
                                    .rounded(px(3.0))
                                    .text_xs()
                                    .text_color(theme::cobalt_dark())
                                    .child(format!("#{tag}"))
                            },
                        )))
                    })
                    .child(
                        h_flex()
                            .gap_1()
                            .pt_3()
                            .border_t_1()
                            .border_color(theme::line())
                            .child(
                                Button::new("toggle-pin")
                                    .small()
                                    .ghost()
                                    .icon(if is_pinned {
                                        IconName::StarOff
                                    } else {
                                        IconName::Star
                                    })
                                    .tooltip(if is_pinned { "Unpin" } else { "Pin" })
                                    .on_click(move |_, _, cx| {
                                        view.update(cx, |this, cx| {
                                            this.toggle_pin(pin_name.clone(), cx);
                                        });
                                    }),
                            )
                            .child({
                                let view = cx.entity().clone();
                                Button::new("toggle-archive")
                                    .small()
                                    .ghost()
                                    .icon(IconName::FolderClosed)
                                    .tooltip(if is_archived { "Restore" } else { "Archive" })
                                    .on_click(move |_, _, cx| {
                                        view.update(cx, |this, cx| {
                                            this.toggle_archive(archive_name.clone(), cx);
                                        });
                                    })
                            })
                            .child({
                                let view = cx.entity().clone();
                                Button::new("delete-memo")
                                    .small()
                                    .danger()
                                    .icon(IconName::Delete)
                                    .tooltip("Delete")
                                    .on_click(move |_, window, cx| {
                                        let view = view.clone();
                                        let memo_name = delete_name.clone();
                                        window.open_dialog(cx, move |dialog, _, _| {
                                            let confirm_view = view.clone();
                                            let confirm_name = memo_name.clone();
                                            dialog
                                                .title("Delete memo")
                                                .child("This memo will be permanently deleted.")
                                                .footer(move |_, _, _, _| {
                                                    let confirm_view = confirm_view.clone();
                                                    let confirm_name = confirm_name.clone();
                                                    vec![
                                                        Button::new("confirm-delete")
                                                            .danger()
                                                            .label("Delete")
                                                            .on_click(move |_, window, cx| {
                                                                window.close_dialog(cx);
                                                                confirm_view.update(
                                                                    cx,
                                                                    |this, cx| {
                                                                        this.delete_memo(
                                                                            confirm_name.clone(),
                                                                            cx,
                                                                        );
                                                                    },
                                                                );
                                                            }),
                                                        Button::new("cancel-delete")
                                                            .label("Cancel")
                                                            .on_click(|_, window, cx| {
                                                                window.close_dialog(cx);
                                                            }),
                                                    ]
                                                })
                                        });
                                    })
                            }),
                    ),
            )
            .into_any_element()
    }

    fn render_detail_panel(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
        memo: Memo,
        name: String,
    ) -> AnyElement {
        let detail_error = self.detail_error.clone();
        v_flex()
            .flex_1()
            .min_h_0()
            .gap_3()
            .child(
                h_flex()
                    .gap_1()
                    .border_b_1()
                    .border_color(theme::line())
                    .child(self.detail_tab_button(DetailTab::Content, cx))
                    .child(self.detail_tab_button(DetailTab::Activity, cx))
                    .child(self.detail_tab_button(DetailTab::Links, cx))
                    .child(self.detail_tab_button(DetailTab::Share, cx))
                    .child(self.detail_tab_button(DetailTab::Files, cx)),
            )
            .when_some(detail_error, |panel, error| {
                panel.child(
                    div()
                        .text_xs()
                        .text_color(theme::color(0x9c3028))
                        .child(error),
                )
            })
            .child(self.render_detail_body(window, cx, memo, name))
            .into_any_element()
    }

    fn detail_tab_button(&self, tab: DetailTab, cx: &mut Context<Self>) -> impl IntoElement {
        let selected = self.detail_tab == tab;
        let view = cx.entity().clone();
        Button::new(match tab {
            DetailTab::Content => "detail-content",
            DetailTab::Activity => "detail-activity",
            DetailTab::Links => "detail-links",
            DetailTab::Share => "detail-share",
            DetailTab::Files => "detail-files",
        })
        .xsmall()
        .ghost()
        .when(selected, |button| button.primary())
        .label(tab.label())
        .on_click(move |_, _, cx| {
            view.update(cx, |this, cx| this.set_detail_tab(tab, cx));
        })
    }

    fn render_detail_body(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
        memo: Memo,
        name: String,
    ) -> AnyElement {
        match self.detail_tab {
            DetailTab::Content => div()
                .flex_1()
                .min_h_0()
                .w_full()
                .child(
                    TextView::markdown(
                        gpui::SharedString::from(format!("inspector-{name}")),
                        memo.content,
                        window,
                        cx,
                    )
                    .selectable(true)
                    .scrollable(true)
                    .w_full()
                    .h_full(),
                )
                .into_any_element(),
            DetailTab::Activity => self.render_activity_panel(cx),
            DetailTab::Links => self.render_links_panel(),
            DetailTab::Share => self.render_share_panel(cx),
            DetailTab::Files => self.render_files_panel(window, cx),
        }
    }

    fn render_activity_panel(&self, cx: &mut Context<Self>) -> AnyElement {
        let reactions = reaction_counts(&self.detail.reactions);
        let comments = self.detail.comments.clone();
        let view = cx.entity().clone();
        v_flex()
            .flex_1()
            .min_h_0()
            .gap_3()
            .child(
                h_flex()
                    .items_center()
                    .gap_1()
                    .flex_wrap()
                    .children(reactions.into_iter().map(|(reaction, count)| {
                        div()
                            .px_2()
                            .py_1()
                            .border_1()
                            .border_color(theme::line())
                            .rounded(px(3.0))
                            .text_xs()
                            .child(format!("{reaction} {count}"))
                    }))
                    .child(
                        Button::new("add-reaction")
                            .xsmall()
                            .ghost()
                            .label("👍")
                            .tooltip("React")
                            .on_click(move |_, _, cx| {
                                view.update(cx, |this, cx| {
                                    this.add_reaction("👍".into(), cx);
                                });
                            }),
                    ),
            )
            .child(
                v_flex()
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scrollbar()
                    .gap_3()
                    .when(comments.is_empty(), |list| {
                        list.child(
                            div()
                                .text_xs()
                                .text_color(theme::graphite())
                                .child("No comments yet."),
                        )
                    })
                    .children(comments.into_iter().map(|comment| {
                        v_flex()
                            .gap_1()
                            .pb_3()
                            .border_b_1()
                            .border_color(theme::line())
                            .child(
                                div()
                                    .font_family(theme::mono_family())
                                    .text_xs()
                                    .text_color(theme::graphite())
                                    .child(
                                        comment
                                            .create_time
                                            .map(relative_time)
                                            .unwrap_or_else(|| "now".into()),
                                    ),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .line_height(px(20.0))
                                    .child(memo_excerpt(&comment.content, None)),
                            )
                    })),
            )
            .child(
                h_flex()
                    .items_center()
                    .gap_2()
                    .child(Input::new(&self.comment_input))
                    .child(
                        Button::new("send-comment")
                            .small()
                            .primary()
                            .icon(IconName::ArrowUp)
                            .tooltip("Send comment")
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.save_comment(window, cx);
                            })),
                    ),
            )
            .into_any_element()
    }

    fn render_links_panel(&self) -> AnyElement {
        let relations = self.detail.relations.clone();
        v_flex()
            .flex_1()
            .min_h_0()
            .overflow_y_scrollbar()
            .gap_2()
            .when(relations.is_empty(), |list| {
                list.child(
                    div()
                        .text_xs()
                        .text_color(theme::graphite())
                        .child("No references or backlinks."),
                )
            })
            .children(relations.into_iter().map(|relation| {
                let relation_type = format!("{:?}", relation.type_).to_lowercase();
                v_flex()
                    .gap_1()
                    .p_2()
                    .border_1()
                    .border_color(theme::line())
                    .rounded(px(3.0))
                    .child(
                        div()
                            .font_family(theme::mono_family())
                            .text_xs()
                            .text_color(theme::cobalt_dark())
                            .child(relation_type),
                    )
                    .child(
                        div().text_xs().child(
                            relation
                                .related_memo
                                .snippet
                                .unwrap_or(relation.related_memo.name),
                        ),
                    )
            }))
            .into_any_element()
    }

    fn render_share_panel(&self, cx: &mut Context<Self>) -> AnyElement {
        let shares = self.detail.shares.clone();
        let view = cx.entity().clone();
        v_flex()
            .flex_1()
            .min_h_0()
            .gap_3()
            .child(
                h_flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme::graphite())
                            .child("Read-only share links"),
                    )
                    .child(
                        Button::new("create-share")
                            .small()
                            .primary()
                            .icon(IconName::Plus)
                            .tooltip("Create share link")
                            .on_click(move |_, _, cx| {
                                view.update(cx, |this, cx| this.create_share(cx));
                            }),
                    ),
            )
            .child(
                v_flex()
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scrollbar()
                    .gap_2()
                    .when(shares.is_empty(), |list| {
                        list.child(
                            div()
                                .text_xs()
                                .text_color(theme::graphite())
                                .child("No active share links."),
                        )
                    })
                    .children(shares.into_iter().map(|share| {
                        let share_name = share.name.clone().unwrap_or_default();
                        let token = resource_id(&share_name);
                        let share_url = self
                            .session
                            .as_ref()
                            .map(|session| format!("{}/memos/shares/{token}", session.base_url()))
                            .unwrap_or_else(|| format!("memos/shares/{token}"));
                        let view = cx.entity().clone();
                        let delete_name = share_name.clone();
                        h_flex()
                            .gap_1()
                            .items_center()
                            .p_2()
                            .border_1()
                            .border_color(theme::line())
                            .rounded(px(3.0))
                            .child(
                                div()
                                    .flex_1()
                                    .min_w_0()
                                    .font_family(theme::mono_family())
                                    .text_xs()
                                    .text_color(theme::cobalt_dark())
                                    .overflow_hidden()
                                    .text_ellipsis()
                                    .child(share_url.clone()),
                            )
                            .child(
                                Button::new("copy-share")
                                    .xsmall()
                                    .ghost()
                                    .icon(IconName::Copy)
                                    .tooltip("Copy link")
                                    .on_click(move |_, _, cx| {
                                        cx.write_to_clipboard(gpui::ClipboardItem::new_string(
                                            share_url.clone(),
                                        ));
                                    }),
                            )
                            .child(
                                Button::new("delete-share")
                                    .xsmall()
                                    .danger()
                                    .icon(IconName::Delete)
                                    .tooltip("Revoke link")
                                    .on_click(move |_, _, cx| {
                                        view.update(cx, |this, cx| {
                                            this.delete_share(delete_name.clone(), cx);
                                        });
                                    }),
                            )
                    })),
            )
            .into_any_element()
    }

    fn render_files_panel(&self, _window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let attachments = self.detail.attachments.clone();
        let view = cx.entity().clone();
        v_flex()
            .flex_1()
            .min_h_0()
            .gap_3()
            .child(
                h_flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme::graphite())
                            .child("Files attached to this memo"),
                    )
                    .child(
                        Button::new("upload-attachment")
                            .small()
                            .primary()
                            .icon(IconName::Plus)
                            .tooltip("Upload attachment")
                            .disabled(self.current_user.is_none())
                            .on_click(move |_, window, cx| {
                                view.update(cx, |this, cx| {
                                    this.upload_attachment(window, cx);
                                });
                            }),
                    ),
            )
            .child(
                v_flex()
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scrollbar()
                    .gap_2()
                    .when(attachments.is_empty(), |list| {
                        list.child(
                            div()
                                .text_xs()
                                .text_color(theme::graphite())
                                .child("No attachments on this memo."),
                        )
                    })
                    .children(attachments.into_iter().map(|attachment| {
                        h_flex()
                            .gap_2()
                            .items_center()
                            .p_2()
                            .border_1()
                            .border_color(theme::line())
                            .rounded(px(3.0))
                            .child(Icon::new(IconName::File).size_4())
                            .child(
                                v_flex()
                                    .flex_1()
                                    .min_w_0()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_xs()
                                            .overflow_hidden()
                                            .text_ellipsis()
                                            .child(attachment.filename),
                                    )
                                    .child(
                                        div()
                                            .font_family(theme::mono_family())
                                            .text_xs()
                                            .text_color(theme::graphite())
                                            .child(attachment.type_),
                                    ),
                            )
                    })),
            )
            .into_any_element()
    }

    fn render_views_page(&self, cx: &mut Context<Self>) -> AnyElement {
        let view = cx.entity().clone();
        let server_views = self.memo_views.clone();
        let server_view_rows = server_views.into_iter().map(|memo_view| {
            h_flex()
                .min_h(px(56.0))
                .px_4()
                .items_center()
                .justify_between()
                .border_b_1()
                .border_color(theme::line())
                .child(
                    v_flex()
                        .gap_1()
                        .child(div().text_sm().font_semibold().child(memo_view.title))
                        .child(
                            div()
                                .font_family(theme::mono_family())
                                .text_xs()
                                .text_color(theme::graphite())
                                .child(memo_view.filter),
                        ),
                )
                .child(
                    div()
                        .font_family(theme::mono_family())
                        .text_xs()
                        .text_color(theme::cobalt_dark())
                        .child("SERVER"),
                )
        });
        module_page(
            "Saved views",
            "Reusable filters",
            v_flex()
                .gap_0()
                .border_1()
                .border_color(theme::line())
                .when(!self.memo_views.is_empty(), |panel| {
                    panel
                        .child(panel_label("SERVER VIEWS"))
                        .children(server_view_rows)
                        .child(div().h(px(12.0)))
                })
                .child(panel_label("QUICK VIEWS"))
                .children(
                    [
                        ("Pinned", "pinned == true", QuickFilter::Pinned),
                        (
                            "Open tasks",
                            "has_incomplete_tasks == true",
                            QuickFilter::Tasks,
                        ),
                        ("Code notes", "has_code == true", QuickFilter::Code),
                    ]
                    .into_iter()
                    .map(move |(title, filter, quick_filter)| {
                        let view = view.clone();
                        h_flex()
                            .min_h(px(56.0))
                            .px_4()
                            .items_center()
                            .justify_between()
                            .border_b_1()
                            .border_color(theme::line())
                            .child(
                                v_flex()
                                    .gap_1()
                                    .child(div().text_sm().font_semibold().child(title))
                                    .child(
                                        div()
                                            .font_family(theme::mono_family())
                                            .text_xs()
                                            .text_color(theme::graphite())
                                            .child(filter),
                                    ),
                            )
                            .child(
                                Button::new(gpui::SharedString::from(format!("open-view-{title}")))
                                    .ghost()
                                    .icon(IconName::ArrowRight)
                                    .tooltip("Open view")
                                    .on_click(move |_, _, cx| {
                                        view.update(cx, |this, cx| {
                                            this.route = Route::Timeline;
                                            this.set_quick_filter(quick_filter, cx);
                                        });
                                    }),
                            )
                    }),
                ),
        )
    }

    fn render_attachments_page(&self, _cx: &mut Context<Self>) -> AnyElement {
        let attachments = if self.library_attachments.is_empty() {
            self.memos
                .iter()
                .flat_map(|memo| memo.attachments.iter().cloned())
                .collect::<Vec<_>>()
        } else {
            self.library_attachments.clone()
        };
        let content = v_flex()
            .border_1()
            .border_color(theme::line())
            .when(attachments.is_empty(), |list| {
                list.child(empty_state(
                    IconName::File,
                    "No attachments",
                    "Uploaded files will appear here.",
                ))
            })
            .children(attachments.into_iter().map(|attachment| {
                h_flex()
                    .min_h(px(52.0))
                    .px_4()
                    .items_center()
                    .justify_between()
                    .border_b_1()
                    .border_color(theme::line())
                    .child(
                        h_flex()
                            .gap_3()
                            .items_center()
                            .child(Icon::new(IconName::File).size_4())
                            .child(attachment.filename),
                    )
                    .child(
                        div()
                            .font_family(theme::mono_family())
                            .text_xs()
                            .text_color(theme::graphite())
                            .child(attachment.type_),
                    )
            }));
        module_page("Attachments", "Instance file library", content)
    }

    fn render_inbox_page(&self, _cx: &mut Context<Self>) -> AnyElement {
        let notifications = self.notifications.clone();
        let content = v_flex()
            .border_1()
            .border_color(theme::line())
            .when(notifications.is_empty(), |list| {
                list.child(empty_state(
                    IconName::Bell,
                    "Inbox is clear",
                    "New mentions and comments will appear here.",
                ))
            })
            .children(notifications.into_iter().map(|notification| {
                let kind = notification
                    .type_
                    .map(|kind| format!("{kind:?}"))
                    .unwrap_or_else(|| "Notification".into());
                let sender = notification
                    .sender
                    .unwrap_or_else(|| "Unknown sender".into());
                let name = notification
                    .name
                    .unwrap_or_else(|| "notifications/unknown".into());
                h_flex()
                    .min_h(px(64.0))
                    .px_4()
                    .items_center()
                    .justify_between()
                    .border_b_1()
                    .border_color(theme::line())
                    .child(
                        h_flex()
                            .gap_3()
                            .items_center()
                            .child(Icon::new(IconName::Bell).size_4())
                            .child(
                                v_flex()
                                    .gap_1()
                                    .child(div().text_sm().font_semibold().child(kind))
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme::graphite())
                                            .child(format!("from {sender}")),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .font_family(theme::mono_family())
                            .text_xs()
                            .text_color(theme::graphite())
                            .child(resource_id(&name)),
                    )
            }));
        module_page("Inbox", "Mentions and comments", content)
    }

    fn render_settings_page(&self, cx: &mut Context<Self>) -> AnyElement {
        let display_name = self
            .current_user
            .as_ref()
            .and_then(|user| user.display_name.clone())
            .unwrap_or_else(|| "Public session".into());
        let username = self
            .current_user
            .as_ref()
            .map(|user| format!("@{}", user.username))
            .unwrap_or_else(|| "Anonymous".into());
        let server = self
            .session
            .as_ref()
            .map(|session| session.base_url().to_string())
            .or_else(|| {
                self.instance
                    .as_ref()
                    .and_then(|profile| profile.instance_url.clone())
            })
            .unwrap_or_else(|| "Local preview".into());

        module_page(
            "Settings",
            "Account and instance",
            v_flex()
                .border_1()
                .border_color(theme::line())
                .child(setting_row("Account", &display_name, Some(&username)))
                .child(setting_row("Server", &server, None))
                .child(setting_row(
                    "Version",
                    self.instance
                        .as_ref()
                        .and_then(|profile| profile.version.as_deref())
                        .unwrap_or("Unknown"),
                    None,
                ))
                .child(
                    h_flex()
                        .min_h(px(64.0))
                        .px_4()
                        .items_center()
                        .justify_between()
                        .border_t_1()
                        .border_color(theme::line())
                        .child(
                            v_flex()
                                .gap_1()
                                .child(div().text_sm().font_semibold().child("Session"))
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme::graphite())
                                        .child("Disconnect from this instance"),
                                ),
                        )
                        .child(
                            Button::new("disconnect")
                                .danger()
                                .outline()
                                .label("Disconnect")
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.disconnect(cx);
                                })),
                        ),
                ),
        )
    }

    fn user_initials(&self) -> String {
        let source = self
            .current_user
            .as_ref()
            .and_then(|user| user.display_name.as_deref())
            .or_else(|| {
                self.current_user
                    .as_ref()
                    .map(|user| user.username.as_str())
            })
            .unwrap_or("G");
        source
            .split_whitespace()
            .filter_map(|part| part.chars().next())
            .take(2)
            .collect::<String>()
            .to_uppercase()
    }
}

trait ApiSessionLabel {
    fn label(&self) -> String;
}

impl ApiSessionLabel for crate::api::ApiSession {
    fn label(&self) -> String {
        self.base_url().to_string()
    }
}

fn panel_label(label: &'static str) -> impl IntoElement {
    div()
        .px_2()
        .pb_1()
        .font_family(theme::mono_family())
        .text_xs()
        .text_color(theme::graphite())
        .child(label)
}

fn module_page(
    title: &'static str,
    subtitle: &'static str,
    content: impl IntoElement,
) -> AnyElement {
    v_flex()
        .id(title)
        .flex_1()
        .min_w_0()
        .h_full()
        .child(
            v_flex()
                .h(px(88.0))
                .flex_shrink_0()
                .px_6()
                .justify_center()
                .border_b_1()
                .border_color(theme::line())
                .child(div().text_lg().font_semibold().child(title))
                .child(
                    div()
                        .text_xs()
                        .text_color(theme::graphite())
                        .child(subtitle),
                ),
        )
        .child(
            v_flex()
                .flex_1()
                .min_h_0()
                .overflow_y_scrollbar()
                .p_6()
                .child(content),
        )
        .into_any_element()
}

fn empty_state(icon: IconName, title: &'static str, description: &'static str) -> AnyElement {
    v_flex()
        .w_full()
        .min_h(px(220.0))
        .items_center()
        .justify_center()
        .gap_3()
        .text_center()
        .child(Icon::new(icon).size_6().text_color(theme::graphite()))
        .child(div().text_sm().font_semibold().child(title))
        .child(
            div()
                .text_xs()
                .text_color(theme::graphite())
                .child(description),
        )
        .into_any_element()
}

fn setting_row(label: &'static str, value: &str, secondary: Option<&str>) -> AnyElement {
    h_flex()
        .min_h(px(64.0))
        .px_4()
        .items_center()
        .justify_between()
        .border_b_1()
        .border_color(theme::line())
        .child(div().text_sm().font_semibold().child(label))
        .child(
            v_flex()
                .items_end()
                .gap_1()
                .child(div().text_sm().child(value.to_string()))
                .when_some(secondary, |column, secondary| {
                    column.child(
                        div()
                            .text_xs()
                            .text_color(theme::graphite())
                            .child(secondary.to_string()),
                    )
                }),
        )
        .into_any_element()
}

fn reaction_counts(reactions: &[memos_api::types::Reaction]) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for reaction in reactions {
        *counts.entry(reaction.reaction_type.clone()).or_insert(0) += 1;
    }
    counts
}

fn tag_counts(memos: &[Memo]) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for tag in memos.iter().flat_map(|memo| memo.tags.iter()) {
        *counts.entry(tag.clone()).or_insert(0) += 1;
    }
    counts
}

fn memo_excerpt(content: &str, title: Option<&str>) -> String {
    let mut text = content.to_string();
    if let Some(title) = title {
        text = text
            .strip_prefix(&format!("# {title}"))
            .unwrap_or(&text)
            .trim_start()
            .to_string();
    }
    let flattened = text
        .lines()
        .filter(|line| !line.trim_start().starts_with("# "))
        .take(10)
        .collect::<Vec<_>>()
        .join("\n");
    if flattened.chars().count() > 520 {
        format!("{}...", flattened.chars().take(517).collect::<String>())
    } else {
        flattened
    }
}

fn relative_time(time: DateTime<Utc>) -> String {
    let elapsed = Utc::now() - time;
    if elapsed.num_minutes() < 1 {
        "now".into()
    } else if elapsed.num_hours() < 1 {
        format!("{}m", elapsed.num_minutes())
    } else if elapsed.num_days() < 1 {
        format!("{}h", elapsed.num_hours())
    } else if elapsed.num_days() < 30 {
        format!("{}d", elapsed.num_days())
    } else {
        time.with_timezone(&Local).format("%b %d").to_string()
    }
}

fn visibility_label(visibility: MemoVisibility) -> &'static str {
    match visibility {
        MemoVisibility::Private => "Private",
        MemoVisibility::Protected => "Protected",
        MemoVisibility::Public => "Public",
        MemoVisibility::VisibilityUnspecified => "Unspecified",
    }
}

fn visibility_color(visibility: MemoVisibility) -> gpui::Hsla {
    match visibility {
        MemoVisibility::Private => theme::graphite(),
        MemoVisibility::Protected => theme::amber(),
        MemoVisibility::Public => theme::signal_green(),
        MemoVisibility::VisibilityUnspecified => theme::graphite(),
    }
}

fn resource_id(name: &str) -> String {
    name.rsplit('/').next().unwrap_or(name).to_string()
}
