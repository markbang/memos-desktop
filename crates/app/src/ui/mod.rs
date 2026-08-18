mod auth;
mod workspace;

use std::sync::Arc;

use chrono::Utc;
use gpui::{
    AnyElement, AppContext, Context, Entity, IntoElement, PathPromptOptions, Render, Subscription,
    Window,
};
use gpui_component::input::{InputEvent, InputState};
use memos_api::types::{InstanceProfile, Memo, MemoProperty, MemoState, MemoVisibility, User};
use tokio::runtime::Runtime;

use crate::{
    api::{ApiSession, MemoDetailData},
    config::AppConfig,
    demo,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AuthMode {
    Password,
    AccessToken,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Route {
    Timeline,
    Explore,
    Views,
    Archive,
    Attachments,
    Inbox,
    Settings,
}

impl Route {
    fn title(self) -> &'static str {
        match self {
            Self::Timeline => "Timeline",
            Self::Explore => "Explore",
            Self::Views => "Saved views",
            Self::Archive => "Archive",
            Self::Attachments => "Attachments",
            Self::Inbox => "Inbox",
            Self::Settings => "Settings",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum QuickFilter {
    All,
    Pinned,
    Tasks,
    Links,
    Code,
}

impl QuickFilter {
    fn label(self) -> &'static str {
        match self {
            Self::All => "All memos",
            Self::Pinned => "Pinned",
            Self::Tasks => "Tasks",
            Self::Links => "Links",
            Self::Code => "Code",
        }
    }

    fn matches(self, memo: &Memo) -> bool {
        match self {
            Self::All => true,
            Self::Pinned => memo.pinned.unwrap_or(false),
            Self::Tasks => memo
                .property
                .as_ref()
                .and_then(|property| property.has_task_list)
                .unwrap_or(false),
            Self::Links => memo
                .property
                .as_ref()
                .and_then(|property| property.has_link)
                .unwrap_or(false),
            Self::Code => memo
                .property
                .as_ref()
                .and_then(|property| property.has_code)
                .unwrap_or(false),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DetailTab {
    Content,
    Activity,
    Links,
    Share,
    Files,
}

enum ModuleData {
    Views(Vec<memos_api::types::MemoView>),
    Notifications(Vec<memos_api::types::UserNotification>),
    Attachments(Vec<memos_api::types::Attachment>),
}

impl DetailTab {
    fn label(self) -> &'static str {
        match self {
            Self::Content => "Content",
            Self::Activity => "Activity",
            Self::Links => "Links",
            Self::Share => "Share",
            Self::Files => "Files",
        }
    }
}

pub struct MemosDesktop {
    runtime: Arc<Runtime>,
    demo_mode: bool,
    connected: bool,
    auth_mode: AuthMode,
    route: Route,
    quick_filter: QuickFilter,
    loading: bool,
    saving: bool,
    error: Option<String>,
    notice: Option<String>,

    session: Option<ApiSession>,
    instance: Option<InstanceProfile>,
    current_user: Option<User>,
    memos: Vec<Memo>,
    selected_memo_name: Option<String>,
    search_query: String,
    visibility: MemoVisibility,
    detail_tab: DetailTab,
    detail_loading: bool,
    detail_error: Option<String>,
    detail: MemoDetailData,
    memo_views: Vec<memos_api::types::MemoView>,
    notifications: Vec<memos_api::types::UserNotification>,
    library_attachments: Vec<memos_api::types::Attachment>,
    module_loading: bool,

    server_input: Entity<InputState>,
    username_input: Entity<InputState>,
    password_input: Entity<InputState>,
    token_input: Entity<InputState>,
    search_input: Entity<InputState>,
    composer_input: Entity<InputState>,
    comment_input: Entity<InputState>,
    _subscriptions: Vec<Subscription>,
}

impl MemosDesktop {
    pub fn new(demo_mode: bool, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let config = AppConfig::load();
        let server_url = if config.server_url.is_empty() {
            "http://localhost:5230".to_string()
        } else {
            config.server_url
        };

        let server_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("https://memos.example.com")
                .default_value(server_url)
        });
        let username_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Username")
                .default_value(config.username)
        });
        let password_input = cx.new(|cx| {
            InputState::new(window, cx)
                .masked(true)
                .placeholder("Password")
        });
        let token_input = cx.new(|cx| {
            InputState::new(window, cx)
                .masked(true)
                .placeholder("Personal access token")
        });
        let search_input = cx.new(|cx| InputState::new(window, cx).placeholder("Search memos"));
        let composer_input = cx.new(|cx| {
            InputState::new(window, cx)
                .auto_grow(3, 10)
                .placeholder("Capture a thought in Markdown...")
        });
        let comment_input =
            cx.new(|cx| InputState::new(window, cx).placeholder("Add a comment..."));

        let _subscriptions = vec![
            cx.subscribe_in(&search_input, window, Self::on_search_input),
            cx.subscribe_in(&composer_input, window, Self::on_composer_input),
            cx.subscribe_in(&comment_input, window, Self::on_comment_input),
        ];

        let runtime = Arc::new(Runtime::new().expect("failed to create network runtime"));
        let (connected, instance, current_user, memos, selected_memo_name) = if demo_mode {
            let memos = demo::memos();
            let selected = memos.first().and_then(|memo| memo.name.clone());
            (
                true,
                Some(demo::instance()),
                Some(demo::user()),
                memos,
                selected,
            )
        } else {
            (false, None, None, Vec::new(), None)
        };

        Self {
            runtime,
            demo_mode,
            connected,
            auth_mode: AuthMode::Password,
            route: Route::Timeline,
            quick_filter: QuickFilter::All,
            loading: false,
            saving: false,
            error: None,
            notice: None,
            session: None,
            instance,
            current_user,
            memos,
            selected_memo_name,
            search_query: String::new(),
            visibility: MemoVisibility::Private,
            detail_tab: DetailTab::Content,
            detail_loading: false,
            detail_error: None,
            detail: MemoDetailData::default(),
            memo_views: Vec::new(),
            notifications: Vec::new(),
            library_attachments: Vec::new(),
            module_loading: false,
            server_input,
            username_input,
            password_input,
            token_input,
            search_input,
            composer_input,
            comment_input,
            _subscriptions,
        }
    }

    fn on_search_input(
        &mut self,
        state: &Entity<InputState>,
        event: &InputEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if matches!(event, InputEvent::Change) {
            self.search_query = state.read(cx).value().to_string();
            cx.notify();
        }
    }

    fn on_composer_input(
        &mut self,
        _: &Entity<InputState>,
        event: &InputEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if matches!(event, InputEvent::PressEnter { secondary: true }) {
            self.save_memo(window, cx);
        }
    }

    fn on_comment_input(
        &mut self,
        _: &Entity<InputState>,
        event: &InputEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if matches!(event, InputEvent::PressEnter { secondary: true }) {
            self.save_comment(window, cx);
        }
    }

    fn set_detail_tab(&mut self, tab: DetailTab, cx: &mut Context<Self>) {
        self.detail_tab = tab;
        cx.notify();
    }

    fn load_detail(&mut self, memo_name: String, cx: &mut Context<Self>) {
        self.detail = MemoDetailData::default();
        self.detail_error = None;
        self.detail_tab = DetailTab::Content;
        let Some(session) = self.session.clone() else {
            cx.notify();
            return;
        };
        self.detail_loading = true;
        cx.spawn(async move |this, cx| {
            let result = session.load_memo_detail(memo_name).await;
            _ = this.update(cx, |this, cx| {
                this.detail_loading = false;
                match result {
                    Ok(detail) => {
                        this.detail = detail;
                        this.detail_error = None;
                    }
                    Err(error) => this.detail_error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn save_comment(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(memo_name) = self.selected_memo_name.clone() else {
            return;
        };
        let content = self.comment_input.read(cx).value().trim().to_string();
        if content.is_empty() || self.current_user.is_none() {
            return;
        }
        self.saving = true;
        self.detail_error = None;

        if self.demo_mode {
            let mut comment = local_memo(content, MemoVisibility::Private);
            comment.parent = Some(memo_name);
            self.detail.comments.push(comment);
            self.comment_input.update(cx, |input, cx| {
                input.set_value("", window, cx);
            });
            self.saving = false;
            cx.notify();
            return;
        }

        let Some(session) = self.session.clone() else {
            self.saving = false;
            return;
        };
        let comment_input = self.comment_input.clone();
        cx.spawn_in(window, async move |this, window| {
            let result = session.create_memo_comment(memo_name, content).await;
            _ = this.update_in(window, |this, window, cx| {
                this.saving = false;
                match result {
                    Ok(comment) => {
                        this.detail.comments.push(comment);
                        comment_input.update(cx, |input, cx| {
                            input.set_value("", window, cx);
                        });
                        this.detail_error = None;
                    }
                    Err(error) => this.detail_error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn add_reaction(&mut self, reaction_type: String, cx: &mut Context<Self>) {
        let Some(memo_name) = self.selected_memo_name.clone() else {
            return;
        };
        if self.current_user.is_none() {
            return;
        }
        if self.demo_mode {
            self.detail.reactions.push(memos_api::types::Reaction {
                content_id: memo_name.clone(),
                create_time: Some(Utc::now()),
                creator: self
                    .current_user
                    .as_ref()
                    .and_then(|user| user.name.clone()),
                name: Some(format!(
                    "{memo_name}/reactions/{}",
                    self.detail.reactions.len() + 1
                )),
                reaction_type,
            });
            cx.notify();
            return;
        }
        let Some(session) = self.session.clone() else {
            return;
        };
        self.saving = true;
        cx.spawn(async move |this, cx| {
            let result = session.upsert_memo_reaction(memo_name, reaction_type).await;
            _ = this.update(cx, |this, cx| {
                this.saving = false;
                match result {
                    Ok(reaction) => this.detail.reactions.push(reaction),
                    Err(error) => this.detail_error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn create_share(&mut self, cx: &mut Context<Self>) {
        let Some(memo_name) = self.selected_memo_name.clone() else {
            return;
        };
        if self.current_user.is_none() {
            return;
        }
        if self.demo_mode {
            self.detail.shares.push(memos_api::types::MemoShare {
                create_time: Some(Utc::now()),
                expire_time: None,
                name: Some(format!(
                    "{memo_name}/shares/demo-{}",
                    self.detail.shares.len() + 1
                )),
            });
            cx.notify();
            return;
        }
        let Some(session) = self.session.clone() else {
            return;
        };
        self.saving = true;
        cx.spawn(async move |this, cx| {
            let result = session.create_memo_share(memo_name).await;
            _ = this.update(cx, |this, cx| {
                this.saving = false;
                match result {
                    Ok(share) => this.detail.shares.push(share),
                    Err(error) => this.detail_error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn delete_share(&mut self, share_name: String, cx: &mut Context<Self>) {
        let Some(memo_name) = self.selected_memo_name.clone() else {
            return;
        };
        if self.demo_mode {
            self.detail
                .shares
                .retain(|share| share.name.as_deref() != Some(share_name.as_str()));
            cx.notify();
            return;
        }
        let Some(session) = self.session.clone() else {
            return;
        };
        self.saving = true;
        cx.spawn(async move |this, cx| {
            let result = session
                .delete_memo_share(memo_name, share_name.clone())
                .await;
            _ = this.update(cx, |this, cx| {
                this.saving = false;
                match result {
                    Ok(()) => this
                        .detail
                        .shares
                        .retain(|share| share.name.as_deref() != Some(share_name.as_str())),
                    Err(error) => this.detail_error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn upload_attachment(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(memo_name) = self.selected_memo_name.clone() else {
            return;
        };
        let Some(session) = self.session.clone() else {
            self.detail_error = Some("Connect to an instance before uploading files.".into());
            cx.notify();
            return;
        };
        let existing = self.detail.attachments.clone();
        let receiver = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: false,
            multiple: false,
            prompt: Some("Attach file to memo".into()),
        });
        self.saving = true;
        self.detail_error = None;
        cx.spawn_in(window, async move |this, window| {
            let paths = receiver.await.ok()?.ok()??;
            let Some(path) = paths.into_iter().next() else {
                _ = this.update_in(window, |this, _, cx| {
                    this.saving = false;
                    cx.notify();
                });
                return Some(());
            };
            let result = session
                .upload_memo_attachment(memo_name, existing, path)
                .await;
            _ = this.update_in(window, |this, _, cx| {
                this.saving = false;
                match result {
                    Ok(attachment) => this.detail.attachments.push(attachment),
                    Err(error) => this.detail_error = Some(error.to_string()),
                }
                cx.notify();
            });
            Some(())
        })
        .detach();
    }

    fn set_auth_mode(&mut self, mode: AuthMode, cx: &mut Context<Self>) {
        self.auth_mode = mode;
        self.error = None;
        cx.notify();
    }

    fn authenticate(&mut self, anonymous: bool, cx: &mut Context<Self>) {
        if self.loading {
            return;
        }

        let server_url = self.server_input.read(cx).value().to_string();
        let username = self.username_input.read(cx).value().to_string();
        let password = self.password_input.read(cx).value().to_string();
        let token = self.token_input.read(cx).value().to_string();
        let auth_mode = self.auth_mode;
        let runtime = self.runtime.clone();

        self.loading = true;
        self.error = None;
        self.notice = Some("Connecting to instance...".into());
        cx.notify();

        cx.spawn(async move |this, cx| {
            let result = async {
                let session = ApiSession::new(&server_url, runtime)?;
                let profile = session.instance_profile().await?;
                let user = if anonymous {
                    None
                } else {
                    Some(match auth_mode {
                        AuthMode::Password => {
                            session.sign_in_password(username.clone(), password).await?
                        }
                        AuthMode::AccessToken => session.sign_in_with_access_token(token).await?,
                    })
                };
                let memos = session.list_memos(None, false).await?;
                Ok::<_, crate::api::ApiError>((session, profile, user, memos))
            }
            .await;

            _ = this.update(cx, |this, cx| {
                this.loading = false;
                this.notice = None;
                match result {
                    Ok((session, profile, user, memos)) => {
                        this.connected = true;
                        this.error = None;
                        this.selected_memo_name = memos.first().and_then(|memo| memo.name.clone());
                        this.memos = memos;
                        this.instance = Some(profile);
                        this.current_user = user;
                        this.session = Some(session);
                        this.route = Route::Timeline;
                        if let Some(name) = this.selected_memo_name.clone() {
                            this.load_detail(name, cx);
                        }
                        let _ = AppConfig {
                            server_url,
                            username,
                        }
                        .save();
                    }
                    Err(error) => {
                        this.error = Some(error.to_string());
                    }
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn disconnect(&mut self, cx: &mut Context<Self>) {
        if let Some(session) = self.session.take() {
            cx.spawn(async move |_, _| {
                let _ = session.sign_out().await;
            })
            .detach();
        }
        self.connected = false;
        self.instance = None;
        self.current_user = None;
        self.memos.clear();
        self.selected_memo_name = None;
        self.error = None;
        self.notice = None;
        cx.notify();
    }

    fn navigate(&mut self, route: Route, cx: &mut Context<Self>) {
        self.route = route;
        self.quick_filter = QuickFilter::All;
        self.error = None;
        if self.demo_mode {
            cx.notify();
            return;
        }
        match route {
            Route::Timeline | Route::Archive | Route::Explore => self.reload_memos(cx),
            Route::Views | Route::Attachments | Route::Inbox => self.load_module_data(cx),
            Route::Settings => cx.notify(),
        }
    }

    fn load_module_data(&mut self, cx: &mut Context<Self>) {
        let Some(session) = self.session.clone() else {
            cx.notify();
            return;
        };
        let user_name = self
            .current_user
            .as_ref()
            .and_then(|user| user.name.clone());
        let route = self.route;
        self.module_loading = true;
        cx.spawn(async move |this, cx| {
            let result: Result<ModuleData, crate::api::ApiError> = match route {
                Route::Views => match user_name {
                    Some(user_name) => session
                        .list_memo_views(user_name)
                        .await
                        .map(ModuleData::Views),
                    None => Err(crate::api::ApiError::MissingField("current user name")),
                },
                Route::Inbox => match user_name {
                    Some(user_name) => session
                        .list_notifications(user_name)
                        .await
                        .map(ModuleData::Notifications),
                    None => Err(crate::api::ApiError::MissingField("current user name")),
                },
                Route::Attachments => session
                    .list_attachments()
                    .await
                    .map(ModuleData::Attachments),
                _ => unreachable!("module loader called for timeline route"),
            };
            _ = this.update(cx, |this, cx| {
                this.module_loading = false;
                match result {
                    Ok(ModuleData::Views(views)) => this.memo_views = views,
                    Ok(ModuleData::Notifications(notifications)) => {
                        this.notifications = notifications
                    }
                    Ok(ModuleData::Attachments(attachments)) => {
                        this.library_attachments = attachments
                    }
                    Err(error) => this.error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn reload_memos(&mut self, cx: &mut Context<Self>) {
        let Some(session) = self.session.clone() else {
            cx.notify();
            return;
        };
        let archived = self.route == Route::Archive;
        let filter =
            (self.route == Route::Explore).then(|| r#"visibility == "PUBLIC""#.to_string());
        self.loading = true;
        cx.notify();

        cx.spawn(async move |this, cx| {
            let result = session.list_memos(filter, archived).await;
            _ = this.update(cx, |this, cx| {
                this.loading = false;
                match result {
                    Ok(memos) => {
                        this.selected_memo_name = memos.first().and_then(|memo| memo.name.clone());
                        this.memos = memos;
                        if let Some(name) = this.selected_memo_name.clone() {
                            this.load_detail(name, cx);
                        }
                        this.error = None;
                    }
                    Err(error) => this.error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn set_quick_filter(&mut self, filter: QuickFilter, cx: &mut Context<Self>) {
        self.quick_filter = filter;
        cx.notify();
    }

    fn set_visibility(&mut self, visibility: MemoVisibility, cx: &mut Context<Self>) {
        self.visibility = visibility;
        cx.notify();
    }

    fn select_memo(&mut self, name: Option<String>, cx: &mut Context<Self>) {
        self.selected_memo_name = name.clone();
        if let Some(name) = name {
            if !self.demo_mode {
                self.load_detail(name, cx);
            } else {
                self.detail = MemoDetailData::default();
                self.detail_error = None;
                self.detail_tab = DetailTab::Content;
            }
        } else {
            self.detail = MemoDetailData::default();
            self.detail_error = None;
        }
        cx.notify();
    }

    fn save_memo(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.saving || self.current_user.is_none() {
            return;
        }
        let content = self.composer_input.read(cx).value().trim().to_string();
        if content.is_empty() {
            self.error = Some("Write something before saving the memo.".into());
            cx.notify();
            return;
        }

        let visibility = self.visibility;
        self.saving = true;
        self.error = None;
        cx.notify();

        if self.demo_mode {
            let memo = local_memo(content, visibility);
            self.memos.insert(0, memo.clone());
            self.selected_memo_name = memo.name;
            self.composer_input.update(cx, |input, cx| {
                input.set_value("", window, cx);
            });
            self.saving = false;
            cx.notify();
            return;
        }

        let Some(session) = self.session.clone() else {
            self.saving = false;
            return;
        };
        let composer = self.composer_input.clone();
        cx.spawn_in(window, async move |this, window| {
            let result = session.create_memo(content, visibility).await;
            _ = this.update_in(window, |this, window, cx| {
                this.saving = false;
                match result {
                    Ok(memo) => {
                        this.selected_memo_name = memo.name.clone();
                        this.memos.insert(0, memo);
                        this.error = None;
                        composer.update(cx, |input, cx| {
                            input.set_value("", window, cx);
                        });
                    }
                    Err(error) => this.error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn toggle_pin(&mut self, memo_name: String, cx: &mut Context<Self>) {
        let Some(index) = self.memo_index(&memo_name) else {
            return;
        };
        let mut memo = self.memos[index].clone();
        memo.pinned = Some(!memo.pinned.unwrap_or(false));
        self.update_memo(index, memo, "pinned", cx);
    }

    fn toggle_archive(&mut self, memo_name: String, cx: &mut Context<Self>) {
        let Some(index) = self.memo_index(&memo_name) else {
            return;
        };
        let mut memo = self.memos[index].clone();
        memo.state = if memo.state == MemoState::Archived {
            MemoState::Normal
        } else {
            MemoState::Archived
        };
        self.update_memo(index, memo, "state", cx);
    }

    fn update_memo(
        &mut self,
        index: usize,
        memo: Memo,
        update_mask: &'static str,
        cx: &mut Context<Self>,
    ) {
        if self.demo_mode {
            if update_mask == "state" {
                self.memos.remove(index);
                self.selected_memo_name = self.memos.first().and_then(|memo| memo.name.clone());
            } else {
                self.memos[index] = memo;
            }
            cx.notify();
            return;
        }

        let Some(session) = self.session.clone() else {
            return;
        };
        self.saving = true;
        cx.notify();
        cx.spawn(async move |this, cx| {
            let result = session.update_memo(memo, update_mask.into()).await;
            _ = this.update(cx, |this, cx| {
                this.saving = false;
                match result {
                    Ok(updated) => {
                        if update_mask == "state" {
                            if index < this.memos.len() {
                                this.memos.remove(index);
                            }
                            this.selected_memo_name =
                                this.memos.first().and_then(|memo| memo.name.clone());
                        } else if index < this.memos.len() {
                            this.memos[index] = updated;
                        }
                        this.error = None;
                    }
                    Err(error) => this.error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn delete_memo(&mut self, memo_name: String, cx: &mut Context<Self>) {
        let Some(index) = self.memo_index(&memo_name) else {
            return;
        };
        if self.demo_mode {
            self.memos.remove(index);
            self.selected_memo_name = self.memos.first().and_then(|memo| memo.name.clone());
            cx.notify();
            return;
        }

        let Some(session) = self.session.clone() else {
            return;
        };
        self.saving = true;
        cx.notify();
        cx.spawn(async move |this, cx| {
            let result = session.delete_memo(memo_name).await;
            _ = this.update(cx, |this, cx| {
                this.saving = false;
                match result {
                    Ok(()) => {
                        if index < this.memos.len() {
                            this.memos.remove(index);
                        }
                        this.selected_memo_name =
                            this.memos.first().and_then(|memo| memo.name.clone());
                        this.error = None;
                    }
                    Err(error) => this.error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn memo_index(&self, name: &str) -> Option<usize> {
        self.memos
            .iter()
            .position(|memo| memo.name.as_deref() == Some(name))
    }

    fn selected_memo(&self) -> Option<&Memo> {
        let selected = self.selected_memo_name.as_deref()?;
        self.memos
            .iter()
            .find(|memo| memo.name.as_deref() == Some(selected))
    }

    fn visible_memos(&self) -> Vec<Memo> {
        let query = self.search_query.trim().to_lowercase();
        self.memos
            .iter()
            .filter(|memo| self.quick_filter.matches(memo))
            .filter(|memo| {
                self.route != Route::Explore || memo.visibility == MemoVisibility::Public
            })
            .filter(|memo| {
                query.is_empty()
                    || memo.content.to_lowercase().contains(&query)
                    || memo
                        .tags
                        .iter()
                        .any(|tag| tag.to_lowercase().contains(&query))
            })
            .cloned()
            .collect()
    }
}

impl Render for MemosDesktop {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let element: AnyElement = if self.connected {
            self.render_workspace(window, cx)
        } else {
            self.render_auth(window, cx)
        };
        element
    }
}

fn local_memo(content: String, visibility: MemoVisibility) -> Memo {
    let now = Utc::now();
    let tags = content
        .split_whitespace()
        .filter_map(|word| word.strip_prefix('#'))
        .map(|tag| {
            tag.trim_matches(|character: char| {
                !character.is_alphanumeric() && character != '-' && character != '_'
            })
        })
        .filter(|tag| !tag.is_empty())
        .map(str::to_string)
        .collect::<Vec<_>>();
    Memo {
        attachments: Vec::new(),
        content: content.clone(),
        create_time: Some(now),
        creator: Some("users/demo".into()),
        location: None,
        name: Some(format!("memos/local-{}", now.timestamp_millis())),
        parent: None,
        pinned: Some(false),
        property: Some(MemoProperty {
            has_code: Some(content.contains("```")),
            has_incomplete_tasks: Some(content.contains("- [ ]")),
            has_link: Some(content.contains("http://") || content.contains("https://")),
            has_task_list: Some(content.contains("- [")),
            title: content
                .lines()
                .next()
                .and_then(|line| line.strip_prefix("# "))
                .map(str::to_string),
        }),
        reactions: Vec::new(),
        relations: Vec::new(),
        snippet: content.lines().next().map(str::to_string),
        state: MemoState::Normal,
        tags,
        update_time: Some(now),
        visibility,
    }
}
