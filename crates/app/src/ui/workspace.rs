use std::{collections::BTreeMap, rc::Rc};

use serde::{Serialize, de::DeserializeOwned};

use chrono::{DateTime, Local, Utc};
use gpui::{
    AnyElement, AppContext as _, Context, InteractiveElement as _, IntoElement, ObjectFit,
    ParentElement as _, StatefulInteractiveElement as _, Styled, StyledImage as _, Window, div,
    img, prelude::FluentBuilder as _, px,
};
use gpui_component::{
    Disableable, Icon, IconName, Sizable, Size, StyledExt, WindowExt as _,
    avatar::Avatar,
    button::{Button, ButtonVariants as _},
    checkbox::Checkbox,
    h_flex,
    input::{Input, InputState},
    menu::{ContextMenuExt as _, PopupMenu, PopupMenuItem},
    scroll::ScrollableElement,
    spinner::Spinner,
    text::TextView,
    v_flex,
};
use memos_api::types::{Memo, MemoState, MemoVisibility, Shortcut, UserRole};

use super::{
    DetailTab, MemosDesktop, QuickFilter, Route, SettingsSection, external_attachment_url,
};
use crate::{theme, theme::ThemePreference};

impl MemosDesktop {
    pub(super) fn render_workspace(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let show_inspector = matches!(
            self.route,
            Route::Timeline | Route::Archive | Route::Explore | Route::Profile
        ) && self.selected_memo().is_some();

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

    fn account_context_menu(
        menu: PopupMenu,
        view: gpui::Entity<Self>,
        theme_preference: ThemePreference,
        has_saved_login: bool,
    ) -> PopupMenu {
        let account_view = view.clone();
        let preferences_view = view.clone();
        let system_view = view.clone();
        let light_view = view.clone();
        let dark_view = view.clone();
        let forget_view = view.clone();
        let disconnect_view = view;
        menu.label("Memos Desktop")
            .item(
                PopupMenuItem::new("Account")
                    .icon(IconName::User)
                    .on_click(move |_, _, cx| {
                        account_view.update(cx, |this, cx| {
                            this.settings_section = SettingsSection::Account;
                            this.navigate(Route::Settings, cx);
                        });
                    }),
            )
            .item(
                PopupMenuItem::new("Preferences")
                    .icon(IconName::Settings2)
                    .on_click(move |_, _, cx| {
                        preferences_view.update(cx, |this, cx| {
                            this.settings_section = SettingsSection::Preferences;
                            this.navigate(Route::Settings, cx);
                        });
                    }),
            )
            .separator()
            .item(
                PopupMenuItem::new("Follow system theme")
                    .checked(theme_preference == ThemePreference::System)
                    .on_click(move |_, window, cx| {
                        system_view.update(cx, |this, cx| {
                            this.set_theme_preference(ThemePreference::System, window, cx);
                        });
                    }),
            )
            .item(
                PopupMenuItem::new("Light theme")
                    .icon(IconName::Sun)
                    .checked(theme_preference == ThemePreference::Light)
                    .on_click(move |_, window, cx| {
                        light_view.update(cx, |this, cx| {
                            this.set_theme_preference(ThemePreference::Light, window, cx);
                        });
                    }),
            )
            .item(
                PopupMenuItem::new("Dark theme")
                    .icon(IconName::Moon)
                    .checked(theme_preference == ThemePreference::Dark)
                    .on_click(move |_, window, cx| {
                        dark_view.update(cx, |this, cx| {
                            this.set_theme_preference(ThemePreference::Dark, window, cx);
                        });
                    }),
            )
            .separator()
            .item(
                PopupMenuItem::new("Forget saved login")
                    .icon(IconName::Delete)
                    .disabled(!has_saved_login)
                    .on_click(move |_, _, cx| {
                        forget_view.update(cx, |this, cx| {
                            this.forget_saved_login(cx);
                        });
                    }),
            )
            .item(
                PopupMenuItem::new("Disconnect")
                    .icon(IconName::WindowClose)
                    .on_click(move |_, window, cx| {
                        disconnect_view.update(cx, |this, cx| {
                            this.disconnect(window, cx);
                        });
                    }),
            )
    }

    fn render_nav_rail(&self, cx: &mut Context<Self>) -> AnyElement {
        let current_user = self.current_user.clone();
        let avatar_name = current_user
            .as_ref()
            .and_then(|user| user.display_name.clone())
            .or_else(|| current_user.as_ref().map(|user| user.username.clone()))
            .unwrap_or_else(|| "Guest".into());
        let avatar_path = current_user
            .as_ref()
            .and_then(|user| user.name.as_ref())
            .and_then(|name| self.user_avatars.get(name))
            .cloned();
        let avatar: AnyElement = match avatar_path {
            Some(path) => Avatar::new()
                .with_size(Size::Size(px(32.0)))
                .src(path)
                .into_any_element(),
            None => Avatar::new()
                .with_size(Size::Size(px(32.0)))
                .name(avatar_name)
                .into_any_element(),
        };
        let theme_preference = self.theme_preference;
        let has_saved_login = self.saved_login_available;
        let view = cx.entity().clone();
        let settings_context_view = cx.entity().clone();
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
            .border_color(theme::nav_border())
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
                    .child(
                        div()
                            .child(self.nav_button(Route::Settings, IconName::Settings, cx))
                            .context_menu(move |menu, _, _| {
                                Self::account_context_menu(
                                    menu,
                                    settings_context_view.clone(),
                                    theme_preference,
                                    has_saved_login,
                                )
                            }),
                    )
                    .child(
                        div()
                            .id("account-context")
                            .cursor_pointer()
                            .child(avatar)
                            .context_menu(move |menu, _, _| {
                                Self::account_context_menu(
                                    menu,
                                    view.clone(),
                                    theme_preference,
                                    has_saved_login,
                                )
                            }),
                    ),
            )
            .into_any_element()
    }

    fn nav_button(&self, route: Route, icon: IconName, cx: &mut Context<Self>) -> impl IntoElement {
        let selected = self.route == route;
        let requires_auth = matches!(
            route,
            Route::Views | Route::Archive | Route::Attachments | Route::Inbox | Route::Settings
        );
        let view = cx.entity().clone();
        Button::new(gpui::SharedString::from(format!("nav-{:?}", route)))
            .ghost()
            .large()
            .disabled(requires_auth && self.current_user.is_none())
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
                row.hover(|style| style.bg(theme::hover_surface()))
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
        let module_route = matches!(
            self.route,
            Route::Views | Route::Attachments | Route::Inbox | Route::Settings | Route::Profile
        );
        if self.module_loading && module_route {
            return v_flex()
                .flex_1()
                .h_full()
                .items_center()
                .justify_center()
                .gap_3()
                .child(Spinner::new().large())
                .child(
                    div()
                        .text_xs()
                        .text_color(theme::graphite())
                        .child("Loading from Memos..."),
                )
                .into_any_element();
        }
        let content = match self.route {
            Route::Timeline | Route::Archive | Route::Explore | Route::Profile => {
                self.render_timeline_content(window, cx)
            }
            Route::Views => self.render_views_page(cx),
            Route::Attachments => self.render_attachments_page(cx),
            Route::Inbox => self.render_inbox_page(cx),
            Route::Settings => self.render_settings_page(cx),
        };
        if !module_route {
            return content;
        }

        let error = self.error.clone();
        let notice = self.notice.clone();
        let dismiss_error = cx.entity().clone();
        let dismiss_notice = cx.entity().clone();
        v_flex()
            .flex_1()
            .min_w_0()
            .h_full()
            .when_some(error, |page, error| {
                page.child(
                    h_flex()
                        .px_4()
                        .py_2()
                        .gap_2()
                        .items_center()
                        .border_b_1()
                        .border_color(theme::error_border())
                        .bg(theme::error_background())
                        .text_sm()
                        .text_color(theme::error_text())
                        .child(Icon::new(IconName::TriangleAlert).size_4())
                        .child(div().flex_1().min_w_0().child(error))
                        .child(
                            Button::new("dismiss-module-error")
                                .ghost()
                                .icon(IconName::Close)
                                .tooltip("Dismiss")
                                .on_click(move |_, _, cx| {
                                    dismiss_error.update(cx, |this, cx| {
                                        this.error = None;
                                        cx.notify();
                                    });
                                }),
                        ),
                )
            })
            .when_some(notice, |page, notice| {
                page.child(
                    h_flex()
                        .px_4()
                        .py_2()
                        .gap_2()
                        .items_center()
                        .border_b_1()
                        .border_color(theme::line())
                        .bg(theme::success_background())
                        .text_sm()
                        .text_color(theme::success_text())
                        .child(Icon::new(IconName::CircleCheck).size_4())
                        .child(div().flex_1().min_w_0().child(notice))
                        .child(
                            Button::new("dismiss-module-notice")
                                .ghost()
                                .icon(IconName::Close)
                                .tooltip("Dismiss")
                                .on_click(move |_, _, cx| {
                                    dismiss_notice.update(cx, |this, cx| {
                                        this.notice = None;
                                        cx.notify();
                                    });
                                }),
                        ),
                )
            })
            .child(content)
            .into_any_element()
    }

    fn render_timeline_content(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let memos = self.visible_memos();
        let count = memos.len();
        let title = if self.route == Route::Profile {
            self.profile_user
                .as_ref()
                .and_then(|user| user.display_name.clone())
                .or_else(|| self.profile_user.as_ref().map(|user| user.username.clone()))
                .unwrap_or_else(|| "Profile".into())
        } else {
            self.route.title().to_string()
        };
        let subtitle = match self.route {
            Route::Timeline if self.current_user.is_some() => "Private working stream".to_string(),
            Route::Timeline => "Public memos from this instance".to_string(),
            Route::Archive => "Memos outside the active stream".to_string(),
            Route::Explore => "Visible memos from this instance".to_string(),
            Route::Profile => self
                .profile_user
                .as_ref()
                .map(|user| {
                    let total = self
                        .profile_stats
                        .as_ref()
                        .and_then(|stats| stats.total_memo_count)
                        .unwrap_or(count as i32);
                    let description = user.description.as_deref().unwrap_or("Public activity");
                    format!("@{} · {total} memos · {description}", user.username)
                })
                .unwrap_or_else(|| "Public activity for this user".into()),
            _ => String::new(),
        };
        let profile_avatar = if self.route == Route::Profile {
            self.profile_user.as_ref().map(|user| {
                let name = user.name.as_deref().unwrap_or("users/profile");
                match self.user_avatars.get(name) {
                    Some(path) => Avatar::new()
                        .with_size(Size::Size(px(36.0)))
                        .src(path.clone())
                        .into_any_element(),
                    None => Avatar::new()
                        .with_size(Size::Size(px(36.0)))
                        .name(
                            user.display_name
                                .clone()
                                .unwrap_or_else(|| user.username.clone()),
                        )
                        .into_any_element(),
                }
            })
        } else {
            None
        };
        let is_timeline = self.route == Route::Timeline;
        let can_create = is_timeline && self.current_user.is_some();
        let loading = self.loading;
        let loading_more = self.loading_more;
        let has_more = self.next_memo_page_token.is_some();
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
                        h_flex()
                            .items_center()
                            .gap_3()
                            .when_some(profile_avatar, |header, avatar| header.child(avatar))
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
                        .border_color(theme::error_border())
                        .bg(theme::error_background())
                        .text_sm()
                        .text_color(theme::error_text())
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
                    .when(memos.is_empty() && loading, |list| {
                        list.child(
                            v_flex()
                                .min_h(px(220.0))
                                .items_center()
                                .justify_center()
                                .child(Spinner::new().large()),
                        )
                    })
                    .when(memos.is_empty() && !loading, |list| {
                        list.child(empty_state(
                            IconName::BookOpen,
                            "No memos in this view",
                            "Change the filter or capture a new memo.",
                        ))
                    })
                    .children(memos.into_iter().map(|memo| self.render_memo_row(memo, cx)))
                    .when(has_more, |list| {
                        list.child(
                            h_flex().w_full().justify_center().p_4().child(
                                Button::new("load-more-memos")
                                    .outline()
                                    .label("Load more")
                                    .loading(loading_more)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.load_more_memos(cx);
                                    })),
                            ),
                        )
                    }),
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
                            ))
                            .child(
                                Button::new("transcribe-audio")
                                    .ghost()
                                    .icon(IconName::Bot)
                                    .tooltip("Transcribe audio")
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.transcribe_into_composer(window, cx);
                                    })),
                            ),
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

    fn can_manage_memo(&self, memo: &Memo) -> bool {
        self.current_user.as_ref().is_some_and(|user| {
            user.role == UserRole::Admin || memo.creator.as_deref() == user.name.as_deref()
        })
    }

    fn user_avatar(&self, user_name: &str, size: f32) -> AnyElement {
        let user = self.known_users.get(user_name);
        let label = user
            .and_then(|user| user.display_name.clone())
            .or_else(|| user.map(|user| user.username.clone()))
            .unwrap_or_else(|| resource_id(user_name));
        match self.user_avatars.get(user_name) {
            Some(path) => Avatar::new()
                .with_size(Size::Size(px(size)))
                .src(path.clone())
                .into_any_element(),
            None => Avatar::new()
                .with_size(Size::Size(px(size)))
                .name(label)
                .into_any_element(),
        }
    }

    fn open_image_preview(
        &self,
        attachment: memos_api::types::Attachment,
        path: std::path::PathBuf,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let view = cx.entity().clone();
        let filename = attachment.filename.clone();
        window.open_dialog(cx, move |dialog, _, _| {
            let view = view.clone();
            let attachment = attachment.clone();
            dialog
                .w(px(900.0))
                .h(px(700.0))
                .overlay_closable(true)
                .title(filename.clone())
                .child(
                    div()
                        .flex_1()
                        .min_h_0()
                        .w_full()
                        .overflow_hidden()
                        .bg(theme::subtle_surface())
                        .child(img(path.clone()).size_full().object_fit(ObjectFit::Contain)),
                )
                .footer(move |_, _, _, _| {
                    let view = view.clone();
                    let attachment = attachment.clone();
                    vec![
                        Button::new("open-preview-original")
                            .primary()
                            .icon(IconName::ExternalLink)
                            .label("Open original")
                            .on_click(move |_, window, cx| {
                                window.close_dialog(cx);
                                view.update(cx, |this, cx| {
                                    this.open_attachment_resource(attachment.clone(), cx);
                                });
                            }),
                        Button::new("close-image-preview")
                            .label("Close")
                            .on_click(|_, window, cx| window.close_dialog(cx)),
                    ]
                })
        });
    }

    fn attachment_gallery(
        &self,
        attachments: &[memos_api::types::Attachment],
        max_items: usize,
        tile_height: f32,
        scope: &'static str,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let image_attachments = attachments
            .iter()
            .filter(|attachment| attachment.type_.starts_with("image/"))
            .collect::<Vec<_>>();
        let items = image_attachments
            .iter()
            .filter_map(|attachment| {
                let name = attachment.name.as_ref()?;
                let path = self.attachment_previews.get(name)?.clone();
                Some(((*attachment).clone(), path))
            })
            .take(max_items)
            .collect::<Vec<_>>();
        if items.is_empty() {
            return None;
        }
        let shown_items = items.len();
        let hidden = image_attachments.len().saturating_sub(items.len());
        let view = cx.entity().clone();
        Some(
            div()
                .grid()
                .grid_cols(3)
                .gap_2()
                .children(
                    items
                        .into_iter()
                        .enumerate()
                        .map(move |(ix, (attachment, path))| {
                            let preview_view = view.clone();
                            let preview_attachment = attachment.clone();
                            let preview_path = path.clone();
                            let preview_id = attachment
                                .name
                                .clone()
                                .unwrap_or_else(|| format!("image-{ix}"));
                            div()
                                .id(gpui::SharedString::from(format!(
                                    "{scope}-preview-{preview_id}"
                                )))
                                .relative()
                                .h(px(tile_height))
                                .min_w_0()
                                .overflow_hidden()
                                .rounded(px(4.0))
                                .border_1()
                                .border_color(theme::line())
                                .cursor_pointer()
                                .hover(|style| style.border_color(theme::cobalt()))
                                .on_click(move |_, window, cx| {
                                    cx.stop_propagation();
                                    preview_view.update(cx, |this, cx| {
                                        this.open_image_preview(
                                            preview_attachment.clone(),
                                            preview_path.clone(),
                                            window,
                                            cx,
                                        );
                                    });
                                })
                                .child(img(path).size_full().object_fit(ObjectFit::Cover))
                                .when(ix + 1 == shown_items && hidden > 0, |item| {
                                    item.child(
                                        div()
                                            .absolute()
                                            .inset_0()
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .bg(gpui::black().opacity(0.58))
                                            .text_color(gpui::white())
                                            .text_lg()
                                            .font_semibold()
                                            .child(format!("+{hidden}")),
                                    )
                                })
                        }),
                )
                .into_any_element(),
        )
    }

    fn render_memo_row(&self, memo: Memo, cx: &mut Context<Self>) -> AnyElement {
        let name = memo.name.clone().unwrap_or_else(|| "memos/unknown".into());
        let selected = self.selected_memo_name.as_deref() == Some(name.as_str());
        let view = cx.entity().clone();
        let profile_view = cx.entity().clone();
        let creator = memo.creator.clone();
        let title = memo
            .property
            .as_ref()
            .and_then(|property| property.title.clone());
        let excerpt = memo_excerpt(&memo.content, title.as_deref());
        let visibility = visibility_label(memo.visibility);
        let timestamp = memo.create_time;
        let gallery = self.attachment_gallery(&memo.attachments, 6, 118.0, "timeline", cx);
        let creator_user = creator
            .as_deref()
            .and_then(|name| self.known_users.get(name));
        let creator_label = creator_user
            .and_then(|user| user.display_name.clone())
            .or_else(|| creator_user.map(|user| user.username.clone()));

        h_flex()
            .id(gpui::SharedString::from(format!("memo-{name}")))
            .w_full()
            .border_b_1()
            .border_color(theme::line())
            .cursor_pointer()
            .when(selected, |row| row.bg(theme::pale_cobalt()))
            .when(!selected, |row| {
                row.hover(|style| style.bg(theme::hover_surface()))
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
                            .text_color(theme::ink())
                            .whitespace_normal()
                            .child(excerpt),
                    )
                    .when_some(gallery, |body, gallery| body.child(gallery))
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
                            .when_some(creator, |metadata, creator| {
                                let label = creator_label
                                    .clone()
                                    .unwrap_or_else(|| format!("@{}", resource_id(&creator)));
                                let avatar = self.user_avatar(&creator, 20.0);
                                metadata.child(
                                    h_flex()
                                        .id(gpui::SharedString::from(format!(
                                            "memo-creator-{creator}"
                                        )))
                                        .items_center()
                                        .gap_1()
                                        .cursor_pointer()
                                        .hover(|style| style.text_color(theme::cobalt_dark()))
                                        .on_click(move |_, _, cx| {
                                            cx.stop_propagation();
                                            profile_view.update(cx, |this, cx| {
                                                this.open_user_profile(creator.clone(), cx);
                                            });
                                        })
                                        .child(avatar)
                                        .child(label),
                                )
                            })
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

    fn open_edit_memo_dialog(&self, memo: Memo, window: &mut Window, cx: &mut Context<Self>) {
        let memo_name = memo.name.clone().unwrap_or_default();
        let create_time_value = memo
            .create_time
            .map(|time| time.to_rfc3339())
            .unwrap_or_default();
        let editor = cx.new(|cx| {
            InputState::new(window, cx)
                .multi_line(true)
                .rows(18)
                .default_value(memo.content)
        });
        let create_time = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Creation time (RFC3339)")
                .default_value(create_time_value)
        });
        let view = cx.entity().clone();
        window.open_dialog(cx, move |dialog, _, _| {
            let editor = editor.clone();
            let create_time = create_time.clone();
            let view = view.clone();
            let memo_name = memo_name.clone();
            dialog
                .title("Edit memo")
                .child(
                    v_flex()
                        .gap_3()
                        .child(Input::new(&create_time))
                        .child(Input::new(&editor).h(px(420.0))),
                )
                .footer(move |_, _, _, _| {
                    let editor = editor.clone();
                    let create_time = create_time.clone();
                    let view = view.clone();
                    let memo_name = memo_name.clone();
                    vec![
                        Button::new("save-memo-edit")
                            .primary()
                            .label("Save")
                            .on_click(move |_, window, cx| {
                                let content = editor.read(cx).value().to_string();
                                let create_time = create_time.read(cx).value().trim().to_string();
                                let create_time = if create_time.is_empty() {
                                    None
                                } else {
                                    let time =
                                        match chrono::DateTime::parse_from_rfc3339(&create_time) {
                                            Ok(time) => time,
                                            Err(error) => {
                                                window.close_dialog(cx);
                                                view.update(cx, |this, cx| {
                                                    this.error = Some(format!(
                                                        "Creation time must be RFC3339: {error}"
                                                    ));
                                                    cx.notify();
                                                });
                                                return;
                                            }
                                        };
                                    Some(time.with_timezone(&Utc))
                                };
                                window.close_dialog(cx);
                                view.update(cx, |this, cx| {
                                    this.update_memo_content(
                                        memo_name.clone(),
                                        content,
                                        create_time,
                                        cx,
                                    );
                                });
                            }),
                        Button::new("cancel-memo-edit")
                            .label("Cancel")
                            .on_click(|_, window, cx| window.close_dialog(cx)),
                    ]
                })
        });
    }

    fn open_location_dialog(&self, memo: Memo, window: &mut Window, cx: &mut Context<Self>) {
        let memo_name = memo.name.clone().unwrap_or_default();
        let placeholder = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Location label")
                .default_value(
                    memo.location
                        .as_ref()
                        .and_then(|location| location.placeholder.clone())
                        .unwrap_or_default(),
                )
        });
        let latitude = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Latitude")
                .default_value(
                    memo.location
                        .as_ref()
                        .and_then(|location| location.latitude)
                        .map(|value| value.to_string())
                        .unwrap_or_default(),
                )
        });
        let longitude = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Longitude")
                .default_value(
                    memo.location
                        .as_ref()
                        .and_then(|location| location.longitude)
                        .map(|value| value.to_string())
                        .unwrap_or_default(),
                )
        });
        let view = cx.entity().clone();
        window.open_dialog(cx, move |dialog, _, _| {
            let placeholder = placeholder.clone();
            let latitude = latitude.clone();
            let longitude = longitude.clone();
            let view = view.clone();
            let memo_name = memo_name.clone();
            dialog
                .title("Memo location")
                .child(
                    v_flex()
                        .gap_3()
                        .child(Input::new(&placeholder))
                        .child(Input::new(&latitude))
                        .child(Input::new(&longitude)),
                )
                .footer(move |_, _, _, _| {
                    let placeholder = placeholder.clone();
                    let latitude = latitude.clone();
                    let longitude = longitude.clone();
                    let save_view = view.clone();
                    let remove_view = view.clone();
                    let save_name = memo_name.clone();
                    let remove_name = memo_name.clone();
                    vec![
                        Button::new("save-location")
                            .primary()
                            .label("Save")
                            .on_click(move |_, window, cx| {
                                let latitude = latitude.read(cx).value().parse::<f64>().ok();
                                let longitude = longitude.read(cx).value().parse::<f64>().ok();
                                if latitude.is_some() != longitude.is_some()
                                    || latitude
                                        .is_some_and(|value| !(-90.0..=90.0).contains(&value))
                                    || longitude
                                        .is_some_and(|value| !(-180.0..=180.0).contains(&value))
                                {
                                    window.close_dialog(cx);
                                    save_view.update(cx, |this, cx| {
                                        this.error = Some(
                                            "Latitude must be -90..90 and longitude -180..180."
                                                .into(),
                                        );
                                        cx.notify();
                                    });
                                    return;
                                }
                                let location = memos_api::types::Location {
                                    latitude,
                                    longitude,
                                    placeholder: non_empty_text(placeholder.read(cx).value()),
                                };
                                window.close_dialog(cx);
                                save_view.update(cx, |this, cx| {
                                    this.update_memo_location(
                                        save_name.clone(),
                                        Some(location.clone()),
                                        cx,
                                    );
                                });
                            }),
                        Button::new("remove-location")
                            .danger()
                            .label("Remove")
                            .on_click(move |_, window, cx| {
                                window.close_dialog(cx);
                                remove_view.update(cx, |this, cx| {
                                    this.update_memo_location(remove_name.clone(), None, cx);
                                });
                            }),
                        Button::new("cancel-location")
                            .label("Cancel")
                            .on_click(|_, window, cx| window.close_dialog(cx)),
                    ]
                })
        });
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
        let edit_view = cx.entity().clone();
        let edit_memo = memo.clone();
        let location_view = cx.entity().clone();
        let location_memo = memo.clone();
        let pin_name = name.clone();
        let archive_name = name.clone();
        let delete_name = name.clone();
        let is_archived = memo.state == MemoState::Archived;
        let is_pinned = memo.pinned.unwrap_or(false);
        let can_manage = self.can_manage_memo(&memo);

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
                    .when(can_manage, |panel| {
                        panel.child(
                            h_flex()
                                .gap_1()
                                .pt_3()
                                .border_t_1()
                                .border_color(theme::line())
                                .child(
                                    Button::new("edit-memo")
                                        .small()
                                        .ghost()
                                        .icon(IconName::Replace)
                                        .tooltip("Edit memo")
                                        .on_click(move |_, window, cx| {
                                            edit_view.update(cx, |this, cx| {
                                                this.open_edit_memo_dialog(
                                                    edit_memo.clone(),
                                                    window,
                                                    cx,
                                                );
                                            });
                                        }),
                                )
                                .child(
                                    Button::new("memo-location")
                                        .small()
                                        .ghost()
                                        .icon(IconName::Map)
                                        .tooltip("Edit location")
                                        .on_click(move |_, window, cx| {
                                            location_view.update(cx, |this, cx| {
                                                this.open_location_dialog(
                                                    location_memo.clone(),
                                                    window,
                                                    cx,
                                                );
                                            });
                                        }),
                                )
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
                                                                                confirm_name
                                                                                    .clone(),
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
                        )
                    }),
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
        let detail_loading = self.detail_loading;
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
                panel.child(div().text_xs().text_color(theme::error_text()).child(error))
            })
            .when(detail_loading, |panel| {
                panel.child(
                    v_flex()
                        .flex_1()
                        .items_center()
                        .justify_center()
                        .child(Spinner::new().large()),
                )
            })
            .when(!detail_loading, |panel| {
                panel.child(self.render_detail_body(window, cx, memo, name))
            })
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

    fn memo_visibility_button(
        &self,
        memo_name: String,
        current: MemoVisibility,
        visibility: MemoVisibility,
        icon: IconName,
        label: &'static str,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let enabled = self
            .selected_memo()
            .is_some_and(|memo| self.can_manage_memo(memo));
        let view = cx.entity().clone();
        Button::new(match visibility {
            MemoVisibility::Private => "memo-visibility-private",
            MemoVisibility::Protected => "memo-visibility-protected",
            MemoVisibility::Public => "memo-visibility-public",
            MemoVisibility::VisibilityUnspecified => "memo-visibility-unspecified",
        })
        .xsmall()
        .ghost()
        .icon(icon)
        .tooltip(label)
        .disabled(!enabled)
        .when(current == visibility, |button| button.primary())
        .on_click(move |_, _, cx| {
            view.update(cx, |this, cx| {
                this.update_memo_visibility(memo_name.clone(), visibility, cx);
            });
        })
    }

    fn render_detail_body(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
        memo: Memo,
        name: String,
    ) -> AnyElement {
        let can_manage = self.can_manage_memo(&memo);
        match self.detail_tab {
            DetailTab::Content => {
                let content = memo.content;
                let tasks = markdown_tasks(&content);
                let gallery =
                    self.attachment_gallery(&self.detail.attachments, 9, 92.0, "detail", cx);
                let link_metadata = self.link_metadata.values().cloned().collect::<Vec<_>>();
                let task_view = cx.entity().clone();
                let task_memo_name = name.clone();
                v_flex()
                    .flex_1()
                    .min_h_0()
                    .w_full()
                    .gap_3()
                    .child(
                        h_flex()
                            .gap_1()
                            .child(self.memo_visibility_button(
                                name.clone(),
                                memo.visibility,
                                MemoVisibility::Private,
                                IconName::EyeOff,
                                "Private",
                                cx,
                            ))
                            .child(self.memo_visibility_button(
                                name.clone(),
                                memo.visibility,
                                MemoVisibility::Protected,
                                IconName::User,
                                "Protected",
                                cx,
                            ))
                            .child(self.memo_visibility_button(
                                name.clone(),
                                memo.visibility,
                                MemoVisibility::Public,
                                IconName::Globe,
                                "Public",
                                cx,
                            )),
                    )
                    .when(!tasks.is_empty(), |panel| {
                        panel.child(
                            v_flex()
                                .gap_1()
                                .p_2()
                                .border_1()
                                .border_color(theme::line())
                                .rounded(px(3.0))
                                .children(tasks.into_iter().map(move |task| {
                                    let view = task_view.clone();
                                    let memo_name = task_memo_name.clone();
                                    Checkbox::new(("memo-task", task.line_index))
                                        .checked(task.checked)
                                        .disabled(!can_manage)
                                        .label(task.label)
                                        .on_click(move |_, _, cx| {
                                            view.update(cx, |this, cx| {
                                                this.toggle_memo_task(
                                                    memo_name.clone(),
                                                    task.line_index,
                                                    cx,
                                                );
                                            });
                                        })
                                })),
                        )
                    })
                    .when_some(gallery, |panel, gallery| {
                        panel.child(
                            v_flex()
                                .gap_2()
                                .child(
                                    h_flex()
                                        .items_center()
                                        .justify_between()
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_semibold()
                                                .text_color(theme::graphite())
                                                .child(format!(
                                                    "ATTACHMENTS ({})",
                                                    self.detail.attachments.len()
                                                )),
                                        )
                                        .child(
                                            Button::new("open-files-tab")
                                                .xsmall()
                                                .ghost()
                                                .label("View all")
                                                .on_click(cx.listener(|this, _, _, cx| {
                                                    this.set_detail_tab(DetailTab::Files, cx);
                                                })),
                                        ),
                                )
                                .child(gallery),
                        )
                    })
                    .when(!link_metadata.is_empty(), |panel| {
                        panel.child(
                            v_flex().gap_2().children(
                                link_metadata.into_iter().enumerate().filter_map(
                                    |(ix, metadata)| {
                                        let url = metadata.url?;
                                        let open_url = url.clone();
                                        Some(
                                            h_flex()
                                                .gap_2()
                                                .p_2()
                                                .border_1()
                                                .border_color(theme::line())
                                                .rounded(px(3.0))
                                                .child(
                                                    v_flex()
                                                        .flex_1()
                                                        .min_w_0()
                                                        .gap_1()
                                                        .child(
                                                            div().text_sm().font_semibold().child(
                                                                metadata
                                                                    .title
                                                                    .unwrap_or_else(|| url.clone()),
                                                            ),
                                                        )
                                                        .when_some(
                                                            metadata.description,
                                                            |content, description| {
                                                                content.child(
                                                                    div()
                                                                        .text_xs()
                                                                        .text_color(
                                                                            theme::graphite(),
                                                                        )
                                                                        .child(description),
                                                                )
                                                            },
                                                        ),
                                                )
                                                .child(
                                                    Button::new(("open-link-preview", ix))
                                                        .ghost()
                                                        .icon(IconName::ExternalLink)
                                                        .tooltip("Open link")
                                                        .on_click(move |_, _, cx| {
                                                            cx.open_url(&open_url)
                                                        }),
                                                ),
                                        )
                                    },
                                ),
                            ),
                        )
                    })
                    .child(
                        div().flex_1().min_h_0().w_full().child(
                            TextView::markdown(
                                gpui::SharedString::from(format!("inspector-{name}")),
                                content,
                                window,
                                cx,
                            )
                            .selectable(true)
                            .scrollable(true)
                            .w_full()
                            .h_full(),
                        ),
                    )
                    .into_any_element()
            }
            DetailTab::Activity => self.render_activity_panel(cx),
            DetailTab::Links => self.render_links_panel(window, cx, can_manage),
            DetailTab::Share => self.render_share_panel(cx, can_manage),
            DetailTab::Files => self.render_files_panel(window, cx, can_manage),
        }
    }

    fn render_activity_panel(&self, cx: &mut Context<Self>) -> AnyElement {
        let reactions = reaction_counts(&self.detail.reactions);
        let current_user_name = self
            .current_user
            .as_ref()
            .and_then(|user| user.name.clone());
        let can_interact = current_user_name.is_some();
        let is_admin = self
            .current_user
            .as_ref()
            .is_some_and(|user| user.role == UserRole::Admin);
        let own_reactions = self
            .detail
            .reactions
            .iter()
            .filter(|reaction| reaction.creator == current_user_name)
            .cloned()
            .collect::<Vec<_>>();
        let comments = self.detail.comments.clone();
        let comments_empty = comments.is_empty();
        let comment_view = cx.entity().clone();
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
                    .children(
                        own_reactions
                            .into_iter()
                            .enumerate()
                            .filter_map(|(ix, reaction)| {
                                let reaction_name = reaction.name?;
                                let view = cx.entity().clone();
                                Some(
                                    Button::new(("remove-reaction", ix))
                                        .xsmall()
                                        .ghost()
                                        .icon(IconName::Close)
                                        .tooltip("Remove my reaction")
                                        .on_click(move |_, _, cx| {
                                            view.update(cx, |this, cx| {
                                                this.remove_reaction(reaction_name.clone(), cx);
                                            });
                                        }),
                                )
                            }),
                    )
                    .child(
                        Button::new("add-reaction")
                            .xsmall()
                            .ghost()
                            .disabled(!can_interact)
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
                    .when(comments_empty, |list| {
                        list.child(
                            div()
                                .text_xs()
                                .text_color(theme::graphite())
                                .child("No comments yet."),
                        )
                    })
                    .children(comments.into_iter().enumerate().map(move |(ix, comment)| {
                        let can_delete = is_admin || comment.creator == current_user_name;
                        let comment_name = comment.name.clone().unwrap_or_default();
                        let view = comment_view.clone();
                        v_flex()
                            .gap_1()
                            .pb_3()
                            .border_b_1()
                            .border_color(theme::line())
                            .child(
                                h_flex()
                                    .items_center()
                                    .justify_between()
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
                                    .when(can_delete, |row| {
                                        row.child(
                                            Button::new(("delete-comment", ix))
                                                .xsmall()
                                                .danger()
                                                .icon(IconName::Delete)
                                                .tooltip("Delete comment")
                                                .on_click(move |_, _, cx| {
                                                    view.update(cx, |this, cx| {
                                                        this.delete_comment(
                                                            comment_name.clone(),
                                                            cx,
                                                        );
                                                    });
                                                }),
                                        )
                                    }),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .line_height(px(20.0))
                                    .child(memo_excerpt(&comment.content, None)),
                            )
                    })),
            )
            .when(can_interact, |panel| {
                panel.child(
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
            })
            .into_any_element()
    }

    fn open_add_relation_dialog(&self, window: &mut Window, cx: &mut Context<Self>) {
        let input = cx.new(|cx| InputState::new(window, cx).placeholder("Memo ID or memos/{id}"));
        let view = cx.entity().clone();
        window.open_dialog(cx, move |dialog, _, _| {
            let input = input.clone();
            let view = view.clone();
            dialog
                .title("Add memo reference")
                .child(Input::new(&input))
                .footer(move |_, _, _, _| {
                    let input = input.clone();
                    let view = view.clone();
                    vec![
                        Button::new("confirm-add-relation")
                            .primary()
                            .label("Add")
                            .on_click(move |_, window, cx| {
                                let related = input.read(cx).value().trim().to_string();
                                if related.is_empty() {
                                    return;
                                }
                                window.close_dialog(cx);
                                view.update(cx, |this, cx| {
                                    this.add_relation(related.clone(), cx);
                                });
                            }),
                        Button::new("cancel-add-relation")
                            .label("Cancel")
                            .on_click(|_, window, cx| window.close_dialog(cx)),
                    ]
                })
        });
    }

    fn render_links_panel(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
        can_manage: bool,
    ) -> AnyElement {
        let relations = self.detail.relations.clone();
        let relations_empty = relations.is_empty();
        let add_view = cx.entity().clone();
        let relation_view = cx.entity().clone();
        v_flex()
            .flex_1()
            .min_h_0()
            .gap_2()
            .child(
                h_flex().justify_end().child(
                    Button::new("add-relation")
                        .small()
                        .primary()
                        .disabled(!can_manage)
                        .icon(IconName::Plus)
                        .tooltip("Add reference")
                        .on_click(move |_, window, cx| {
                            add_view.update(cx, |this, cx| {
                                this.open_add_relation_dialog(window, cx);
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
                    .when(relations_empty, |list| {
                        list.child(
                            div()
                                .text_xs()
                                .text_color(theme::graphite())
                                .child("No references or backlinks."),
                        )
                    })
                    .children(
                        relations
                            .into_iter()
                            .enumerate()
                            .map(move |(ix, relation)| {
                                let relation_type = format!("{:?}", relation.type_).to_lowercase();
                                let view = relation_view.clone();
                                let related_name = relation.related_memo.name.clone();
                                h_flex()
                                    .items_center()
                                    .gap_2()
                                    .p_2()
                                    .border_1()
                                    .border_color(theme::line())
                                    .rounded(px(3.0))
                                    .child(
                                        v_flex()
                                            .flex_1()
                                            .min_w_0()
                                            .gap_1()
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
                                                        .unwrap_or(related_name),
                                                ),
                                            ),
                                    )
                                    .child(
                                        Button::new(("remove-relation", ix))
                                            .xsmall()
                                            .danger()
                                            .disabled(!can_manage)
                                            .icon(IconName::Delete)
                                            .tooltip("Remove relation")
                                            .on_click(move |_, _, cx| {
                                                view.update(cx, |this, cx| {
                                                    this.remove_relation(ix, cx);
                                                });
                                            }),
                                    )
                            }),
                    ),
            )
            .into_any_element()
    }

    fn open_create_share_dialog(&self, window: &mut Window, cx: &mut Context<Self>) {
        let view = cx.entity().clone();
        window.open_dialog(cx, move |dialog, _, _| {
            let view = view.clone();
            dialog
                .title("Create share link")
                .child("Choose when this read-only link should expire.")
                .footer(move |_, _, _, _| {
                    [
                        ("share-never", "Never", None),
                        ("share-seven-days", "7 days", Some(24 * 7)),
                        ("share-thirty-days", "30 days", Some(24 * 30)),
                    ]
                    .into_iter()
                    .map(|(id, label, ttl)| {
                        let view = view.clone();
                        Button::new(id)
                            .when(ttl.is_none(), |button| button.primary())
                            .label(label)
                            .on_click(move |_, window, cx| {
                                window.close_dialog(cx);
                                view.update(cx, |this, cx| {
                                    this.create_share_with_ttl(ttl, cx);
                                });
                            })
                    })
                    .collect()
                })
        });
    }

    fn render_share_panel(&self, cx: &mut Context<Self>, can_manage: bool) -> AnyElement {
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
                            .disabled(!can_manage)
                            .icon(IconName::Plus)
                            .tooltip("Create share link")
                            .on_click(move |_, window, cx| {
                                view.update(cx, |this, cx| {
                                    this.open_create_share_dialog(window, cx);
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
                                    .disabled(!can_manage)
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

    fn open_external_attachment_dialog(&self, window: &mut Window, cx: &mut Context<Self>) {
        let filename = cx.new(|cx| InputState::new(window, cx).placeholder("Display filename"));
        let url = cx.new(|cx| InputState::new(window, cx).placeholder("https://example.com/file"));
        let mime = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("MIME type")
                .default_value("application/octet-stream")
        });
        let view = cx.entity().clone();
        window.open_dialog(cx, move |dialog, _, _| {
            let filename = filename.clone();
            let url = url.clone();
            let mime = mime.clone();
            let view = view.clone();
            dialog
                .title("Attach external link")
                .child(
                    v_flex()
                        .gap_3()
                        .child(Input::new(&filename))
                        .child(Input::new(&url))
                        .child(Input::new(&mime)),
                )
                .footer(move |_, _, _, _| {
                    let filename = filename.clone();
                    let url = url.clone();
                    let mime = mime.clone();
                    let view = view.clone();
                    vec![
                        Button::new("confirm-external-attachment")
                            .primary()
                            .label("Attach")
                            .on_click(move |_, window, cx| {
                                let filename = filename.read(cx).value().trim().to_string();
                                let url = url.read(cx).value().trim().to_string();
                                let mime = mime.read(cx).value().trim().to_string();
                                let valid_url = url::Url::parse(&url).is_ok_and(|url| {
                                    matches!(url.scheme(), "http" | "https") && url.host().is_some()
                                });
                                if filename.is_empty()
                                    || !valid_url
                                    || mime.is_empty()
                                    || mime.contains(char::is_whitespace)
                                {
                                    window.close_dialog(cx);
                                    view.update(cx, |this, cx| {
                                        this.detail_error = Some(
                                            "Enter a filename, an HTTP(S) URL, and a valid MIME type."
                                                .into(),
                                        );
                                        cx.notify();
                                    });
                                    return;
                                }
                                window.close_dialog(cx);
                                view.update(cx, |this, cx| {
                                    this.attach_external_link(
                                        filename.clone(),
                                        url.clone(),
                                        mime.clone(),
                                        cx,
                                    );
                                });
                            }),
                        Button::new("cancel-external-attachment")
                            .label("Cancel")
                            .on_click(|_, window, cx| window.close_dialog(cx)),
                    ]
                })
        });
    }

    fn render_files_panel(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
        can_manage: bool,
    ) -> AnyElement {
        let attachments = self.detail.attachments.clone();
        let attachments_empty = attachments.is_empty();
        let upload_view = cx.entity().clone();
        let external_view = cx.entity().clone();
        let attachment_view = cx.entity().clone();
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
                        h_flex()
                            .gap_1()
                            .child(
                                Button::new("attach-external")
                                    .small()
                                    .ghost()
                                    .icon(IconName::ExternalLink)
                                    .tooltip("Attach external link")
                                    .disabled(!can_manage)
                                    .on_click(move |_, window, cx| {
                                        external_view.update(cx, |this, cx| {
                                            this.open_external_attachment_dialog(window, cx);
                                        });
                                    }),
                            )
                            .child(
                                Button::new("upload-attachment")
                                    .small()
                                    .primary()
                                    .icon(IconName::Plus)
                                    .tooltip("Upload attachments")
                                    .disabled(!can_manage)
                                    .on_click(move |_, window, cx| {
                                        upload_view.update(cx, |this, cx| {
                                            this.upload_attachment(window, cx);
                                        });
                                    }),
                            ),
                    ),
            )
            .child(
                v_flex()
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scrollbar()
                    .gap_2()
                    .when(attachments_empty, |list| {
                        list.child(
                            div()
                                .text_xs()
                                .text_color(theme::graphite())
                                .child("No attachments on this memo."),
                        )
                    })
                    .children(
                        attachments
                            .into_iter()
                            .enumerate()
                            .map(move |(ix, attachment)| {
                                let open_view = attachment_view.clone();
                                let edit_view = attachment_view.clone();
                                let delete_view = attachment_view.clone();
                                let open_attachment = attachment.clone();
                                let edit_attachment = attachment.clone();
                                let name = attachment.name.clone().unwrap_or_default();
                                let metadata = attachment_metadata(&attachment);
                                let preview = self.attachment_previews.get(&name).cloned();
                                let has_preview = preview.is_some();
                                let can_open = external_attachment_url(&attachment).is_some()
                                    || attachment.name.is_some();
                                h_flex()
                                    .gap_2()
                                    .items_center()
                                    .p_2()
                                    .border_1()
                                    .border_color(theme::line())
                                    .rounded(px(3.0))
                                    .when_some(preview, |row, path| {
                                        row.child(
                                            div()
                                                .size(px(40.0))
                                                .flex_shrink_0()
                                                .overflow_hidden()
                                                .rounded(px(3.0))
                                                .child(
                                                    img(path)
                                                        .size_full()
                                                        .object_fit(ObjectFit::Cover),
                                                ),
                                        )
                                    })
                                    .when(!has_preview, |row| {
                                        row.child(Icon::new(IconName::File).size_4())
                                    })
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
                                                    .child(metadata),
                                            ),
                                    )
                                    .when(can_open, |row| {
                                        row.child(
                                            Button::new(("open-attachment", ix))
                                                .xsmall()
                                                .ghost()
                                                .icon(IconName::ExternalLink)
                                                .tooltip("Open attachment")
                                                .on_click(move |_, _, cx| {
                                                    open_view.update(cx, |this, cx| {
                                                        this.open_attachment_resource(
                                                            open_attachment.clone(),
                                                            cx,
                                                        );
                                                    });
                                                }),
                                        )
                                    })
                                    .child(
                                        Button::new(("edit-attachment", ix))
                                            .xsmall()
                                            .ghost()
                                            .disabled(!can_manage)
                                            .icon(IconName::Replace)
                                            .tooltip("Edit attachment JSON")
                                            .on_click(move |_, window, cx| {
                                                edit_view.update(cx, |this, cx| {
                                                    this.open_json_editor(
                                                        "Edit attachment",
                                                        edit_attachment.clone(),
                                                        window,
                                                        cx,
                                                        |this, attachment, cx| {
                                                            this.save_attachment_resource(
                                                                attachment, cx,
                                                            );
                                                        },
                                                    );
                                                });
                                            }),
                                    )
                                    .child(
                                        Button::new(("delete-attachment", ix))
                                            .xsmall()
                                            .danger()
                                            .disabled(!can_manage)
                                            .icon(IconName::Delete)
                                            .tooltip("Delete attachment")
                                            .on_click(move |_, _, cx| {
                                                delete_view.update(cx, |this, cx| {
                                                    this.delete_attachment_resource(
                                                        name.clone(),
                                                        cx,
                                                    );
                                                });
                                            }),
                                    )
                            }),
                    ),
            )
            .into_any_element()
    }

    fn open_memo_view_dialog(
        &self,
        existing: Option<Shortcut>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let name = existing.as_ref().and_then(|view| view.name.clone());
        let title = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("View title")
                .default_value(
                    existing
                        .as_ref()
                        .map(|view| view.title.clone())
                        .unwrap_or_default(),
                )
        });
        let filter = cx.new(|cx| {
            InputState::new(window, cx)
                .multi_line(true)
                .rows(6)
                .placeholder("CEL filter expression")
                .default_value(
                    existing
                        .as_ref()
                        .and_then(|view| view.filter.clone())
                        .unwrap_or_default(),
                )
        });
        let view = cx.entity().clone();
        window.open_dialog(cx, move |dialog, _, _| {
            let title = title.clone();
            let filter = filter.clone();
            let view = view.clone();
            let name = name.clone();
            dialog
                .title(if name.is_some() {
                    "Edit view"
                } else {
                    "Create view"
                })
                .child(
                    v_flex()
                        .gap_3()
                        .child(Input::new(&title))
                        .child(Input::new(&filter).h(px(140.0))),
                )
                .footer(move |_, _, _, _| {
                    let title = title.clone();
                    let filter = filter.clone();
                    let view = view.clone();
                    let name = name.clone();
                    vec![
                        Button::new("save-memo-view")
                            .primary()
                            .label("Save")
                            .on_click(move |_, window, cx| {
                                let title = title.read(cx).value().trim().to_string();
                                let filter = filter.read(cx).value().trim().to_string();
                                if title.is_empty() || filter.is_empty() {
                                    return;
                                }
                                window.close_dialog(cx);
                                view.update(cx, |this, cx| {
                                    this.save_memo_view(
                                        Shortcut {
                                            filter: Some(filter.clone()),
                                            name: name.clone(),
                                            title: title.clone(),
                                        },
                                        cx,
                                    );
                                });
                            }),
                        Button::new("cancel-memo-view")
                            .label("Cancel")
                            .on_click(|_, window, cx| window.close_dialog(cx)),
                    ]
                })
        });
    }

    fn render_views_page(&self, cx: &mut Context<Self>) -> AnyElement {
        let quick_view_entity = cx.entity().clone();
        let create_view_entity = cx.entity().clone();
        let server_view_entity = cx.entity().clone();
        let server_views = self.memo_views.clone();
        let server_views_empty = server_views.is_empty();
        let server_view_rows = server_views
            .into_iter()
            .enumerate()
            .map(move |(ix, memo_view)| {
                let open_entity = server_view_entity.clone();
                let edit_entity = server_view_entity.clone();
                let delete_entity = server_view_entity.clone();
                let filter = memo_view.filter.clone().unwrap_or_default();
                let edit_view = memo_view.clone();
                let name = memo_view.name.clone().unwrap_or_default();
                h_flex()
                    .min_h(px(64.0))
                    .px_4()
                    .items_center()
                    .justify_between()
                    .border_b_1()
                    .border_color(theme::line())
                    .child(
                        v_flex()
                            .flex_1()
                            .min_w_0()
                            .gap_1()
                            .child(div().text_sm().font_semibold().child(memo_view.title))
                            .child(
                                div()
                                    .font_family(theme::mono_family())
                                    .text_xs()
                                    .text_color(theme::graphite())
                                    .overflow_hidden()
                                    .text_ellipsis()
                                    .child(filter.clone()),
                            ),
                    )
                    .child(
                        h_flex()
                            .items_center()
                            .gap_1()
                            .child(
                                Button::new(("edit-server-view", ix))
                                    .ghost()
                                    .icon(IconName::Replace)
                                    .tooltip("Edit view")
                                    .on_click(move |_, window, cx| {
                                        edit_entity.update(cx, |this, cx| {
                                            this.open_memo_view_dialog(
                                                Some(edit_view.clone()),
                                                window,
                                                cx,
                                            );
                                        });
                                    }),
                            )
                            .child(
                                Button::new(("delete-server-view", ix))
                                    .danger()
                                    .icon(IconName::Delete)
                                    .tooltip("Delete view")
                                    .on_click(move |_, _, cx| {
                                        delete_entity.update(cx, |this, cx| {
                                            this.delete_memo_view_resource(name.clone(), cx);
                                        });
                                    }),
                            )
                            .child(
                                Button::new(("open-server-view", ix))
                                    .primary()
                                    .icon(IconName::ArrowRight)
                                    .tooltip("Open view")
                                    .on_click(move |_, _, cx| {
                                        open_entity.update(cx, |this, cx| {
                                            this.open_memo_view(filter.clone(), cx);
                                        });
                                    }),
                            ),
                    )
            });
        module_page(
            "Saved views",
            "Reusable filters",
            v_flex()
                .gap_4()
                .child(
                    h_flex().justify_end().child(
                        Button::new("create-memo-view")
                            .primary()
                            .icon(IconName::Plus)
                            .label("New view")
                            .disabled(self.current_user.is_none())
                            .on_click(move |_, window, cx| {
                                create_view_entity.update(cx, |this, cx| {
                                    this.open_memo_view_dialog(None, window, cx);
                                });
                            }),
                    ),
                )
                .child(
                    v_flex()
                        .border_1()
                        .border_color(theme::line())
                        .child(panel_label("SERVER VIEWS"))
                        .when(server_views_empty, |panel| {
                            panel.child(
                                div()
                                    .p_4()
                                    .text_xs()
                                    .text_color(theme::graphite())
                                    .child("No saved server views."),
                            )
                        })
                        .children(server_view_rows),
                )
                .child(
                    v_flex()
                        .border_1()
                        .border_color(theme::line())
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
                            .map(
                                move |(title, filter, quick_filter)| {
                                    let view = quick_view_entity.clone();
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
                                            Button::new(gpui::SharedString::from(format!(
                                                "open-view-{title}"
                                            )))
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
                                },
                            ),
                        ),
                ),
        )
    }

    fn render_attachments_page(&self, cx: &mut Context<Self>) -> AnyElement {
        let attachments = if self.library_attachments.is_empty() {
            self.memos
                .iter()
                .flat_map(|memo| memo.attachments.iter().cloned())
                .collect::<Vec<_>>()
        } else {
            self.library_attachments.clone()
        };
        let attachments_empty = attachments.is_empty();
        let media_count = attachments
            .iter()
            .filter(|attachment| attachment.type_.starts_with("image/"))
            .count();
        let media_gallery = self.attachment_gallery(&attachments, 30, 148.0, "library", cx);
        let file_attachments = attachments.into_iter().collect::<Vec<_>>();
        let has_more = self.next_attachment_page_token.is_some();
        let loading_more = self.loading_more;
        let upload_view = cx.entity().clone();
        let load_view = cx.entity().clone();
        let attachment_view = cx.entity().clone();
        let content = v_flex()
            .gap_3()
            .child(
                h_flex().justify_end().child(
                    Button::new("upload-library-attachments")
                        .primary()
                        .icon(IconName::Plus)
                        .label("Upload files")
                        .disabled(self.current_user.is_none())
                        .on_click(move |_, window, cx| {
                            upload_view.update(cx, |this, cx| {
                                this.upload_library_attachments(window, cx);
                            });
                        }),
                ),
            )
            .when_some(media_gallery, |content, gallery| {
                content.child(
                    v_flex()
                        .gap_2()
                        .child(
                            h_flex()
                                .items_center()
                                .justify_between()
                                .child(panel_label("MEDIA"))
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme::graphite())
                                        .child(format!("{media_count} images")),
                                ),
                        )
                        .child(gallery),
                )
            })
            .child(
                v_flex()
                    .border_1()
                    .border_color(theme::line())
                    .child(panel_label("ALL ATTACHMENTS"))
                    .when(attachments_empty, |list| {
                        list.child(empty_state(
                            IconName::File,
                            "No attachments",
                            "Uploaded files will appear here.",
                        ))
                    })
                    .when(file_attachments.is_empty() && !attachments_empty, |list| {
                        list.child(
                            div()
                                .p_4()
                                .text_xs()
                                .text_color(theme::graphite())
                                .child("Images and files are listed here for management."),
                        )
                    })
                    .children(file_attachments.into_iter().enumerate().map(
                        move |(ix, attachment)| {
                            let open_view = attachment_view.clone();
                            let edit_view = attachment_view.clone();
                            let delete_view = attachment_view.clone();
                            let open_attachment = attachment.clone();
                            let edit_attachment = attachment.clone();
                            let name = attachment.name.clone().unwrap_or_default();
                            let metadata = attachment_metadata(&attachment);
                            let preview = self.attachment_previews.get(&name).cloned();
                            let has_preview = preview.is_some();
                            let can_open = external_attachment_url(&attachment).is_some()
                                || attachment.name.is_some();
                            h_flex()
                                .min_h(px(58.0))
                                .px_4()
                                .items_center()
                                .justify_between()
                                .border_b_1()
                                .border_color(theme::line())
                                .child(
                                    h_flex()
                                        .flex_1()
                                        .min_w_0()
                                        .gap_3()
                                        .items_center()
                                        .when_some(preview, |row, path| {
                                            row.child(
                                                div()
                                                    .size(px(44.0))
                                                    .flex_shrink_0()
                                                    .overflow_hidden()
                                                    .rounded(px(3.0))
                                                    .child(
                                                        img(path)
                                                            .size_full()
                                                            .object_fit(ObjectFit::Cover),
                                                    ),
                                            )
                                        })
                                        .when(!has_preview, |row| {
                                            row.child(Icon::new(IconName::File).size_4())
                                        })
                                        .child(
                                            v_flex()
                                                .min_w_0()
                                                .gap_1()
                                                .child(
                                                    div()
                                                        .text_sm()
                                                        .overflow_hidden()
                                                        .text_ellipsis()
                                                        .child(attachment.filename),
                                                )
                                                .child(
                                                    div()
                                                        .font_family(theme::mono_family())
                                                        .text_xs()
                                                        .text_color(theme::graphite())
                                                        .child(metadata),
                                                ),
                                        ),
                                )
                                .child(
                                    h_flex()
                                        .gap_1()
                                        .when(can_open, |actions| {
                                            actions.child(
                                                Button::new(("open-library-attachment", ix))
                                                    .ghost()
                                                    .icon(IconName::ExternalLink)
                                                    .tooltip("Open attachment")
                                                    .on_click(move |_, _, cx| {
                                                        open_view.update(cx, |this, cx| {
                                                            this.open_attachment_resource(
                                                                open_attachment.clone(),
                                                                cx,
                                                            );
                                                        });
                                                    }),
                                            )
                                        })
                                        .child(
                                            Button::new(("edit-library-attachment", ix))
                                                .ghost()
                                                .icon(IconName::Replace)
                                                .tooltip("Edit attachment JSON")
                                                .on_click(move |_, window, cx| {
                                                    edit_view.update(cx, |this, cx| {
                                                        this.open_json_editor(
                                                            "Edit attachment",
                                                            edit_attachment.clone(),
                                                            window,
                                                            cx,
                                                            |this, attachment, cx| {
                                                                this.save_attachment_resource(
                                                                    attachment, cx,
                                                                );
                                                            },
                                                        );
                                                    });
                                                }),
                                        )
                                        .child(
                                            Button::new(("delete-library-attachment", ix))
                                                .danger()
                                                .icon(IconName::Delete)
                                                .tooltip("Delete attachment")
                                                .on_click(move |_, _, cx| {
                                                    delete_view.update(cx, |this, cx| {
                                                        this.delete_attachment_resource(
                                                            name.clone(),
                                                            cx,
                                                        );
                                                    });
                                                }),
                                        ),
                                )
                        },
                    )),
            )
            .when(has_more, |content| {
                content.child(
                    h_flex().justify_center().child(
                        Button::new("load-more-attachments")
                            .outline()
                            .label("Load more")
                            .loading(loading_more)
                            .on_click(move |_, _, cx| {
                                load_view.update(cx, |this, cx| {
                                    this.load_more_attachments(cx);
                                });
                            }),
                    ),
                )
            });
        module_page("Attachments", "Instance file library", content)
    }

    fn render_inbox_page(&self, cx: &mut Context<Self>) -> AnyElement {
        let notifications = self.notifications.clone();
        let notifications_empty = notifications.is_empty();
        let has_more = self.next_notification_page_token.is_some();
        let loading_more = self.loading_more;
        let notification_view = cx.entity().clone();
        let load_view = cx.entity().clone();
        let content = v_flex()
            .border_1()
            .border_color(theme::line())
            .when(notifications_empty, |list| {
                list.child(empty_state(
                    IconName::Bell,
                    "Inbox is clear",
                    "New mentions and comments will appear here.",
                ))
            })
            .children(
                notifications
                    .into_iter()
                    .enumerate()
                    .map(move |(ix, notification)| {
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
                        let snippet = notification
                            .memo_comment
                            .as_ref()
                            .and_then(|payload| payload.memo_snippet.clone())
                            .or_else(|| {
                                notification
                                    .memo_mention
                                    .as_ref()
                                    .and_then(|payload| payload.memo_snippet.clone())
                            })
                            .unwrap_or_else(|| "No preview available".into());
                        let target_memo = notification
                            .memo_comment
                            .as_ref()
                            .and_then(|payload| {
                                payload
                                    .related_memo
                                    .clone()
                                    .or_else(|| payload.memo.clone())
                            })
                            .or_else(|| {
                                notification.memo_mention.as_ref().and_then(|payload| {
                                    payload
                                        .related_memo
                                        .clone()
                                        .or_else(|| payload.memo.clone())
                                })
                            });
                        let archived = notification.status
                            == Some(memos_api::types::UserNotificationStatus::Archived);
                        let open_view = notification_view.clone();
                        let status_view = notification_view.clone();
                        let delete_view = notification_view.clone();
                        let status_name = name.clone();
                        let delete_name = name.clone();
                        h_flex()
                    .min_h(px(72.0))
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
                                    .flex_1()
                                    .min_w_0()
                                    .gap_1()
                                    .child(div().text_sm().font_semibold().child(kind))
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme::graphite())
                                            .child(format!("from {sender}")),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .overflow_hidden()
                                            .text_ellipsis()
                                            .child(snippet),
                                    ),
                            ),
                    )
                    .child(
                        h_flex()
                            .gap_1()
                            .when_some(target_memo, |actions, memo_name| {
                                actions.child(
                                    Button::new(("open-notification", ix))
                                        .ghost()
                                        .icon(IconName::ArrowRight)
                                        .tooltip("Open related memo")
                                        .on_click(move |_, _, cx| {
                                            open_view.update(cx, |this, cx| {
                                                this.open_notification_memo(
                                                    memo_name.clone(),
                                                    cx,
                                                );
                                            });
                                        }),
                                )
                            })
                            .child(
                                Button::new(("archive-notification", ix))
                                    .ghost()
                                    .icon(if archived {
                                        IconName::Inbox
                                    } else {
                                        IconName::FolderClosed
                                    })
                                    .tooltip(if archived { "Mark unread" } else { "Archive" })
                                    .on_click(move |_, _, cx| {
                                        let status = if archived {
                                            memos_api::types::UserNotificationStatus::Unread
                                        } else {
                                            memos_api::types::UserNotificationStatus::Archived
                                        };
                                        status_view.update(cx, |this, cx| {
                                            this.set_notification_status(
                                                status_name.clone(),
                                                status,
                                                cx,
                                            );
                                        });
                                    }),
                            )
                            .child(
                                Button::new(("delete-notification", ix))
                                    .danger()
                                    .icon(IconName::Delete)
                                    .tooltip("Delete notification")
                                    .on_click(move |_, _, cx| {
                                        delete_view.update(cx, |this, cx| {
                                            this.delete_notification_resource(
                                                delete_name.clone(),
                                                cx,
                                            );
                                        });
                                    }),
                            ),
                    )
                    }),
            )
            .when(has_more, |content| {
                content.child(
                    h_flex().justify_center().p_3().child(
                        Button::new("load-more-notifications")
                            .outline()
                            .label("Load more")
                            .loading(loading_more)
                            .on_click(move |_, _, cx| {
                                load_view.update(cx, |this, cx| {
                                    this.load_more_notifications(cx);
                                });
                            }),
                    ),
                )
            });
        module_page("Inbox", "Mentions and comments", content)
    }

    fn open_json_editor<T, F>(
        &self,
        title: &'static str,
        value: T,
        window: &mut Window,
        cx: &mut Context<Self>,
        on_save: F,
    ) where
        T: Serialize + DeserializeOwned + Clone + 'static,
        F: Fn(&mut MemosDesktop, T, &mut Context<MemosDesktop>) + 'static,
    {
        let source = serde_json::to_string_pretty(&value).unwrap_or_default();
        let editor = cx.new(|cx| {
            InputState::new(window, cx)
                .code_editor("json")
                .multi_line(true)
                .rows(22)
                .default_value(source)
        });
        let view = cx.entity().clone();
        let on_save = Rc::new(on_save);
        window.open_dialog(cx, move |dialog, _, _| {
            let editor = editor.clone();
            let view = view.clone();
            let on_save = on_save.clone();
            dialog
                .title(title)
                .child(Input::new(&editor).h(px(520.0)))
                .footer(move |_, _, _, _| {
                    let editor = editor.clone();
                    let view = view.clone();
                    let on_save = on_save.clone();
                    vec![
                        Button::new("save-json-resource")
                            .primary()
                            .label("Save")
                            .on_click(move |_, window, cx| {
                                let source = editor.read(cx).value().to_string();
                                match serde_json::from_str::<T>(&source) {
                                    Ok(value) => {
                                        window.close_dialog(cx);
                                        view.update(cx, |this, cx| on_save(this, value, cx));
                                    }
                                    Err(error) => {
                                        view.update(cx, |this, cx| {
                                            this.error =
                                                Some(format!("Invalid JSON for {title}: {error}"));
                                            cx.notify();
                                        });
                                    }
                                }
                            }),
                        Button::new("cancel-json-resource")
                            .label("Cancel")
                            .on_click(|_, window, cx| window.close_dialog(cx)),
                    ]
                })
        });
    }

    pub(super) fn show_link_provider_dialog(
        &self,
        providers: Vec<memos_api::types::IdentityProvider>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let view = cx.entity().clone();
        window.open_dialog(cx, move |dialog, _, _| {
            let view = view.clone();
            dialog.title("Link SSO identity").child(
                v_flex()
                    .gap_2()
                    .children(providers.clone().into_iter().enumerate().map(
                        move |(ix, provider)| {
                            let view = view.clone();
                            let label = provider.title.clone();
                            Button::new(("link-sso-provider", ix))
                                .large()
                                .outline()
                                .icon(IconName::ExternalLink)
                                .label(label)
                                .on_click(move |_, window, cx| {
                                    window.close_dialog(cx);
                                    view.update(cx, |this, cx| {
                                        this.begin_identity_link(provider.clone(), window, cx);
                                    });
                                })
                        },
                    )),
            )
        });
    }

    fn open_profile_dialog(&self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(user) = self.current_user.clone() else {
            return;
        };
        let username = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Username")
                .default_value(user.username.clone())
        });
        let display_name = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Display name")
                .default_value(user.display_name.clone().unwrap_or_default())
        });
        let email = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Email")
                .default_value(user.email.clone().unwrap_or_default())
        });
        let avatar = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Avatar URL")
                .default_value(user.avatar_url.clone().unwrap_or_default())
        });
        let description = cx.new(|cx| {
            InputState::new(window, cx)
                .multi_line(true)
                .rows(4)
                .placeholder("Description")
                .default_value(user.description.clone().unwrap_or_default())
        });
        let password = cx.new(|cx| {
            InputState::new(window, cx)
                .masked(true)
                .placeholder("New password (leave empty to keep current)")
        });
        let view = cx.entity().clone();
        window.open_dialog(cx, move |dialog, _, _| {
            let username = username.clone();
            let display_name = display_name.clone();
            let email = email.clone();
            let avatar = avatar.clone();
            let description = description.clone();
            let password = password.clone();
            let view = view.clone();
            let user = user.clone();
            dialog
                .title("Edit profile")
                .child(
                    v_flex()
                        .gap_3()
                        .child(Input::new(&username))
                        .child(Input::new(&display_name))
                        .child(Input::new(&email))
                        .child(Input::new(&avatar))
                        .child(Input::new(&description).h(px(110.0)))
                        .child(Input::new(&password).mask_toggle()),
                )
                .footer(move |_, _, _, _| {
                    let username = username.clone();
                    let display_name = display_name.clone();
                    let email = email.clone();
                    let avatar = avatar.clone();
                    let description = description.clone();
                    let password = password.clone();
                    let view = view.clone();
                    let user = user.clone();
                    vec![
                        Button::new("save-profile")
                            .primary()
                            .label("Save")
                            .on_click(move |_, window, cx| {
                                let mut user = user.clone();
                                let username = username.read(cx).value().trim().to_string();
                                if username.is_empty() {
                                    return;
                                }
                                user.username = username;
                                user.display_name = non_empty_text(display_name.read(cx).value());
                                user.email = non_empty_text(email.read(cx).value());
                                user.avatar_url = non_empty_text(avatar.read(cx).value());
                                user.description = non_empty_text(description.read(cx).value());
                                let password = password.read(cx).value().to_string();
                                user.password = (!password.is_empty()).then_some(password);
                                let mut mask = "username,display_name,email,avatar_url,description"
                                    .to_string();
                                if user.password.is_some() {
                                    mask.push_str(",password");
                                }
                                window.close_dialog(cx);
                                view.update(cx, |this, cx| {
                                    this.save_profile(user.clone(), mask.clone(), cx);
                                });
                            }),
                        Button::new("cancel-profile")
                            .label("Cancel")
                            .on_click(|_, window, cx| window.close_dialog(cx)),
                    ]
                })
        });
    }

    fn open_token_dialog(&self, window: &mut Window, cx: &mut Context<Self>) {
        let description = cx.new(|cx| InputState::new(window, cx).placeholder("Token description"));
        let days = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Expiration in days; 0 means never")
                .default_value("0")
        });
        let view = cx.entity().clone();
        window.open_dialog(cx, move |dialog, _, _| {
            let description = description.clone();
            let days = days.clone();
            let view = view.clone();
            dialog
                .title("Create personal access token")
                .child(
                    v_flex()
                        .gap_3()
                        .child(Input::new(&description))
                        .child(Input::new(&days)),
                )
                .footer(move |_, _, _, _| {
                    let description = description.clone();
                    let days = days.clone();
                    let view = view.clone();
                    vec![
                        Button::new("create-access-token")
                            .primary()
                            .label("Create")
                            .on_click(move |_, window, cx| {
                                let description = description.read(cx).value().to_string();
                                let days = match days.read(cx).value().parse::<i32>() {
                                    Ok(days) if days >= 0 => days,
                                    _ => {
                                        window.close_dialog(cx);
                                        view.update(cx, |this, cx| {
                                            this.error = Some(
                                                "Token expiration must be zero or a positive number of days."
                                                    .into(),
                                            );
                                            cx.notify();
                                        });
                                        return;
                                    }
                                };
                                window.close_dialog(cx);
                                view.update(cx, |this, cx| {
                                    this.create_access_token_resource(
                                        description.clone(),
                                        days,
                                        cx,
                                    );
                                });
                            }),
                        Button::new("cancel-access-token")
                            .label("Cancel")
                            .on_click(|_, window, cx| window.close_dialog(cx)),
                    ]
                })
        });
    }

    fn open_webhook_dialog(
        &self,
        existing: Option<memos_api::types::UserWebhook>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let display_name = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Webhook name")
                .default_value(
                    existing
                        .as_ref()
                        .and_then(|webhook| webhook.display_name.clone())
                        .unwrap_or_default(),
                )
        });
        let url = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("https://example.com/webhook")
                .default_value(
                    existing
                        .as_ref()
                        .and_then(|webhook| webhook.url.clone())
                        .unwrap_or_default(),
                )
        });
        let secret = cx.new(|cx| {
            InputState::new(window, cx)
                .masked(true)
                .placeholder("Signing secret (optional)")
        });
        let view = cx.entity().clone();
        let webhook_name = existing.and_then(|webhook| webhook.name);
        window.open_dialog(cx, move |dialog, _, _| {
            let display_name = display_name.clone();
            let url = url.clone();
            let secret = secret.clone();
            let view = view.clone();
            let webhook_name = webhook_name.clone();
            dialog
                .title(if webhook_name.is_some() {
                    "Edit webhook"
                } else {
                    "Create webhook"
                })
                .child(
                    v_flex()
                        .gap_3()
                        .child(Input::new(&display_name))
                        .child(Input::new(&url))
                        .child(Input::new(&secret).mask_toggle()),
                )
                .footer(move |_, _, _, _| {
                    let display_name = display_name.clone();
                    let url = url.clone();
                    let secret = secret.clone();
                    let view = view.clone();
                    let webhook_name = webhook_name.clone();
                    vec![
                        Button::new("save-webhook")
                            .primary()
                            .label("Save")
                            .on_click(move |_, window, cx| {
                                let url = url.read(cx).value().trim().to_string();
                                let valid_url = url::Url::parse(&url).is_ok_and(|url| {
                                    matches!(url.scheme(), "http" | "https") && url.host().is_some()
                                });
                                if !valid_url {
                                    window.close_dialog(cx);
                                    view.update(cx, |this, cx| {
                                        this.error =
                                            Some("Webhook URL must be a valid HTTP(S) URL.".into());
                                        cx.notify();
                                    });
                                    return;
                                }
                                let webhook = memos_api::types::UserWebhook {
                                    create_time: None,
                                    display_name: non_empty_text(display_name.read(cx).value()),
                                    name: webhook_name.clone(),
                                    signing_secret: non_empty_text(secret.read(cx).value()),
                                    signing_secret_set: None,
                                    update_time: None,
                                    url: Some(url),
                                };
                                window.close_dialog(cx);
                                view.update(cx, |this, cx| {
                                    this.save_webhook_resource(webhook.clone(), cx);
                                });
                            }),
                        Button::new("cancel-webhook")
                            .label("Cancel")
                            .on_click(|_, window, cx| window.close_dialog(cx)),
                    ]
                })
        });
    }

    fn settings_section_button(
        &self,
        section: SettingsSection,
        label: &'static str,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let selected = self.settings_section == section;
        let view = cx.entity().clone();
        Button::new(match section {
            SettingsSection::Account => "settings-account",
            SettingsSection::Preferences => "settings-preferences",
            SettingsSection::Tokens => "settings-tokens",
            SettingsSection::Webhooks => "settings-webhooks",
            SettingsSection::Administration => "settings-administration",
        })
        .ghost()
        .when(selected, |button| button.primary())
        .label(label)
        .on_click(move |_, _, cx| {
            view.update(cx, |this, cx| this.set_settings_section(section, cx));
        })
    }

    fn render_account_settings(&self, cx: &mut Context<Self>) -> AnyElement {
        let Some(user) = self.current_user.clone() else {
            return empty_state(
                IconName::User,
                "Anonymous session",
                "Sign in to edit an account.",
            );
        };
        let edit_view = cx.entity().clone();
        let link_identity_view = cx.entity().clone();
        let identity_view = cx.entity().clone();
        let display_name = user
            .display_name
            .clone()
            .unwrap_or_else(|| user.username.clone());
        let stats = self.account_resources.stats.clone().unwrap_or_default();
        let identities = self.account_resources.identities.clone();
        let avatar = user
            .name
            .as_deref()
            .map(|name| self.user_avatar(name, 44.0));
        v_flex()
            .gap_4()
            .child(
                v_flex()
                    .border_1()
                    .border_color(theme::line())
                    .child(
                        h_flex()
                            .min_h(px(72.0))
                            .px_4()
                            .items_center()
                            .justify_between()
                            .child(
                                h_flex()
                                    .items_center()
                                    .gap_3()
                                    .when_some(avatar, |profile, avatar| profile.child(avatar))
                                    .child(
                                        v_flex()
                                            .gap_1()
                                            .child(
                                                div().text_sm().font_semibold().child(display_name),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(theme::graphite())
                                                    .child(format!("@{}", user.username)),
                                            ),
                                    ),
                            )
                            .child(
                                Button::new("edit-profile")
                                    .primary()
                                    .icon(IconName::Replace)
                                    .label("Edit profile")
                                    .on_click(move |_, window, cx| {
                                        edit_view.update(cx, |this, cx| {
                                            this.open_profile_dialog(window, cx);
                                        });
                                    }),
                            ),
                    )
                    .child(setting_row(
                        "Email",
                        user.email.as_deref().unwrap_or("Not configured"),
                        None,
                    ))
                    .child(setting_row(
                        "Memos",
                        &stats.total_memo_count.unwrap_or_default().to_string(),
                        Some("Total memos created"),
                    ))
                    .child(setting_row(
                        "Pinned",
                        &stats.pinned_memos.len().to_string(),
                        Some("Pinned memo count"),
                    )),
            )
            .child(
                v_flex()
                    .border_1()
                    .border_color(theme::line())
                    .child(
                        h_flex()
                            .px_4()
                            .py_2()
                            .items_center()
                            .justify_between()
                            .child(panel_label("LINKED IDENTITIES"))
                            .child(
                                Button::new("link-sso-identity")
                                    .primary()
                                    .icon(IconName::Plus)
                                    .label("Link SSO")
                                    .on_click(move |_, window, cx| {
                                        link_identity_view.update(cx, |this, cx| {
                                            this.discover_identity_link(window, cx);
                                        });
                                    }),
                            ),
                    )
                    .when(identities.is_empty(), |panel| {
                        panel.child(
                            div()
                                .p_4()
                                .text_xs()
                                .text_color(theme::graphite())
                                .child("No linked SSO identities."),
                        )
                    })
                    .children(
                        identities
                            .into_iter()
                            .enumerate()
                            .map(move |(ix, identity)| {
                                let view = identity_view.clone();
                                let name = identity.name.clone().unwrap_or_default();
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
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .child(identity.idp_name.unwrap_or_default()),
                                            )
                                            .child(
                                                div()
                                                    .font_family(theme::mono_family())
                                                    .text_xs()
                                                    .text_color(theme::graphite())
                                                    .child(identity.extern_uid.unwrap_or_default()),
                                            ),
                                    )
                                    .child(
                                        Button::new(("unlink-identity", ix))
                                            .danger()
                                            .icon(IconName::Delete)
                                            .tooltip("Unlink identity")
                                            .on_click(move |_, _, cx| {
                                                view.update(cx, |this, cx| {
                                                    this.delete_linked_identity_resource(
                                                        name.clone(),
                                                        cx,
                                                    );
                                                });
                                            }),
                                    )
                            }),
                    ),
            )
            .into_any_element()
    }

    fn render_preferences_settings(&self, cx: &mut Context<Self>) -> AnyElement {
        let settings = self.account_resources.settings.clone();
        let view = cx.entity().clone();
        v_flex()
            .gap_4()
            .child(
                v_flex()
                    .border_1()
                    .border_color(theme::line())
                    .child(panel_label("APPEARANCE"))
                    .child(
                        h_flex().p_3().gap_2().flex_wrap().children(
                            [
                                (ThemePreference::System, IconName::Palette),
                                (ThemePreference::Light, IconName::Sun),
                                (ThemePreference::Dark, IconName::Moon),
                            ]
                            .into_iter()
                            .map(|(preference, icon)| {
                                let view = cx.entity().clone();
                                Button::new(gpui::SharedString::from(format!(
                                    "theme-{:?}",
                                    preference
                                )))
                                .outline()
                                .when(self.theme_preference == preference, |button| {
                                    button.primary()
                                })
                                .icon(icon)
                                .label(preference.label())
                                .on_click(
                                    move |_, window, cx| {
                                        view.update(cx, |this, cx| {
                                            this.set_theme_preference(preference, window, cx);
                                        });
                                    },
                                )
                            }),
                        ),
                    ),
            )
            .child(
                v_flex()
                    .border_1()
                    .border_color(theme::line())
                    .child(panel_label("SERVER SETTINGS"))
                    .when(settings.is_empty(), |panel| {
                        panel.child(empty_state(
                            IconName::Settings2,
                            "No user settings",
                            "This Memos instance returned no user settings.",
                        ))
                    })
                    .children(settings.into_iter().enumerate().map(move |(ix, setting)| {
                        let view = view.clone();
                        let edit_setting = setting.clone();
                        let name = setting.name.clone().unwrap_or_default();
                        h_flex()
                            .min_h(px(60.0))
                            .px_4()
                            .items_center()
                            .justify_between()
                            .border_b_1()
                            .border_color(theme::line())
                            .child(
                                div()
                                    .font_family(theme::mono_family())
                                    .text_sm()
                                    .child(resource_id(&name)),
                            )
                            .child(
                                Button::new(("edit-user-setting", ix))
                                    .ghost()
                                    .icon(IconName::Replace)
                                    .label("Edit")
                                    .on_click(move |_, window, cx| {
                                        view.update(cx, |this, cx| {
                                            this.open_json_editor(
                                                "Edit user setting",
                                                edit_setting.clone(),
                                                window,
                                                cx,
                                                |this, setting, cx| {
                                                    this.save_user_setting_resource(setting, cx);
                                                },
                                            );
                                        });
                                    }),
                            )
                    })),
            )
            .into_any_element()
    }

    fn render_token_settings(&self, cx: &mut Context<Self>) -> AnyElement {
        let tokens = self.account_resources.tokens.clone();
        let create_view = cx.entity().clone();
        let delete_view = cx.entity().clone();
        v_flex()
            .gap_3()
            .child(
                h_flex().justify_end().child(
                    Button::new("new-access-token")
                        .primary()
                        .icon(IconName::Plus)
                        .label("New token")
                        .on_click(move |_, window, cx| {
                            create_view.update(cx, |this, cx| {
                                this.open_token_dialog(window, cx);
                            });
                        }),
                ),
            )
            .child(
                v_flex()
                    .border_1()
                    .border_color(theme::line())
                    .when(tokens.is_empty(), |panel| {
                        panel.child(
                            div()
                                .p_4()
                                .text_xs()
                                .text_color(theme::graphite())
                                .child("No personal access tokens."),
                        )
                    })
                    .children(tokens.into_iter().enumerate().map(move |(ix, token)| {
                        let view = delete_view.clone();
                        let name = token.name.clone().unwrap_or_default();
                        h_flex()
                            .min_h(px(60.0))
                            .px_4()
                            .items_center()
                            .justify_between()
                            .border_b_1()
                            .border_color(theme::line())
                            .child(
                                v_flex()
                                    .gap_1()
                                    .child(
                                        div().text_sm().font_semibold().child(
                                            token.description.unwrap_or_else(|| "Token".into()),
                                        ),
                                    )
                                    .child(div().text_xs().text_color(theme::graphite()).child(
                                        match token.expires_at {
                                            Some(time) => format!(
                                                "Expires {}",
                                                time.with_timezone(&Local).format("%Y-%m-%d")
                                            ),
                                            None => "Never expires".into(),
                                        },
                                    )),
                            )
                            .child(
                                Button::new(("delete-access-token", ix))
                                    .danger()
                                    .icon(IconName::Delete)
                                    .tooltip("Delete token")
                                    .on_click(move |_, _, cx| {
                                        view.update(cx, |this, cx| {
                                            this.delete_access_token_resource(name.clone(), cx);
                                        });
                                    }),
                            )
                    })),
            )
            .into_any_element()
    }

    fn render_webhook_settings(&self, cx: &mut Context<Self>) -> AnyElement {
        let webhooks = self.account_resources.webhooks.clone();
        let create_view = cx.entity().clone();
        let webhook_view = cx.entity().clone();
        v_flex()
            .gap_3()
            .child(
                h_flex().justify_end().child(
                    Button::new("new-webhook")
                        .primary()
                        .icon(IconName::Plus)
                        .label("New webhook")
                        .on_click(move |_, window, cx| {
                            create_view.update(cx, |this, cx| {
                                this.open_webhook_dialog(None, window, cx);
                            });
                        }),
                ),
            )
            .child(
                v_flex()
                    .border_1()
                    .border_color(theme::line())
                    .when(webhooks.is_empty(), |panel| {
                        panel.child(
                            div()
                                .p_4()
                                .text_xs()
                                .text_color(theme::graphite())
                                .child("No webhooks configured."),
                        )
                    })
                    .children(webhooks.into_iter().enumerate().map(move |(ix, webhook)| {
                        let edit_view = webhook_view.clone();
                        let secret_view = webhook_view.clone();
                        let delete_view = webhook_view.clone();
                        let edit_webhook = webhook.clone();
                        let name = webhook.name.clone().unwrap_or_default();
                        let secret_name = name.clone();
                        let delete_name = name.clone();
                        h_flex()
                            .min_h(px(64.0))
                            .px_4()
                            .items_center()
                            .justify_between()
                            .border_b_1()
                            .border_color(theme::line())
                            .child(
                                v_flex()
                                    .flex_1()
                                    .min_w_0()
                                    .gap_1()
                                    .child(div().text_sm().font_semibold().child(
                                        webhook.display_name.unwrap_or_else(|| "Webhook".into()),
                                    ))
                                    .child(
                                        div()
                                            .font_family(theme::mono_family())
                                            .text_xs()
                                            .text_color(theme::graphite())
                                            .overflow_hidden()
                                            .text_ellipsis()
                                            .child(webhook.url.unwrap_or_default()),
                                    ),
                            )
                            .child(
                                h_flex()
                                    .gap_1()
                                    .when(webhook.signing_secret_set.unwrap_or(false), |actions| {
                                        actions.child(
                                            Button::new(("copy-webhook-secret", ix))
                                                .ghost()
                                                .icon(IconName::Copy)
                                                .tooltip("Copy signing secret")
                                                .on_click(move |_, _, cx| {
                                                    secret_view.update(cx, |this, cx| {
                                                        this.copy_webhook_secret(
                                                            secret_name.clone(),
                                                            cx,
                                                        );
                                                    });
                                                }),
                                        )
                                    })
                                    .child(
                                        Button::new(("edit-webhook", ix))
                                            .ghost()
                                            .icon(IconName::Replace)
                                            .tooltip("Edit webhook")
                                            .on_click(move |_, window, cx| {
                                                edit_view.update(cx, |this, cx| {
                                                    this.open_webhook_dialog(
                                                        Some(edit_webhook.clone()),
                                                        window,
                                                        cx,
                                                    );
                                                });
                                            }),
                                    )
                                    .child(
                                        Button::new(("delete-webhook", ix))
                                            .danger()
                                            .icon(IconName::Delete)
                                            .tooltip("Delete webhook")
                                            .on_click(move |_, _, cx| {
                                                delete_view.update(cx, |this, cx| {
                                                    this.delete_webhook_resource(
                                                        delete_name.clone(),
                                                        cx,
                                                    );
                                                });
                                            }),
                                    ),
                            )
                    })),
            )
            .into_any_element()
    }

    fn render_admin_settings(&self, cx: &mut Context<Self>) -> AnyElement {
        let users = self.admin_resources.users.clone();
        let current_user_name = self
            .current_user
            .as_ref()
            .and_then(|user| user.name.clone());
        let settings = self.admin_resources.instance_settings.clone();
        let providers = self.admin_resources.identity_providers.clone();
        let resource_view = cx.entity().clone();
        let email_view = cx.entity().clone();
        let create_user_view = cx.entity().clone();
        let create_idp_view = cx.entity().clone();
        let instance_stats = self
            .admin_resources
            .instance_stats
            .clone()
            .unwrap_or_default();
        let user_stats = self.admin_resources.user_stats.clone().unwrap_or_default();
        let total_memos = user_stats
            .stats
            .iter()
            .filter_map(|stats| stats.total_memo_count)
            .sum::<i32>();
        let new_user = memos_api::types::User {
            avatar_url: None,
            create_time: None,
            description: None,
            display_name: Some("New user".into()),
            email: None,
            name: None,
            password: Some("change-me".into()),
            role: memos_api::types::UserRole::User,
            state: memos_api::types::UserState::Normal,
            update_time: None,
            username: "new-user".into(),
        };
        let new_provider = memos_api::types::IdentityProvider {
            config: memos_api::types::IdentityProviderConfig::default(),
            identifier_filter: None,
            name: None,
            title: "OAuth2 provider".into(),
            type_: memos_api::types::IdentityProviderType::Oauth2,
        };
        v_flex()
            .gap_4()
            .child(
                v_flex()
                    .border_1()
                    .border_color(theme::line())
                    .child(panel_label("RESOURCE STATS"))
                    .child(setting_row(
                        "Users",
                        &self.admin_resources.users.len().to_string(),
                        Some("Registered accounts"),
                    ))
                    .child(setting_row(
                        "Total memos",
                        &total_memos.to_string(),
                        Some("Across all users"),
                    ))
                    .child(setting_row(
                        "Database",
                        instance_stats
                            .database
                            .as_ref()
                            .and_then(|stats| stats.driver.as_deref())
                            .unwrap_or("Unknown"),
                        instance_stats
                            .database
                            .as_ref()
                            .and_then(|stats| stats.size_bytes.as_deref()),
                    ))
                    .child(setting_row(
                        "Local storage bytes",
                        instance_stats.local_storage_bytes.as_deref().unwrap_or("Unknown"),
                        None,
                    ))
                    .child(
                        h_flex().justify_end().p_3().child(
                            Button::new("test-instance-email")
                                .outline()
                                .icon(IconName::ExternalLink)
                                .label("Send test email")
                                .on_click(move |_, _, cx| {
                                    email_view.update(cx, |this, cx| {
                                        this.test_instance_email(cx);
                                    });
                                }),
                        ),
                    ),
            )
            .child(
                v_flex()
                    .border_1()
                    .border_color(theme::line())
                    .child(
                        h_flex()
                            .px_4()
                            .py_2()
                            .items_center()
                            .justify_between()
                            .child(panel_label("USERS"))
                            .child(
                                Button::new("create-admin-user")
                                    .primary()
                                    .icon(IconName::Plus)
                                    .label("New user")
                                    .on_click(move |_, window, cx| {
                                        let value = new_user.clone();
                                        create_user_view.update(cx, |this, cx| {
                                            this.open_json_editor(
                                                "Create user",
                                                value,
                                                window,
                                                cx,
                                                |this, user, cx| {
                                                    this.save_admin_user(user, cx);
                                                },
                                            );
                                        });
                                    }),
                            ),
                    )
                    .children(users.into_iter().enumerate().map({
                        let resource_view = resource_view.clone();
                        let current_user_name = current_user_name.clone();
                        move |(ix, user)| {
                            let profile_view = resource_view.clone();
                            let edit_view = resource_view.clone();
                            let delete_view = resource_view.clone();
                            let edit_user = user.clone();
                            let name = user.name.clone().unwrap_or_default();
                            let can_delete = Some(name.clone()) != current_user_name;
                            let profile_name = name.clone();
                            h_flex()
                                .min_h(px(60.0))
                                .px_4()
                                .items_center()
                                .justify_between()
                                .border_b_1()
                                .border_color(theme::line())
                                .child(
                                    v_flex()
                                        .gap_1()
                                        .child(
                                            div().text_sm().font_semibold().child(
                                                user.display_name
                                                    .unwrap_or_else(|| user.username.clone()),
                                            ),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme::graphite())
                                                .child(format!(
                                                    "@{} · {:?}",
                                                    user.username, user.role
                                                )),
                                        ),
                                )
                                .child(
                                    h_flex()
                                        .gap_1()
                                        .child(
                                            Button::new(("open-user-profile", ix))
                                                .ghost()
                                                .icon(IconName::User)
                                                .tooltip("Open profile")
                                                .on_click(move |_, _, cx| {
                                                    profile_view.update(cx, |this, cx| {
                                                        this.open_user_profile(
                                                            profile_name.clone(),
                                                            cx,
                                                        );
                                                    });
                                                }),
                                        )
                                        .child(
                                            Button::new(("edit-admin-user", ix))
                                                .ghost()
                                                .icon(IconName::Replace)
                                                .tooltip("Edit user JSON")
                                                .on_click(move |_, window, cx| {
                                                    edit_view.update(cx, |this, cx| {
                                                        this.open_json_editor(
                                                            "Edit user",
                                                            edit_user.clone(),
                                                            window,
                                                            cx,
                                                            |this, user, cx| {
                                                                this.save_admin_user(user, cx);
                                                            },
                                                        );
                                                    });
                                                }),
                                        )
                                        .child(
                                            Button::new(("delete-admin-user", ix))
                                                .danger()
                                                .disabled(!can_delete)
                                                .icon(IconName::Delete)
                                                .tooltip("Delete user")
                                                .on_click(move |_, _, cx| {
                                                    delete_view.update(cx, |this, cx| {
                                                        this.delete_admin_user(name.clone(), cx);
                                                    });
                                                }),
                                        ),
                                )
                        }
                    })),
            )
            .child(
                v_flex()
                    .border_1()
                    .border_color(theme::line())
                    .child(panel_label("INSTANCE SETTINGS"))
                    .children(settings.into_iter().enumerate().map({
                        let resource_view = resource_view.clone();
                        move |(ix, setting)| {
                            let view = resource_view.clone();
                            let edit_setting = setting.clone();
                            h_flex()
                                .min_h(px(56.0))
                                .px_4()
                                .items_center()
                                .justify_between()
                                .border_b_1()
                                .border_color(theme::line())
                                .child(
                                    div()
                                        .font_family(theme::mono_family())
                                        .text_sm()
                                        .child(
                                            setting
                                                .name
                                                .as_deref()
                                                .map(resource_id)
                                                .unwrap_or_else(|| "UNKNOWN".into()),
                                        ),
                                )
                                .child(
                                    Button::new(("edit-instance-setting", ix))
                                        .ghost()
                                        .icon(IconName::Replace)
                                        .label("Edit JSON")
                                        .on_click(move |_, window, cx| {
                                            view.update(cx, |this, cx| {
                                                this.open_json_editor(
                                                    "Edit instance setting",
                                                    edit_setting.clone(),
                                                    window,
                                                    cx,
                                                    |this, setting, cx| {
                                                        this.save_instance_setting_resource(
                                                            setting, cx,
                                                        );
                                                    },
                                                );
                                            });
                                        }),
                                )
                        }
                    })),
            )
            .child(
                v_flex()
                    .border_1()
                    .border_color(theme::line())
                    .child(
                        h_flex()
                            .px_4()
                            .py_2()
                            .items_center()
                            .justify_between()
                            .child(panel_label("IDENTITY PROVIDERS"))
                            .child(
                                Button::new("create-idp")
                                    .primary()
                                    .icon(IconName::Plus)
                                    .label("New provider")
                                    .on_click(move |_, window, cx| {
                                        let value = new_provider.clone();
                                        create_idp_view.update(cx, |this, cx| {
                                            this.open_json_editor(
                                                "Create identity provider",
                                                value,
                                                window,
                                                cx,
                                                |this, provider, cx| {
                                                    this.save_identity_provider_resource(
                                                        provider, None, cx,
                                                    );
                                                },
                                            );
                                        });
                                    }),
                            ),
                    )
                    .children(providers.into_iter().enumerate().map({
                        let resource_view = resource_view.clone();
                        move |(ix, provider)| {
                            let edit_view = resource_view.clone();
                            let delete_view = resource_view.clone();
                            let edit_provider = provider.clone();
                            let name = provider.name.clone().unwrap_or_default();
                            h_flex()
                                .min_h(px(58.0))
                                .px_4()
                                .items_center()
                                .justify_between()
                                .border_b_1()
                                .border_color(theme::line())
                                .child(
                                    v_flex()
                                        .gap_1()
                                        .child(
                                            div()
                                                .text_sm()
                                                .font_semibold()
                                                .child(provider.title),
                                        )
                                        .child(
                                            div()
                                                .font_family(theme::mono_family())
                                                .text_xs()
                                                .text_color(theme::graphite())
                                                .child(format!("{:?}", provider.type_)),
                                        ),
                                )
                                .child(
                                    h_flex()
                                        .gap_1()
                                        .child(
                                            Button::new(("edit-idp", ix))
                                                .ghost()
                                                .icon(IconName::Replace)
                                                .on_click(move |_, window, cx| {
                                                    edit_view.update(cx, |this, cx| {
                                                        this.open_json_editor(
                                                            "Edit identity provider",
                                                            edit_provider.clone(),
                                                            window,
                                                            cx,
                                                            |this, provider, cx| {
                                                                this.save_identity_provider_resource(
                                                                    provider, None, cx,
                                                                );
                                                            },
                                                        );
                                                    });
                                                }),
                                        )
                                        .child(
                                            Button::new(("delete-idp", ix))
                                                .danger()
                                                .icon(IconName::Delete)
                                                .on_click(move |_, _, cx| {
                                                    delete_view.update(cx, |this, cx| {
                                                        this.delete_identity_provider_resource(
                                                            name.clone(),
                                                            cx,
                                                        );
                                                    });
                                                }),
                                        ),
                                )
                        }
                    })),
            )
            .into_any_element()
    }

    fn render_settings_page(&self, cx: &mut Context<Self>) -> AnyElement {
        let is_admin = self
            .current_user
            .as_ref()
            .map(|user| user.role == memos_api::types::UserRole::Admin)
            .unwrap_or(false);
        let content = match self.settings_section {
            SettingsSection::Account => self.render_account_settings(cx),
            SettingsSection::Preferences => self.render_preferences_settings(cx),
            SettingsSection::Tokens => self.render_token_settings(cx),
            SettingsSection::Webhooks => self.render_webhook_settings(cx),
            SettingsSection::Administration => self.render_admin_settings(cx),
        };
        module_page(
            "Settings",
            "Account and instance",
            v_flex()
                .gap_4()
                .child(
                    h_flex()
                        .gap_1()
                        .flex_wrap()
                        .child(self.settings_section_button(
                            SettingsSection::Account,
                            "Account",
                            cx,
                        ))
                        .child(self.settings_section_button(
                            SettingsSection::Preferences,
                            "Preferences",
                            cx,
                        ))
                        .child(self.settings_section_button(SettingsSection::Tokens, "Tokens", cx))
                        .child(self.settings_section_button(
                            SettingsSection::Webhooks,
                            "Webhooks",
                            cx,
                        ))
                        .when(is_admin, |nav| {
                            nav.child(self.settings_section_button(
                                SettingsSection::Administration,
                                "Administration",
                                cx,
                            ))
                        }),
                )
                .child(content)
                .child(
                    h_flex().justify_end().child(
                        Button::new("disconnect")
                            .danger()
                            .outline()
                            .label("Disconnect")
                            .on_click(
                                cx.listener(|this, _, window, cx| this.disconnect(window, cx)),
                            ),
                    ),
                ),
        )
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

struct MemoTask {
    line_index: usize,
    checked: bool,
    label: String,
}

fn markdown_tasks(content: &str) -> Vec<MemoTask> {
    content
        .lines()
        .enumerate()
        .filter_map(|(line_index, line)| {
            let marker = line.find("[ ]").map(|index| (index, false)).or_else(|| {
                line.find("[x]")
                    .or_else(|| line.find("[X]"))
                    .map(|index| (index, true))
            })?;
            let prefix = line[..marker.0].trim_start();
            if !matches!(prefix, "-" | "*" | "+") {
                return None;
            }
            Some(MemoTask {
                line_index,
                checked: marker.1,
                label: line[marker.0 + 3..].trim().to_string(),
            })
        })
        .collect()
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

fn non_empty_text(value: gpui::SharedString) -> Option<String> {
    let value = value.trim().to_string();
    (!value.is_empty()).then_some(value)
}

fn attachment_metadata(attachment: &memos_api::types::Attachment) -> String {
    let mut parts = vec![attachment.type_.clone()];
    if let Some(size) = attachment.size.as_deref().filter(|size| !size.is_empty()) {
        parts.push(format!("{size} bytes"));
    }
    if let Some(motion) = attachment.motion_media.as_ref() {
        if let Some(family) = motion.family {
            parts.push(family.to_string());
        }
        if let Some(role) = motion.role {
            parts.push(role.to_string());
        }
    }
    parts.join(" · ")
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
