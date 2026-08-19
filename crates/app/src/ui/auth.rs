use gpui::{
    AnyElement, InteractiveElement as _, IntoElement, ParentElement as _, Styled, Window, div,
    prelude::FluentBuilder as _, px,
};
use gpui_component::{
    Icon, IconName, Sizable, StyledExt, WindowExt as _,
    button::{Button, ButtonVariants as _},
    h_flex,
    input::Input,
    v_flex,
};

use super::{AuthMode, MemosDesktop};
use crate::{api::ApiSession, theme};

impl MemosDesktop {
    pub(super) fn show_sso_provider_dialog(
        &self,
        server_url: String,
        session: ApiSession,
        profile: memos_api::types::InstanceProfile,
        providers: Vec<memos_api::types::IdentityProvider>,
        window: &mut Window,
        cx: &mut gpui::Context<Self>,
    ) {
        let view = cx.entity().clone();
        window.open_dialog(cx, move |dialog, _, _| {
            let view = view.clone();
            let server_url = server_url.clone();
            let session = session.clone();
            let profile = profile.clone();
            dialog.title("Sign in with SSO").child(
                v_flex()
                    .gap_2()
                    .children(providers.clone().into_iter().enumerate().map(
                        move |(ix, provider)| {
                            let view = view.clone();
                            let server_url = server_url.clone();
                            let session = session.clone();
                            let profile = profile.clone();
                            let label = provider.title.clone();
                            Button::new(("sso-provider", ix))
                                .large()
                                .outline()
                                .icon(IconName::ExternalLink)
                                .label(label)
                                .on_click(move |_, window, cx| {
                                    window.close_dialog(cx);
                                    view.update(cx, |this, cx| {
                                        this.begin_sso(
                                            server_url.clone(),
                                            session.clone(),
                                            profile.clone(),
                                            provider.clone(),
                                            window,
                                            cx,
                                        );
                                    });
                                })
                        },
                    )),
            )
        });
    }

    pub(super) fn render_auth(
        &mut self,
        _window: &mut Window,
        cx: &mut gpui::Context<Self>,
    ) -> AnyElement {
        let password_mode = self.auth_mode == AuthMode::Password;
        let notice = self.notice.clone();
        let error = self.error.clone();

        h_flex()
            .id("auth-screen")
            .size_full()
            .bg(theme::paper())
            .child(
                v_flex()
                    .w(px(392.0))
                    .h_full()
                    .flex_shrink_0()
                    .justify_between()
                    .p_8()
                    .bg(theme::nav())
                    .text_color(theme::color(0xf2f3ef))
                    .child(
                        v_flex()
                            .gap_8()
                            .child(
                                h_flex()
                                    .items_center()
                                    .gap_3()
                                    .child(
                                        div()
                                            .relative()
                                            .size_10()
                                            .rounded(px(4.0))
                                            .bg(theme::color(0xf2f3ef))
                                            .text_color(theme::nav())
                                            .text_lg()
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
                                            .gap_0p5()
                                            .child(
                                                div()
                                                    .text_base()
                                                    .font_semibold()
                                                    .child("Memos Desktop"),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(theme::color(0xaeb3ba))
                                                    .child("Native client / v0.1.0"),
                                            ),
                                    ),
                            )
                            .child(
                                v_flex()
                                    .gap_4()
                                    .child(
                                        div()
                                            .text_3xl()
                                            .font_semibold()
                                            .line_height(px(38.0))
                                            .child("Connect to your Memos instance."),
                                    )
                                    .child(div().h(px(1.0)).w(px(64.0)).bg(theme::vermilion()))
                                    .child(
                                        v_flex()
                                            .gap_3()
                                            .text_sm()
                                            .text_color(theme::color(0xb9bec5))
                                            .child(status_row("01", "Server profile"))
                                            .child(status_row("02", "Session authentication"))
                                            .child(status_row("03", "Timeline synchronization")),
                                    ),
                            ),
                    )
                    .child(
                        v_flex()
                            .gap_2()
                            .text_xs()
                            .text_color(theme::color(0x7f858e))
                            .child("Passwords are stored only in the system credential store.")
                            .child("Successful password sign-in resumes automatically next time."),
                    ),
            )
            .child(
                v_flex()
                    .flex_1()
                    .min_w_0()
                    .h_full()
                    .items_center()
                    .justify_center()
                    .px_8()
                    .child(
                        v_flex()
                            .w_full()
                            .max_w(px(440.0))
                            .gap_6()
                            .child(
                                v_flex()
                                    .gap_1()
                                    .child(div().text_2xl().font_semibold().child("Open instance"))
                                    .child(div().text_sm().text_color(theme::graphite()).child(
                                        "Use a password session or a personal access token.",
                                    )),
                            )
                            .child(
                                h_flex()
                                    .gap_1()
                                    .p_1()
                                    .rounded(px(4.0))
                                    .bg(theme::subtle_surface())
                                    .child(
                                        Button::new("auth-password")
                                            .small()
                                            .when(password_mode, |button| button.primary())
                                            .when(!password_mode, |button| button.ghost())
                                            .label("Password")
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.set_auth_mode(AuthMode::Password, cx);
                                            })),
                                    )
                                    .child(
                                        Button::new("auth-token")
                                            .small()
                                            .when(!password_mode, |button| button.primary())
                                            .when(password_mode, |button| button.ghost())
                                            .label("Access token")
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.set_auth_mode(AuthMode::AccessToken, cx);
                                            })),
                                    ),
                            )
                            .child(
                                v_flex().gap_2().child(field_label("Server URL")).child(
                                    Input::new(&self.server_input)
                                        .prefix(Icon::new(IconName::Globe).size_4())
                                        .cleanable(true),
                                ),
                            )
                            .when(password_mode, |form| {
                                form.child(
                                    v_flex()
                                        .gap_4()
                                        .child(
                                            v_flex().gap_2().child(field_label("Username")).child(
                                                Input::new(&self.username_input).cleanable(true),
                                            ),
                                        )
                                        .child(
                                            v_flex().gap_2().child(field_label("Password")).child(
                                                Input::new(&self.password_input).mask_toggle(),
                                            ),
                                        ),
                                )
                            })
                            .when(!password_mode, |form| {
                                form.child(
                                    v_flex()
                                        .gap_2()
                                        .child(field_label("Personal access token"))
                                        .child(Input::new(&self.token_input).mask_toggle()),
                                )
                            })
                            .when_some(error, |form, error| {
                                form.child(
                                    h_flex()
                                        .items_start()
                                        .gap_2()
                                        .p_3()
                                        .border_1()
                                        .border_color(theme::error_border())
                                        .bg(theme::error_background())
                                        .text_sm()
                                        .text_color(theme::error_text())
                                        .child(Icon::new(IconName::TriangleAlert).size_4())
                                        .child(div().flex_1().min_w_0().child(error)),
                                )
                            })
                            .when_some(notice, |form, notice| {
                                form.child(
                                    h_flex()
                                        .items_center()
                                        .gap_2()
                                        .text_sm()
                                        .text_color(theme::graphite())
                                        .child(Icon::new(IconName::LoaderCircle).size_4())
                                        .child(notice),
                                )
                            })
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(
                                        Button::new("connect")
                                            .primary()
                                            .large()
                                            .loading(self.loading)
                                            .label("Connect")
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.authenticate(false, cx);
                                            })),
                                    )
                                    .child(
                                        Button::new("browse-public")
                                            .large()
                                            .outline()
                                            .label("Browse public")
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.authenticate(true, cx);
                                            })),
                                    ),
                            )
                            .when(password_mode, |form| {
                                form.child(
                                    Button::new("create-account")
                                        .large()
                                        .outline()
                                        .icon(IconName::Plus)
                                        .label("Create account or initialize instance")
                                        .loading(self.loading)
                                        .on_click(cx.listener(|this, _, _, cx| {
                                            this.register(cx);
                                        })),
                                )
                            })
                            .child(
                                Button::new("connect-sso")
                                    .large()
                                    .outline()
                                    .icon(IconName::ExternalLink)
                                    .label("Sign in with SSO")
                                    .loading(self.loading)
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.discover_sso(window, cx);
                                    })),
                            )
                            .child(
                                v_flex().gap_2().child(field_label("Shared memo")).child(
                                    h_flex()
                                        .gap_2()
                                        .child(
                                            Input::new(&self.shared_link_input)
                                                .prefix(Icon::new(IconName::ExternalLink).size_4())
                                                .cleanable(true),
                                        )
                                        .child(
                                            Button::new("open-shared-memo")
                                                .large()
                                                .label("Open")
                                                .on_click(cx.listener(|this, _, _, cx| {
                                                    this.open_shared_memo(cx);
                                                })),
                                        ),
                                ),
                            ),
                    ),
            )
            .into_any_element()
    }
}

fn field_label(label: &'static str) -> impl IntoElement {
    div()
        .text_xs()
        .font_semibold()
        .text_color(theme::graphite())
        .child(label)
}

fn status_row(index: &'static str, label: &'static str) -> impl IntoElement {
    h_flex()
        .items_center()
        .gap_3()
        .child(
            div()
                .w(px(24.0))
                .font_family(theme::mono_family())
                .text_color(theme::vermilion())
                .child(index),
        )
        .child(label)
}
