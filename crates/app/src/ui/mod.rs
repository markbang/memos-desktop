mod auth;
mod workspace;

use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use chrono::Utc;
use gpui::{
    AnyElement, AppContext, Context, Entity, IntoElement, PathPromptOptions, Render, Subscription,
    Window,
};
use gpui_component::input::{InputEvent, InputState};
use memos_api::types::{
    Attachment, IdentityProvider, InstanceProfile, InstanceSetting, InstanceStats, LinkMetadata,
    LinkedIdentity, ListAllUserStatsResponse, Memo, MemoProperty, MemoState, MemoVisibility,
    PersonalAccessToken, Shortcut, User, UserRole, UserSetting, UserState, UserStats, UserWebhook,
};
use tokio::runtime::Runtime;

use crate::{
    api::{ApiSession, MemoDetailData},
    config::AppConfig,
    credentials, demo,
    theme::{self, ThemePreference},
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
    Profile,
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
            Self::Profile => "Profile",
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
    Views(Vec<Shortcut>),
    Notifications(Vec<memos_api::types::UserNotification>, Option<String>),
    Attachments(Vec<Attachment>, HashMap<String, PathBuf>, Option<String>),
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum SettingsSection {
    #[default]
    Account,
    Preferences,
    Tokens,
    Webhooks,
    Administration,
}

#[derive(Default)]
struct AccountResources {
    settings: Vec<UserSetting>,
    identities: Vec<LinkedIdentity>,
    tokens: Vec<PersonalAccessToken>,
    webhooks: Vec<UserWebhook>,
    stats: Option<UserStats>,
}

#[derive(Default)]
struct AdminResources {
    users: Vec<User>,
    user_stats: Option<ListAllUserStatsResponse>,
    instance_settings: Vec<InstanceSetting>,
    instance_stats: Option<InstanceStats>,
    identity_providers: Vec<IdentityProvider>,
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
    live_cancel: Option<Arc<AtomicBool>>,
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
    saved_login_available: bool,
    known_users: HashMap<String, User>,
    user_avatars: HashMap<String, PathBuf>,
    profile_user: Option<User>,
    profile_stats: Option<UserStats>,
    memos: Vec<Memo>,
    next_memo_page_token: Option<String>,
    active_server_filter: Option<String>,
    selected_memo_name: Option<String>,
    search_query: String,
    visibility: MemoVisibility,
    detail_tab: DetailTab,
    detail_loading: bool,
    detail_error: Option<String>,
    detail: MemoDetailData,
    link_metadata: HashMap<String, LinkMetadata>,
    attachment_previews: HashMap<String, PathBuf>,
    memo_views: Vec<Shortcut>,
    notifications: Vec<memos_api::types::UserNotification>,
    next_notification_page_token: Option<String>,
    library_attachments: Vec<Attachment>,
    next_attachment_page_token: Option<String>,
    account_resources: AccountResources,
    admin_resources: AdminResources,
    settings_section: SettingsSection,
    theme_preference: ThemePreference,
    module_loading: bool,
    loading_more: bool,

    server_input: Entity<InputState>,
    username_input: Entity<InputState>,
    password_input: Entity<InputState>,
    token_input: Entity<InputState>,
    shared_link_input: Entity<InputState>,
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
            config.server_url.clone()
        };

        let server_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("https://memos.example.com")
                .default_value(server_url)
        });
        let username_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Username")
                .default_value(config.username.clone())
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
        let shared_link_input =
            cx.new(|cx| InputState::new(window, cx).placeholder("Shared memo URL or token"));
        let search_input = cx.new(|cx| InputState::new(window, cx).placeholder("Search memos"));
        let composer_input = cx.new(|cx| {
            InputState::new(window, cx)
                .auto_grow(3, 10)
                .placeholder("Capture a thought in Markdown...")
        });
        let comment_input =
            cx.new(|cx| InputState::new(window, cx).placeholder("Add a comment..."));

        let mut _subscriptions = vec![
            cx.subscribe_in(&search_input, window, Self::on_search_input),
            cx.subscribe_in(&composer_input, window, Self::on_composer_input),
            cx.subscribe_in(&comment_input, window, Self::on_comment_input),
        ];
        _subscriptions.push(cx.observe_window_appearance(window, |this, window, cx| {
            if this.theme_preference == ThemePreference::System {
                theme::apply(ThemePreference::System, Some(window), cx);
                cx.notify();
            }
        }));

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

        let mut desktop = Self {
            runtime,
            live_cancel: None,
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
            saved_login_available: false,
            known_users: HashMap::new(),
            user_avatars: HashMap::new(),
            profile_user: None,
            profile_stats: None,
            memos,
            next_memo_page_token: None,
            active_server_filter: None,
            selected_memo_name,
            search_query: String::new(),
            visibility: MemoVisibility::Private,
            detail_tab: DetailTab::Content,
            detail_loading: false,
            detail_error: None,
            detail: MemoDetailData::default(),
            link_metadata: HashMap::new(),
            attachment_previews: HashMap::new(),
            memo_views: Vec::new(),
            notifications: Vec::new(),
            next_notification_page_token: None,
            library_attachments: Vec::new(),
            next_attachment_page_token: None,
            account_resources: AccountResources::default(),
            admin_resources: AdminResources::default(),
            settings_section: SettingsSection::default(),
            theme_preference: config.theme,
            module_loading: false,
            loading_more: false,
            server_input,
            username_input,
            password_input,
            token_input,
            shared_link_input,
            search_input,
            composer_input,
            comment_input,
            _subscriptions,
        };
        if !demo_mode && config.auto_login {
            desktop.start_auto_login(config, cx);
        }
        desktop
    }

    fn start_auto_login(&mut self, config: AppConfig, cx: &mut Context<Self>) {
        if config.server_url.trim().is_empty() || config.username.trim().is_empty() {
            return;
        }
        let server_url = config.server_url;
        let username = config.username;
        let runtime = self.runtime.clone();
        self.loading = true;
        self.notice = Some("Signing in with saved credentials...".into());
        cx.spawn(async move |this, cx| {
            let password = runtime
                .spawn_blocking({
                    let server_url = server_url.clone();
                    let username = username.clone();
                    move || credentials::load_password(&server_url, &username)
                })
                .await
                .map_err(|error| error.to_string())
                .and_then(|result| result);
            let saved_password_found = matches!(&password, Ok(Some(_)));
            let mut missing_saved_password = false;
            let result = match password {
                Ok(Some(password)) => {
                    let session = ApiSession::new(&server_url, runtime.clone());
                    match session {
                        Ok(session) => async {
                            let profile = session.instance_profile().await?;
                            let user = session.sign_in_password(username.clone(), password).await?;
                            let filter = user
                                .name
                                .as_deref()
                                .map(|name| format!("creator == \"{}\"", escape_cel_string(name)));
                            let response = session
                                .list_memos_page(
                                    filter,
                                    Some("pinned desc, create_time desc".into()),
                                    50,
                                    None,
                                    false,
                                )
                                .await?;
                            Ok::<_, crate::api::ApiError>((session, profile, user, response))
                        }
                        .await
                        .map_err(|error| error.to_string()),
                        Err(error) => Err(error.to_string()),
                    }
                }
                Ok(None) => {
                    missing_saved_password = true;
                    Err("No saved password was found.".into())
                }
                Err(error) => Err(format!(
                    "Could not read the system credential store: {error}"
                )),
            };

            _ = this.update(cx, |this, cx| {
                this.loading = false;
                this.notice = None;
                this.saved_login_available = saved_password_found;
                match result {
                    Ok((session, profile, user, response)) => {
                        this.saved_login_available = true;
                        this.apply_connected_session(session, profile, Some(user), response, cx);
                    }
                    Err(error) => {
                        if missing_saved_password {
                            this.persist_connection(server_url.clone(), username.clone(), false);
                        }
                        this.error = Some(format!("Automatic sign-in failed. {error}"));
                    }
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn apply_connected_session(
        &mut self,
        session: ApiSession,
        profile: InstanceProfile,
        user: Option<User>,
        response: memos_api::types::ListMemosResponse,
        cx: &mut Context<Self>,
    ) {
        self.connected = true;
        self.error = None;
        self.next_memo_page_token = non_empty(response.next_page_token);
        self.selected_memo_name = response.memos.first().and_then(|memo| memo.name.clone());
        self.memos = response.memos;
        self.instance = Some(profile);
        self.current_user = user;
        self.session = Some(session);
        if let Some(user) = self.current_user.clone()
            && let Some(name) = user.name.clone()
        {
            self.known_users.insert(name, user);
            self.start_live_updates(cx);
        }
        self.route = Route::Timeline;
        if let Some(name) = self.selected_memo_name.clone() {
            self.load_detail(name, cx);
        }
        self.refresh_memo_assets(cx);
    }

    fn refresh_memo_assets(&mut self, cx: &mut Context<Self>) {
        let Some(session) = self.session.clone() else {
            return;
        };
        let request_session = session.clone();
        let creators = self
            .memos
            .iter()
            .filter_map(|memo| memo.creator.clone())
            .chain(
                self.current_user
                    .as_ref()
                    .and_then(|user| user.name.clone()),
            )
            .collect::<Vec<_>>();
        let profile_users = self.profile_user.iter().cloned().collect::<Vec<_>>();
        let attachments = self
            .memos
            .iter()
            .take(24)
            .flat_map(|memo| {
                memo.attachments
                    .iter()
                    .filter(|attachment| is_previewable_image(attachment))
                    .take(6)
                    .cloned()
            })
            .chain(
                self.detail
                    .attachments
                    .iter()
                    .filter(|attachment| is_previewable_image(attachment))
                    .cloned(),
            )
            .chain(
                self.library_attachments
                    .iter()
                    .filter(|attachment| is_previewable_image(attachment))
                    .cloned(),
            )
            .take(120)
            .collect::<Vec<_>>();
        cx.spawn(async move |this, cx| {
            let mut users = session.batch_get_users(creators).await.unwrap_or_default();
            users.extend(profile_users);
            users.sort_by(|left, right| left.name.cmp(&right.name));
            users.dedup_by(|left, right| left.name == right.name);
            let avatars = cache_user_avatars(&session, &users).await;
            let previews = cache_attachment_previews(&session, &attachments).await;
            _ = this.update(cx, |this, cx| {
                if !this
                    .session
                    .as_ref()
                    .is_some_and(|session| session.same_session(&request_session))
                {
                    return;
                }
                for user in users {
                    if let Some(name) = user.name.clone() {
                        this.known_users.insert(name, user);
                    }
                }
                this.user_avatars.extend(avatars);
                this.attachment_previews.extend(previews);
                cx.notify();
            });
        })
        .detach();
    }

    fn persist_connection(&self, server_url: String, username: String, auto_login: bool) {
        let _ = AppConfig {
            server_url,
            username,
            auto_login,
            theme: self.theme_preference,
        }
        .save();
    }

    fn remember_password(
        &mut self,
        server_url: String,
        username: String,
        password: String,
        cx: &mut Context<Self>,
    ) {
        let runtime = self.runtime.clone();
        cx.spawn(async move |this, cx| {
            let result = runtime
                .spawn_blocking({
                    let server_url = server_url.clone();
                    let username = username.clone();
                    move || credentials::save_password(&server_url, &username, &password)
                })
                .await
                .map_err(|error| error.to_string())
                .and_then(|result| result);
            _ = this.update(cx, |this, cx| {
                match result {
                    Ok(()) => {
                        this.saved_login_available = true;
                        this.persist_connection(server_url, username, true);
                    }
                    Err(error) => {
                        this.saved_login_available = false;
                        this.persist_connection(server_url, username, false);
                        this.notice = Some(format!(
                            "Signed in, but the system credential store could not save the password: {error}"
                        ));
                    }
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn forget_saved_login(&mut self, cx: &mut Context<Self>) {
        let server_url = self
            .session
            .as_ref()
            .map(|session| session.base_url().to_string())
            .unwrap_or_else(|| self.server_input.read(cx).value().to_string());
        let username = self
            .current_user
            .as_ref()
            .map(|user| user.username.clone())
            .unwrap_or_else(|| self.username_input.read(cx).value().to_string());
        self.persist_connection(server_url.clone(), username.clone(), false);
        let runtime = self.runtime.clone();
        cx.spawn(async move |this, cx| {
            let result = runtime
                .spawn_blocking(move || credentials::delete_password(&server_url, &username))
                .await
                .map_err(|error| error.to_string())
                .and_then(|result| result);
            _ = this.update(cx, |this, cx| {
                this.saved_login_available = false;
                this.notice = Some(match result {
                    Ok(()) => "Saved login removed from the system credential store.".into(),
                    Err(error) => format!("Could not remove the saved login: {error}"),
                });
                cx.notify();
            });
        })
        .detach();
    }

    fn set_theme_preference(
        &mut self,
        preference: ThemePreference,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.theme_preference = preference;
        theme::apply(preference, Some(window), cx);
        let server_url = self
            .session
            .as_ref()
            .map(|session| session.base_url().to_string())
            .unwrap_or_else(|| self.server_input.read(cx).value().to_string());
        let username = self
            .current_user
            .as_ref()
            .map(|user| user.username.clone())
            .unwrap_or_else(|| self.username_input.read(cx).value().to_string());
        let auto_login = AppConfig::load().auto_login;
        self.persist_connection(server_url, username, auto_login);
        cx.notify();
    }

    fn on_search_input(
        &mut self,
        state: &Entity<InputState>,
        event: &InputEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            InputEvent::Change => {
                self.search_query = state.read(cx).value().to_string();
                cx.notify();
            }
            InputEvent::PressEnter { .. } if !self.demo_mode => self.reload_memos(cx),
            _ => {}
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
        let urls = self
            .memo_index(&memo_name)
            .map(|index| extract_http_urls(&self.memos[index].content))
            .unwrap_or_default();
        self.detail = MemoDetailData::default();
        self.link_metadata.clear();
        self.detail_error = None;
        self.detail_tab = DetailTab::Content;
        let Some(session) = self.session.clone() else {
            cx.notify();
            return;
        };
        let requested_memo_name = memo_name.clone();
        let request_session = session.clone();
        self.detail_loading = true;
        cx.spawn(async move |this, cx| {
            let result = async {
                let detail = session.load_memo_detail(memo_name).await?;
                let links = if urls.is_empty() {
                    Vec::new()
                } else {
                    session
                        .batch_get_link_metadata(urls)
                        .await
                        .unwrap_or_default()
                };
                let previews = cache_attachment_previews(&session, &detail.attachments).await;
                Ok::<_, crate::api::ApiError>((detail, links, previews))
            }
            .await;
            _ = this.update(cx, |this, cx| {
                let same_session = this
                    .session
                    .as_ref()
                    .is_some_and(|session| session.same_session(&request_session));
                if !same_session
                    || this.selected_memo_name.as_deref() != Some(requested_memo_name.as_str())
                {
                    return;
                }
                this.detail_loading = false;
                match result {
                    Ok((detail, links, previews)) => {
                        this.detail = detail;
                        this.attachment_previews.extend(previews);
                        this.link_metadata = links
                            .into_iter()
                            .filter_map(|metadata| metadata.url.clone().map(|url| (url, metadata)))
                            .collect();
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

    fn remove_reaction(&mut self, reaction_name: String, cx: &mut Context<Self>) {
        if self.demo_mode {
            self.detail
                .reactions
                .retain(|reaction| reaction.name.as_deref() != Some(reaction_name.as_str()));
            cx.notify();
            return;
        }
        let Some(session) = self.session.clone() else {
            return;
        };
        self.saving = true;
        cx.spawn(async move |this, cx| {
            let result = session.delete_memo_reaction(reaction_name.clone()).await;
            _ = this.update(cx, |this, cx| {
                this.saving = false;
                match result {
                    Ok(()) => this.detail.reactions.retain(|reaction| {
                        reaction.name.as_deref() != Some(reaction_name.as_str())
                    }),
                    Err(error) => this.detail_error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn delete_comment(&mut self, comment_name: String, cx: &mut Context<Self>) {
        if self.demo_mode {
            self.detail
                .comments
                .retain(|comment| comment.name.as_deref() != Some(comment_name.as_str()));
            cx.notify();
            return;
        }
        let Some(session) = self.session.clone() else {
            return;
        };
        self.saving = true;
        cx.spawn(async move |this, cx| {
            let result = session.delete_memo(comment_name.clone()).await;
            _ = this.update(cx, |this, cx| {
                this.saving = false;
                match result {
                    Ok(()) => this
                        .detail
                        .comments
                        .retain(|comment| comment.name.as_deref() != Some(comment_name.as_str())),
                    Err(error) => this.detail_error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn add_relation(&mut self, related_memo_name: String, cx: &mut Context<Self>) {
        let Some(memo_name) = self.selected_memo_name.clone() else {
            return;
        };
        let related_memo_name = if related_memo_name.starts_with("memos/") {
            related_memo_name
        } else {
            format!("memos/{related_memo_name}")
        };
        let relation = memos_api::types::MemoRelation {
            memo: memos_api::types::MemoRelationMemo {
                name: memo_name.clone(),
                snippet: None,
            },
            related_memo: memos_api::types::MemoRelationMemo {
                name: related_memo_name,
                snippet: None,
            },
            type_: memos_api::types::MemoRelationType::Reference,
        };
        let mut relations = self.detail.relations.clone();
        relations.push(relation);
        self.persist_relations(memo_name, relations, cx);
    }

    fn remove_relation(&mut self, index: usize, cx: &mut Context<Self>) {
        let Some(memo_name) = self.selected_memo_name.clone() else {
            return;
        };
        if index >= self.detail.relations.len() {
            return;
        }
        let mut relations = self.detail.relations.clone();
        relations.remove(index);
        self.persist_relations(memo_name, relations, cx);
    }

    fn persist_relations(
        &mut self,
        memo_name: String,
        relations: Vec<memos_api::types::MemoRelation>,
        cx: &mut Context<Self>,
    ) {
        if self.demo_mode {
            self.detail.relations = relations;
            cx.notify();
            return;
        }
        let Some(session) = self.session.clone() else {
            return;
        };
        self.saving = true;
        cx.spawn(async move |this, cx| {
            let result = session
                .set_memo_relations(memo_name, relations.clone())
                .await;
            _ = this.update(cx, |this, cx| {
                this.saving = false;
                match result {
                    Ok(()) => this.detail.relations = relations,
                    Err(error) => this.detail_error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn create_share_with_ttl(&mut self, ttl_hours: Option<i64>, cx: &mut Context<Self>) {
        let Some(memo_name) = self.selected_memo_name.clone() else {
            return;
        };
        if self.current_user.is_none() {
            return;
        }
        if self.demo_mode {
            self.detail.shares.push(memos_api::types::MemoShare {
                create_time: Some(Utc::now()),
                expire_time: ttl_hours.map(|hours| Utc::now() + chrono::Duration::hours(hours)),
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
            let expire_time = ttl_hours.map(|hours| Utc::now() + chrono::Duration::hours(hours));
            let result = session
                .create_memo_share_with_expiry(memo_name, expire_time)
                .await;
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

    fn open_attachment_resource(&mut self, attachment: Attachment, cx: &mut Context<Self>) {
        if let Some(url) = external_attachment_url(&attachment) {
            cx.open_url(&url);
            return;
        }
        let Some(session) = self.session.clone() else {
            return;
        };
        self.saving = true;
        cx.spawn(async move |this, cx| {
            let result = session.cache_attachment(attachment, false).await;
            _ = this.update(cx, |this, cx| {
                this.saving = false;
                match result {
                    Ok(path) => match url::Url::from_file_path(path) {
                        Ok(url) => cx.open_url(url.as_str()),
                        Err(()) => {
                            this.error = Some("Unable to open the cached attachment.".into())
                        }
                    },
                    Err(error) => this.error = Some(error.to_string()),
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
            multiple: true,
            prompt: Some("Attach file to memo".into()),
        });
        self.saving = true;
        self.detail_error = None;
        cx.spawn_in(window, async move |this, window| {
            let paths = match receiver.await {
                Ok(Ok(Some(paths))) => paths,
                _ => {
                    _ = this.update_in(window, |this, _, cx| {
                        this.saving = false;
                        cx.notify();
                    });
                    return Some(());
                }
            };
            if paths.is_empty() {
                _ = this.update_in(window, |this, _, cx| {
                    this.saving = false;
                    cx.notify();
                });
                return Some(());
            }
            let mut attachments = existing;
            let mut error = None;
            for path in paths {
                match session
                    .upload_memo_attachment(memo_name.clone(), attachments.clone(), path)
                    .await
                {
                    Ok(attachment) => attachments.push(attachment),
                    Err(upload_error) => {
                        error = Some(upload_error.to_string());
                        break;
                    }
                }
            }
            let previews = cache_attachment_previews(&session, &attachments).await;
            _ = this.update_in(window, |this, _, cx| {
                this.saving = false;
                this.detail.attachments = attachments;
                this.attachment_previews.extend(previews);
                this.detail_error = error;
                cx.notify();
            });
            Some(())
        })
        .detach();
    }

    fn upload_library_attachments(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(session) = self.session.clone() else {
            return;
        };
        let receiver = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: false,
            multiple: true,
            prompt: Some("Upload files to attachment library".into()),
        });
        self.module_loading = true;
        cx.spawn_in(window, async move |this, window| {
            let paths = match receiver.await {
                Ok(Ok(Some(paths))) => paths,
                _ => {
                    _ = this.update_in(window, |this, _, cx| {
                        this.module_loading = false;
                        cx.notify();
                    });
                    return Some(());
                }
            };
            let mut uploaded = Vec::new();
            let mut error = None;
            for path in paths {
                match session.upload_attachment_file(path).await {
                    Ok(attachment) => uploaded.push(attachment),
                    Err(upload_error) => {
                        error = Some(upload_error.to_string());
                        break;
                    }
                }
            }
            let previews = cache_attachment_previews(&session, &uploaded).await;
            _ = this.update_in(window, |this, _, cx| {
                this.module_loading = false;
                this.library_attachments.extend(uploaded);
                this.attachment_previews.extend(previews);
                this.error = error;
                cx.notify();
            });
            Some(())
        })
        .detach();
    }

    fn save_attachment_resource(&mut self, attachment: Attachment, cx: &mut Context<Self>) {
        if self.demo_mode {
            upsert_named(&mut self.library_attachments, attachment.clone(), |item| {
                item.name.as_deref()
            });
            upsert_named(&mut self.detail.attachments, attachment, |item| {
                item.name.as_deref()
            });
            cx.notify();
            return;
        }
        let Some(session) = self.session.clone() else {
            return;
        };
        self.module_loading = true;
        cx.spawn(async move |this, cx| {
            let result = session
                .update_attachment(attachment, "filename,external_link,memo".into())
                .await;
            _ = this.update(cx, |this, cx| {
                this.module_loading = false;
                match result {
                    Ok(attachment) => {
                        upsert_named(&mut this.library_attachments, attachment.clone(), |item| {
                            item.name.as_deref()
                        });
                        upsert_named(&mut this.detail.attachments, attachment, |item| {
                            item.name.as_deref()
                        });
                    }
                    Err(error) => this.error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn delete_attachment_resource(&mut self, attachment_name: String, cx: &mut Context<Self>) {
        if self.demo_mode {
            self.detail
                .attachments
                .retain(|attachment| attachment.name.as_deref() != Some(attachment_name.as_str()));
            self.library_attachments
                .retain(|attachment| attachment.name.as_deref() != Some(attachment_name.as_str()));
            self.attachment_previews.remove(&attachment_name);
            cx.notify();
            return;
        }
        let Some(session) = self.session.clone() else {
            return;
        };
        self.saving = true;
        cx.spawn(async move |this, cx| {
            let result = session.delete_attachment(attachment_name.clone()).await;
            _ = this.update(cx, |this, cx| {
                this.saving = false;
                match result {
                    Ok(()) => {
                        this.detail.attachments.retain(|attachment| {
                            attachment.name.as_deref() != Some(attachment_name.as_str())
                        });
                        this.library_attachments.retain(|attachment| {
                            attachment.name.as_deref() != Some(attachment_name.as_str())
                        });
                        this.attachment_previews.remove(&attachment_name);
                    }
                    Err(error) => this.detail_error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn attach_external_link(
        &mut self,
        filename: String,
        url: String,
        mime_type: String,
        cx: &mut Context<Self>,
    ) {
        let Some(memo_name) = self.selected_memo_name.clone() else {
            return;
        };
        if self.demo_mode {
            self.detail.attachments.push(Attachment {
                content: None,
                create_time: Some(Utc::now()),
                external_link: Some(url),
                filename,
                memo: Some(memo_name),
                motion_media: None,
                name: Some(format!(
                    "attachments/demo-{}",
                    self.detail.attachments.len() + 1
                )),
                size: None,
                type_: mime_type,
            });
            cx.notify();
            return;
        }
        let Some(session) = self.session.clone() else {
            return;
        };
        let mut attachments = self.detail.attachments.clone();
        self.saving = true;
        cx.spawn(async move |this, cx| {
            let result = async {
                let attachment = session
                    .create_external_attachment(filename, url, mime_type)
                    .await?;
                attachments.push(attachment);
                session
                    .set_memo_attachments(memo_name, attachments.clone())
                    .await?;
                Ok::<_, crate::api::ApiError>(attachments)
            }
            .await;
            _ = this.update(cx, |this, cx| {
                this.saving = false;
                match result {
                    Ok(attachments) => this.detail.attachments = attachments,
                    Err(error) => this.detail_error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn transcribe_into_composer(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(session) = self.session.clone() else {
            return;
        };
        let receiver = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: false,
            multiple: false,
            prompt: Some("Select audio to transcribe".into()),
        });
        let composer = self.composer_input.clone();
        self.saving = true;
        cx.spawn_in(window, async move |this, window| {
            let paths = match receiver.await {
                Ok(Ok(Some(paths))) => paths,
                _ => {
                    _ = this.update_in(window, |this, _, cx| {
                        this.saving = false;
                        cx.notify();
                    });
                    return Some(());
                }
            };
            let Some(path) = paths.into_iter().next() else {
                _ = this.update_in(window, |this, _, cx| {
                    this.saving = false;
                    cx.notify();
                });
                return Some(());
            };
            let result = session.transcribe_audio(path).await;
            _ = this.update_in(window, |this, window, cx| {
                this.saving = false;
                match result {
                    Ok(text) => composer.update(cx, |input, cx| {
                        input.insert(&text, window, cx);
                    }),
                    Err(error) => this.error = Some(error.to_string()),
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

    fn discover_sso(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.loading {
            return;
        }
        let server_url = self.server_input.read(cx).value().to_string();
        let runtime = self.runtime.clone();
        self.loading = true;
        self.error = None;
        self.notice = Some("Discovering identity providers...".into());
        cx.spawn_in(window, async move |this, window| {
            let result = async {
                let session = ApiSession::new(&server_url, runtime)?;
                let profile = session.instance_profile().await?;
                let providers = session.list_identity_providers().await?;
                Ok::<_, crate::api::ApiError>((session, profile, providers))
            }
            .await;
            _ = this.update_in(window, |this, window, cx| {
                this.loading = false;
                this.notice = None;
                match result {
                    Ok((session, profile, providers)) if !providers.is_empty() => {
                        this.show_sso_provider_dialog(
                            server_url.clone(),
                            session,
                            profile,
                            providers,
                            window,
                            cx,
                        );
                    }
                    Ok(_) => this.error = Some("This instance has no SSO providers.".into()),
                    Err(error) => this.error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn begin_sso(
        &mut self,
        server_url: String,
        session: ApiSession,
        profile: InstanceProfile,
        provider: IdentityProvider,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let flow = match crate::sso::SsoFlow::prepare(&provider) {
            Ok(flow) => flow,
            Err(error) => {
                self.error = Some(error.to_string());
                cx.notify();
                return;
            }
        };
        cx.open_url(&flow.authorize_url);
        let callback_task = cx.background_spawn(async move { flow.wait_for_callback() });
        self.loading = true;
        self.notice = Some(format!("Waiting for {} authentication...", provider.title));
        cx.spawn_in(window, async move |this, window| {
            let result = async {
                let callback = callback_task.await?;
                let user = session
                    .sign_in_sso(
                        callback.idp_name,
                        callback.code,
                        callback.redirect_uri,
                        callback.code_verifier,
                    )
                    .await?;
                let filter = user
                    .name
                    .as_deref()
                    .map(|name| format!("creator == \"{}\"", escape_cel_string(name)));
                let response = session
                    .list_memos_page(
                        filter,
                        Some("pinned desc, create_time desc".into()),
                        50,
                        None,
                        false,
                    )
                    .await?;
                Ok::<_, crate::api::ApiError>((user, response))
            }
            .await;
            _ = this.update_in(window, |this, _, cx| {
                this.loading = false;
                this.notice = None;
                match result {
                    Ok((user, response)) => {
                        let username = user.username.clone();
                        this.saved_login_available = false;
                        this.apply_connected_session(session, profile, Some(user), response, cx);
                        this.persist_connection(server_url, username, false);
                    }
                    Err(error) => this.error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn discover_identity_link(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(session) = self.session.clone() else {
            return;
        };
        self.module_loading = true;
        cx.spawn_in(window, async move |this, window| {
            let result = session.list_identity_providers().await;
            _ = this.update_in(window, |this, window, cx| {
                this.module_loading = false;
                match result {
                    Ok(providers) if !providers.is_empty() => {
                        this.show_link_provider_dialog(providers, window, cx);
                    }
                    Ok(_) => this.error = Some("This instance has no SSO providers.".into()),
                    Err(error) => this.error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn begin_identity_link(
        &mut self,
        provider: IdentityProvider,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let (Some(session), Some(user_name)) = (
            self.session.clone(),
            self.current_user
                .as_ref()
                .and_then(|user| user.name.clone()),
        ) else {
            return;
        };
        let flow = match crate::sso::SsoFlow::prepare(&provider) {
            Ok(flow) => flow,
            Err(error) => {
                self.error = Some(error.to_string());
                cx.notify();
                return;
            }
        };
        cx.open_url(&flow.authorize_url);
        let callback_task = cx.background_spawn(async move { flow.wait_for_callback() });
        self.module_loading = true;
        cx.spawn_in(window, async move |this, window| {
            let result = async {
                let callback = callback_task.await?;
                let request = memos_api::types::CreateLinkedIdentityRequest {
                    code: callback.code,
                    code_verifier: Some(callback.code_verifier),
                    idp_name: callback.idp_name,
                    parent: user_name.clone(),
                    redirect_uri: callback.redirect_uri,
                };
                session.create_linked_identity(user_name, request).await
            }
            .await;
            _ = this.update_in(window, |this, _, cx| {
                this.module_loading = false;
                match result {
                    Ok(identity) => this.account_resources.identities.push(identity),
                    Err(error) => this.error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn open_shared_memo(&mut self, cx: &mut Context<Self>) {
        if self.loading {
            return;
        }
        let raw = self.shared_link_input.read(cx).value().trim().to_string();
        if raw.is_empty() {
            self.error = Some("Enter a shared memo URL or token.".into());
            cx.notify();
            return;
        }
        let default_server = self.server_input.read(cx).value().to_string();
        let (server_url, token) = match url::Url::parse(&raw) {
            Ok(mut url) => {
                let token = url
                    .path_segments()
                    .and_then(|mut segments| segments.rfind(|segment| !segment.is_empty()))
                    .map(str::to_string);
                url.set_path("");
                url.set_query(None);
                url.set_fragment(None);
                match token {
                    Some(token) => (url.to_string(), token),
                    None => {
                        self.error = Some("Shared memo URL has no token.".into());
                        cx.notify();
                        return;
                    }
                }
            }
            Err(_) => (default_server, raw),
        };
        let runtime = self.runtime.clone();
        self.loading = true;
        self.error = None;
        cx.spawn(async move |this, cx| {
            let result = async {
                let session = ApiSession::new(&server_url, runtime)?;
                let profile = session.instance_profile().await?;
                let memo = session.get_shared_memo(token).await?;
                Ok::<_, crate::api::ApiError>((session, profile, memo))
            }
            .await;
            _ = this.update(cx, |this, cx| {
                this.loading = false;
                match result {
                    Ok((session, profile, memo)) => {
                        this.connected = true;
                        this.session = Some(session);
                        this.instance = Some(profile);
                        this.current_user = None;
                        this.selected_memo_name = memo.name.clone();
                        this.detail = MemoDetailData {
                            attachments: memo.attachments.clone(),
                            reactions: memo.reactions.clone(),
                            relations: memo.relations.clone(),
                            ..Default::default()
                        };
                        this.memos = vec![memo];
                        this.route = Route::Explore;
                        this.error = None;
                    }
                    Err(error) => this.error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn register(&mut self, cx: &mut Context<Self>) {
        if self.loading {
            return;
        }

        let server_url = self.server_input.read(cx).value().trim().to_string();
        let username = self.username_input.read(cx).value().trim().to_string();
        let password = self.password_input.read(cx).value().to_string();
        if username.is_empty() {
            self.error = Some("Username is required.".into());
            cx.notify();
            return;
        }
        if password.is_empty() {
            self.error = Some("Password is required.".into());
            cx.notify();
            return;
        }
        let runtime = self.runtime.clone();

        self.loading = true;
        self.error = None;
        self.notice = Some("Creating account...".into());
        cx.notify();

        cx.spawn(async move |this, cx| {
            let result = async {
                let session = ApiSession::new(&server_url, runtime)?;
                let profile = session.instance_profile().await?;
                session
                    .create_user(User {
                        avatar_url: None,
                        create_time: None,
                        description: None,
                        display_name: Some(username.clone()),
                        email: None,
                        name: None,
                        password: Some(password.clone()),
                        role: UserRole::User,
                        state: UserState::Normal,
                        update_time: None,
                        username: username.clone(),
                    })
                    .await?;
                let user = session
                    .sign_in_password(username.clone(), password.clone())
                    .await?;
                let filter = user
                    .name
                    .as_deref()
                    .map(|name| format!("creator == \"{}\"", escape_cel_string(name)));
                let response = session
                    .list_memos_page(
                        filter,
                        Some("pinned desc, create_time desc".into()),
                        50,
                        None,
                        false,
                    )
                    .await?;
                Ok::<_, crate::api::ApiError>((session, profile, user, response))
            }
            .await;

            _ = this.update(cx, |this, cx| {
                this.loading = false;
                this.notice = None;
                match result {
                    Ok((session, profile, user, response)) => {
                        this.apply_connected_session(session, profile, Some(user), response, cx);
                        this.remember_password(server_url, username, password, cx);
                    }
                    Err(error) => this.error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn authenticate(&mut self, anonymous: bool, cx: &mut Context<Self>) {
        if self.loading {
            return;
        }

        let server_url = self.server_input.read(cx).value().trim().to_string();
        let username = self.username_input.read(cx).value().trim().to_string();
        let password = self.password_input.read(cx).value().to_string();
        let token = self.token_input.read(cx).value().trim().to_string();
        let auth_mode = self.auth_mode;
        if !anonymous {
            let validation_error = match auth_mode {
                AuthMode::Password if username.is_empty() => Some("Username is required."),
                AuthMode::Password if password.is_empty() => Some("Password is required."),
                AuthMode::AccessToken if token.is_empty() => Some("Access token is required."),
                _ => None,
            };
            if let Some(error) = validation_error {
                self.error = Some(error.into());
                cx.notify();
                return;
            }
        }
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
                            session
                                .sign_in_password(username.clone(), password.clone())
                                .await?
                        }
                        AuthMode::AccessToken => session.sign_in_with_access_token(token).await?,
                    })
                };
                let filter = user
                    .as_ref()
                    .and_then(|user| user.name.as_deref())
                    .map(|name| format!("creator == \"{}\"", escape_cel_string(name)))
                    .or_else(|| Some(r#"visibility == "PUBLIC""#.into()));
                let response = session
                    .list_memos_page(
                        filter,
                        Some("pinned desc, create_time desc".into()),
                        50,
                        None,
                        false,
                    )
                    .await?;
                Ok::<_, crate::api::ApiError>((session, profile, user, response))
            }
            .await;

            _ = this.update(cx, |this, cx| {
                this.loading = false;
                this.notice = None;
                match result {
                    Ok((session, profile, user, response)) => {
                        this.apply_connected_session(session, profile, user, response, cx);
                        if !anonymous && auth_mode == AuthMode::Password {
                            this.remember_password(server_url, username, password, cx);
                        } else {
                            this.forget_saved_login(cx);
                            this.persist_connection(server_url, username, false);
                        }
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

    fn start_live_updates(&mut self, cx: &mut Context<Self>) {
        if let Some(cancel) = self.live_cancel.take() {
            cancel.store(true, Ordering::Release);
        }
        let Some(session) = self.session.clone() else {
            return;
        };
        let (mut receiver, cancel) = session.subscribe_live().into_parts();
        self.live_cancel = Some(cancel.clone());
        cx.spawn(async move |this, cx| {
            while let Some(event) = receiver.recv().await {
                if cancel.load(Ordering::Acquire) {
                    break;
                }
                let keep_running = this
                    .update(cx, |this, cx| {
                        if !this.connected {
                            return false;
                        }
                        let selected = this.selected_memo_name.as_deref();
                        let affects_selected = selected == Some(event.name.as_str())
                            || event
                                .parent
                                .as_deref()
                                .is_some_and(|parent| selected == Some(parent));
                        if affects_selected
                            && matches!(
                                event.kind.as_str(),
                                "memo.comment.created" | "reaction.upserted" | "reaction.deleted"
                            )
                            && let Some(name) = this.selected_memo_name.clone()
                        {
                            this.load_detail(name, cx);
                        }
                        if matches!(
                            event.kind.as_str(),
                            "memo.created" | "memo.updated" | "memo.deleted"
                        ) && matches!(
                            this.route,
                            Route::Timeline | Route::Archive | Route::Explore
                        ) {
                            this.reload_memos(cx);
                        }
                        true
                    })
                    .unwrap_or(false);
                if !keep_running {
                    break;
                }
            }
        })
        .detach();
    }

    fn disconnect(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(cancel) = self.live_cancel.take() {
            cancel.store(true, Ordering::Release);
        }
        if let Some(session) = self.session.take() {
            cx.spawn(async move |_, _| {
                let _ = session.sign_out().await;
            })
            .detach();
        }
        self.connected = false;
        self.route = Route::Timeline;
        self.quick_filter = QuickFilter::All;
        self.active_server_filter = None;
        self.instance = None;
        self.current_user = None;
        self.saved_login_available = false;
        self.known_users.clear();
        self.user_avatars.clear();
        self.profile_user = None;
        self.profile_stats = None;
        self.memos.clear();
        self.next_memo_page_token = None;
        self.next_notification_page_token = None;
        self.next_attachment_page_token = None;
        self.selected_memo_name = None;
        self.search_query.clear();
        self.visibility = MemoVisibility::Private;
        self.detail_tab = DetailTab::Content;
        self.detail_loading = false;
        self.detail = MemoDetailData::default();
        self.detail_error = None;
        self.link_metadata.clear();
        self.attachment_previews.clear();
        self.memo_views.clear();
        self.notifications.clear();
        self.library_attachments.clear();
        self.account_resources = AccountResources::default();
        self.admin_resources = AdminResources::default();
        self.settings_section = SettingsSection::default();
        self.loading = false;
        self.saving = false;
        self.module_loading = false;
        self.loading_more = false;
        self.error = None;
        self.notice = None;
        for input in [
            &self.password_input,
            &self.token_input,
            &self.shared_link_input,
            &self.search_input,
            &self.composer_input,
            &self.comment_input,
        ] {
            input.update(cx, |input, cx| input.set_value("", window, cx));
        }
        cx.notify();
    }

    fn open_user_profile(&mut self, user_name: String, cx: &mut Context<Self>) {
        let Some(session) = self.session.clone() else {
            return;
        };
        self.module_loading = true;
        cx.spawn(async move |this, cx| {
            let result = async {
                let (user, stats) = futures::try_join!(
                    session.get_user(user_name.clone()),
                    session.get_user_stats(user_name.clone()),
                )?;
                let filter = user
                    .name
                    .as_deref()
                    .map(|name| format!("creator == \"{}\"", escape_cel_string(name)));
                let response = session
                    .list_memos_page(
                        filter,
                        Some("pinned desc, create_time desc".into()),
                        50,
                        None,
                        false,
                    )
                    .await?;
                Ok::<_, crate::api::ApiError>((user, stats, response))
            }
            .await;
            _ = this.update(cx, |this, cx| {
                this.module_loading = false;
                match result {
                    Ok((user, stats, response)) => {
                        if let Some(name) = user.name.clone() {
                            this.known_users.insert(name, user.clone());
                        }
                        this.profile_user = Some(user);
                        this.profile_stats = Some(stats);
                        this.memos = response.memos;
                        this.next_memo_page_token = non_empty(response.next_page_token);
                        this.selected_memo_name =
                            this.memos.first().and_then(|memo| memo.name.clone());
                        this.route = Route::Profile;
                        this.error = None;
                        this.refresh_memo_assets(cx);
                        if let Some(name) = this.selected_memo_name.clone() {
                            this.load_detail(name, cx);
                        } else {
                            this.detail = MemoDetailData::default();
                            this.detail_error = None;
                        }
                    }
                    Err(error) => this.error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn navigate(&mut self, route: Route, cx: &mut Context<Self>) {
        self.route = route;
        self.quick_filter = QuickFilter::All;
        self.active_server_filter = None;
        self.error = None;
        if self.demo_mode {
            cx.notify();
            return;
        }
        match route {
            Route::Timeline | Route::Archive | Route::Explore => self.reload_memos(cx),
            Route::Views | Route::Attachments | Route::Inbox => self.load_module_data(cx),
            Route::Profile => cx.notify(),
            Route::Settings => self.load_settings_data(cx),
        }
    }

    fn save_memo_view(&mut self, view: Shortcut, cx: &mut Context<Self>) {
        let Some(user_name) = self
            .current_user
            .as_ref()
            .and_then(|user| user.name.clone())
        else {
            return;
        };
        if self.demo_mode {
            if let Some(index) = view.name.as_ref().and_then(|name| {
                self.memo_views
                    .iter()
                    .position(|existing| existing.name.as_ref() == Some(name))
            }) {
                self.memo_views[index] = view;
            } else {
                let mut view = view;
                view.name = Some(format!(
                    "{user_name}/shortcuts/demo-{}",
                    self.memo_views.len() + 1
                ));
                self.memo_views.push(view);
            }
            cx.notify();
            return;
        }
        let Some(session) = self.session.clone() else {
            return;
        };
        let updating = view.name.is_some();
        self.module_loading = true;
        cx.spawn(async move |this, cx| {
            let result = if updating {
                session.update_memo_view(view, "title,filter".into()).await
            } else {
                session.create_memo_view(user_name, view).await
            };
            _ = this.update(cx, |this, cx| {
                this.module_loading = false;
                match result {
                    Ok(view) => {
                        if let Some(index) = view.name.as_ref().and_then(|name| {
                            this.memo_views
                                .iter()
                                .position(|existing| existing.name.as_ref() == Some(name))
                        }) {
                            this.memo_views[index] = view;
                        } else {
                            this.memo_views.push(view);
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

    fn delete_memo_view_resource(&mut self, view_name: String, cx: &mut Context<Self>) {
        if self.demo_mode {
            self.memo_views
                .retain(|view| view.name.as_deref() != Some(view_name.as_str()));
            cx.notify();
            return;
        }
        let Some(session) = self.session.clone() else {
            return;
        };
        self.module_loading = true;
        cx.spawn(async move |this, cx| {
            let result = session.delete_memo_view(view_name.clone()).await;
            _ = this.update(cx, |this, cx| {
                this.module_loading = false;
                match result {
                    Ok(()) => this
                        .memo_views
                        .retain(|view| view.name.as_deref() != Some(view_name.as_str())),
                    Err(error) => this.error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn open_notification_memo(&mut self, memo_name: String, cx: &mut Context<Self>) {
        let Some(session) = self.session.clone() else {
            return;
        };
        self.module_loading = true;
        cx.spawn(async move |this, cx| {
            let result = session.get_memo(memo_name.clone()).await;
            _ = this.update(cx, |this, cx| {
                this.module_loading = false;
                match result {
                    Ok(memo) => {
                        upsert_named(&mut this.memos, memo, |item| item.name.as_deref());
                        this.route = Route::Timeline;
                        this.quick_filter = QuickFilter::All;
                        this.active_server_filter = None;
                        this.selected_memo_name = Some(memo_name.clone());
                        this.load_detail(memo_name, cx);
                    }
                    Err(error) => this.error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn set_notification_status(
        &mut self,
        notification_name: String,
        status: memos_api::types::UserNotificationStatus,
        cx: &mut Context<Self>,
    ) {
        let Some(index) = self.notifications.iter().position(|notification| {
            notification.name.as_deref() == Some(notification_name.as_str())
        }) else {
            return;
        };
        let mut notification = self.notifications[index].clone();
        notification.status = Some(status);
        if self.demo_mode {
            self.notifications[index] = notification;
            cx.notify();
            return;
        }
        let Some(session) = self.session.clone() else {
            return;
        };
        self.module_loading = true;
        cx.spawn(async move |this, cx| {
            let result = session.update_notification(notification).await;
            _ = this.update(cx, |this, cx| {
                this.module_loading = false;
                match result {
                    Ok(notification) => {
                        if let Some(index) = this
                            .notifications
                            .iter()
                            .position(|item| item.name.as_ref() == notification.name.as_ref())
                        {
                            this.notifications[index] = notification;
                        }
                    }
                    Err(error) => this.error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn delete_notification_resource(&mut self, notification_name: String, cx: &mut Context<Self>) {
        if self.demo_mode {
            self.notifications.retain(|notification| {
                notification.name.as_deref() != Some(notification_name.as_str())
            });
            cx.notify();
            return;
        }
        let Some(session) = self.session.clone() else {
            return;
        };
        self.module_loading = true;
        cx.spawn(async move |this, cx| {
            let result = session.delete_notification(notification_name.clone()).await;
            _ = this.update(cx, |this, cx| {
                this.module_loading = false;
                match result {
                    Ok(()) => this.notifications.retain(|notification| {
                        notification.name.as_deref() != Some(notification_name.as_str())
                    }),
                    Err(error) => this.error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn load_settings_data(&mut self, cx: &mut Context<Self>) {
        let (Some(session), Some(user)) = (self.session.clone(), self.current_user.clone()) else {
            cx.notify();
            return;
        };
        let Some(user_name) = user.name.clone() else {
            self.error = Some("Current user has no resource name.".into());
            cx.notify();
            return;
        };
        let is_admin = user.role == memos_api::types::UserRole::Admin;
        self.module_loading = true;
        cx.spawn(async move |this, cx| {
            let result = async {
                let (settings, identities, tokens, webhooks, stats) = futures::try_join!(
                    session.list_user_settings(user_name.clone()),
                    session.list_linked_identities(user_name.clone()),
                    session.list_access_tokens(user_name.clone()),
                    session.list_webhooks(user_name.clone()),
                    session.get_user_stats(user_name.clone()),
                )?;
                let account = AccountResources {
                    settings,
                    identities,
                    tokens,
                    webhooks,
                    stats: Some(stats),
                };
                let admin = if is_admin {
                    let setting_names = [
                        "GENERAL",
                        "STORAGE",
                        "MEMO_RELATED",
                        "TAGS",
                        "NOTIFICATION",
                        "AI",
                    ]
                    .into_iter()
                    .map(|key| format!("instance/settings/{key}"))
                    .collect();
                    let (users, user_stats, instance_settings, instance_stats, identity_providers) =
                        futures::try_join!(
                            session.list_all_users(true),
                            session.list_all_user_stats(),
                            session.list_instance_settings(setting_names),
                            session.get_instance_stats(),
                            session.list_identity_providers(),
                        )?;
                    AdminResources {
                        users,
                        user_stats: Some(user_stats),
                        instance_settings,
                        instance_stats: Some(instance_stats),
                        identity_providers,
                    }
                } else {
                    AdminResources::default()
                };
                Ok::<_, crate::api::ApiError>((account, admin))
            }
            .await;
            _ = this.update(cx, |this, cx| {
                this.module_loading = false;
                match result {
                    Ok((account, admin)) => {
                        this.account_resources = account;
                        this.admin_resources = admin;
                        this.error = None;
                    }
                    Err(error) => this.error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn set_settings_section(&mut self, section: SettingsSection, cx: &mut Context<Self>) {
        self.settings_section = section;
        cx.notify();
    }

    fn save_profile(&mut self, user: User, update_mask: String, cx: &mut Context<Self>) {
        if self.demo_mode {
            self.current_user = Some(user);
            cx.notify();
            return;
        }
        let Some(session) = self.session.clone() else {
            return;
        };
        let previous_name = self
            .current_user
            .as_ref()
            .and_then(|user| user.name.clone());
        let previous_username = self.current_user.as_ref().map(|user| user.username.clone());
        let server_url = session.base_url().to_string();
        let runtime = self.runtime.clone();
        self.module_loading = true;
        cx.spawn(async move |this, cx| {
            let mut migration_error = None;
            let result = match session.update_user(user, update_mask).await {
                Ok(user) => {
                    if let Some(previous_username) = previous_username
                        && previous_username != user.username
                    {
                        let server_url = server_url.clone();
                        let new_username = user.username.clone();
                        match runtime
                            .spawn_blocking(move || {
                                credentials::migrate_password(
                                    &server_url,
                                    &previous_username,
                                    &new_username,
                                )
                            })
                            .await
                        {
                            Ok(Ok(_)) => {}
                            Ok(Err(error)) => migration_error = Some(error),
                            Err(error) => migration_error = Some(error.to_string()),
                        }
                    }
                    Ok(user)
                }
                Err(error) => Err(error),
            };
            _ = this.update(cx, move |this, cx| {
                this.module_loading = false;
                match result {
                    Ok(user) => {
                        if let Some(index) = this
                            .admin_resources
                            .users
                            .iter()
                            .position(|item| item.name.as_ref() == previous_name.as_ref())
                        {
                            this.admin_resources.users[index] = user.clone();
                        }
                        if let Some(error) = migration_error {
                            this.error = Some(format!(
                                "Profile saved, but the saved-login key could not migrate: {error}"
                            ));
                        } else {
                            this.error = None;
                        }
                        let auto_login = AppConfig::load().auto_login;
                        this.persist_connection(server_url, user.username.clone(), auto_login);
                        this.current_user = Some(user);
                    }
                    Err(error) => this.error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn save_user_setting_resource(&mut self, setting: UserSetting, cx: &mut Context<Self>) {
        if self.demo_mode {
            upsert_named(&mut self.account_resources.settings, setting, |item| {
                item.name.as_deref()
            });
            cx.notify();
            return;
        }
        let Some(session) = self.session.clone() else {
            return;
        };
        self.module_loading = true;
        cx.spawn(async move |this, cx| {
            let result = session.update_user_setting(setting, None).await;
            _ = this.update(cx, |this, cx| {
                this.module_loading = false;
                match result {
                    Ok(setting) => {
                        upsert_named(&mut this.account_resources.settings, setting, |item| {
                            item.name.as_deref()
                        })
                    }
                    Err(error) => this.error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn create_access_token_resource(
        &mut self,
        description: String,
        expires_in_days: i32,
        cx: &mut Context<Self>,
    ) {
        let Some(user_name) = self
            .current_user
            .as_ref()
            .and_then(|user| user.name.clone())
        else {
            return;
        };
        let request = memos_api::types::CreatePersonalAccessTokenRequest {
            description: (!description.is_empty()).then_some(description),
            expires_in_days: Some(expires_in_days),
            parent: user_name.clone(),
        };
        let Some(session) = self.session.clone() else {
            return;
        };
        self.module_loading = true;
        cx.spawn(async move |this, cx| {
            let result = session.create_access_token(user_name, request).await;
            _ = this.update(cx, |this, cx| {
                this.module_loading = false;
                match result {
                    Ok(response) => {
                        if let Some(token) = response.personal_access_token {
                            this.account_resources.tokens.push(token);
                        }
                        if let Some(token) = response.token {
                            cx.write_to_clipboard(gpui::ClipboardItem::new_string(token.clone()));
                            this.notice =
                                Some(format!("New access token copied to clipboard: {token}"));
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

    fn delete_access_token_resource(&mut self, token_name: String, cx: &mut Context<Self>) {
        let Some(session) = self.session.clone() else {
            return;
        };
        self.module_loading = true;
        cx.spawn(async move |this, cx| {
            let result = session.delete_access_token(token_name.clone()).await;
            _ = this.update(cx, |this, cx| {
                this.module_loading = false;
                match result {
                    Ok(()) => this
                        .account_resources
                        .tokens
                        .retain(|token| token.name.as_deref() != Some(token_name.as_str())),
                    Err(error) => this.error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn delete_linked_identity_resource(&mut self, identity_name: String, cx: &mut Context<Self>) {
        let Some(session) = self.session.clone() else {
            return;
        };
        self.module_loading = true;
        cx.spawn(async move |this, cx| {
            let result = session.delete_linked_identity(identity_name.clone()).await;
            _ = this.update(cx, |this, cx| {
                this.module_loading = false;
                match result {
                    Ok(()) => this.account_resources.identities.retain(|identity| {
                        identity.name.as_deref() != Some(identity_name.as_str())
                    }),
                    Err(error) => this.error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn save_webhook_resource(&mut self, webhook: UserWebhook, cx: &mut Context<Self>) {
        let Some(user_name) = self
            .current_user
            .as_ref()
            .and_then(|user| user.name.clone())
        else {
            return;
        };
        let Some(session) = self.session.clone() else {
            return;
        };
        let updating = webhook.name.is_some();
        self.module_loading = true;
        cx.spawn(async move |this, cx| {
            let result = if updating {
                session.update_webhook(webhook).await
            } else {
                session.create_webhook(user_name, webhook).await
            };
            _ = this.update(cx, |this, cx| {
                this.module_loading = false;
                match result {
                    Ok(webhook) => {
                        upsert_named(&mut this.account_resources.webhooks, webhook, |item| {
                            item.name.as_deref()
                        })
                    }
                    Err(error) => this.error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn copy_webhook_secret(&mut self, webhook_name: String, cx: &mut Context<Self>) {
        let Some(session) = self.session.clone() else {
            return;
        };
        self.module_loading = true;
        cx.spawn(async move |this, cx| {
            let result = session.get_webhook_secret(webhook_name).await;
            _ = this.update(cx, |this, cx| {
                this.module_loading = false;
                match result {
                    Ok(secret) => {
                        cx.write_to_clipboard(gpui::ClipboardItem::new_string(secret));
                        this.notice = Some("Webhook signing secret copied.".into());
                    }
                    Err(error) => this.error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn delete_webhook_resource(&mut self, webhook_name: String, cx: &mut Context<Self>) {
        let Some(session) = self.session.clone() else {
            return;
        };
        self.module_loading = true;
        cx.spawn(async move |this, cx| {
            let result = session.delete_webhook(webhook_name.clone()).await;
            _ = this.update(cx, |this, cx| {
                this.module_loading = false;
                match result {
                    Ok(()) => this
                        .account_resources
                        .webhooks
                        .retain(|item| item.name.as_deref() != Some(webhook_name.as_str())),
                    Err(error) => this.error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn save_admin_user(&mut self, user: User, cx: &mut Context<Self>) {
        let Some(session) = self.session.clone() else {
            return;
        };
        let creating = user.name.is_none();
        let previous_name = user.name.clone();
        self.module_loading = true;
        cx.spawn(async move |this, cx| {
            let result = if creating {
                session.create_user(user).await
            } else {
                let mut update_mask =
                    "role,username,email,display_name,avatar_url,description,state".to_string();
                if user.password.is_some() {
                    update_mask.push_str(",password");
                }
                session.update_user(user, update_mask).await
            };
            _ = this.update(cx, |this, cx| {
                this.module_loading = false;
                match result {
                    Ok(user) => {
                        if let Some(index) = this
                            .admin_resources
                            .users
                            .iter()
                            .position(|item| item.name.as_ref() == previous_name.as_ref())
                        {
                            this.admin_resources.users[index] = user;
                        } else {
                            this.admin_resources.users.push(user);
                        }
                    }
                    Err(error) => this.error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn delete_admin_user(&mut self, user_name: String, cx: &mut Context<Self>) {
        let Some(session) = self.session.clone() else {
            return;
        };
        self.module_loading = true;
        cx.spawn(async move |this, cx| {
            let result = session.delete_user(user_name.clone(), false).await;
            _ = this.update(cx, |this, cx| {
                this.module_loading = false;
                match result {
                    Ok(()) => this
                        .admin_resources
                        .users
                        .retain(|user| user.name.as_deref() != Some(user_name.as_str())),
                    Err(error) => this.error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn save_instance_setting_resource(&mut self, setting: InstanceSetting, cx: &mut Context<Self>) {
        let Some(session) = self.session.clone() else {
            return;
        };
        self.module_loading = true;
        cx.spawn(async move |this, cx| {
            let result = session.update_instance_setting(setting, None).await;
            _ = this.update(cx, |this, cx| {
                this.module_loading = false;
                match result {
                    Ok(setting) => upsert_named(
                        &mut this.admin_resources.instance_settings,
                        setting,
                        |item| item.name.as_deref(),
                    ),
                    Err(error) => this.error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn test_instance_email(&mut self, cx: &mut Context<Self>) {
        let Some(session) = self.session.clone() else {
            return;
        };
        let recipient_email = self
            .current_user
            .as_ref()
            .and_then(|user| user.email.clone());
        let request = memos_api::types::TestInstanceEmailSettingRequest {
            email: None,
            recipient_email,
        };
        self.module_loading = true;
        cx.spawn(async move |this, cx| {
            let result = session.test_email_setting(request).await;
            _ = this.update(cx, |this, cx| {
                this.module_loading = false;
                match result {
                    Ok(()) => this.notice = Some("Test email sent successfully.".into()),
                    Err(error) => this.error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn save_identity_provider_resource(
        &mut self,
        provider: IdentityProvider,
        provider_id: Option<String>,
        cx: &mut Context<Self>,
    ) {
        let Some(session) = self.session.clone() else {
            return;
        };
        let updating = provider.name.is_some();
        self.module_loading = true;
        cx.spawn(async move |this, cx| {
            let result = if updating {
                session.update_identity_provider(provider).await
            } else {
                session
                    .create_identity_provider(provider, provider_id)
                    .await
            };
            _ = this.update(cx, |this, cx| {
                this.module_loading = false;
                match result {
                    Ok(provider) => upsert_named(
                        &mut this.admin_resources.identity_providers,
                        provider,
                        |item| item.name.as_deref(),
                    ),
                    Err(error) => this.error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn delete_identity_provider_resource(&mut self, provider_name: String, cx: &mut Context<Self>) {
        let Some(session) = self.session.clone() else {
            return;
        };
        self.module_loading = true;
        cx.spawn(async move |this, cx| {
            let result = session
                .delete_identity_provider(provider_name.clone())
                .await;
            _ = this.update(cx, |this, cx| {
                this.module_loading = false;
                match result {
                    Ok(()) => this.admin_resources.identity_providers.retain(|provider| {
                        provider.name.as_deref() != Some(provider_name.as_str())
                    }),
                    Err(error) => this.error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
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
            let result: Result<ModuleData, crate::api::ApiError> =
                match route {
                    Route::Views => match user_name {
                        Some(user_name) => session
                            .list_memo_views(user_name)
                            .await
                            .map(ModuleData::Views),
                        None => Err(crate::api::ApiError::MissingField("current user name")),
                    },
                    Route::Inbox => match user_name {
                        Some(user_name) => session
                            .list_notifications_page(user_name, None)
                            .await
                            .map(|response| {
                                ModuleData::Notifications(
                                    response.notifications,
                                    non_empty(response.next_page_token),
                                )
                            }),
                        None => Err(crate::api::ApiError::MissingField("current user name")),
                    },
                    Route::Attachments => match session.list_attachments_page(None).await {
                        Ok(response) => {
                            let previews =
                                cache_attachment_previews(&session, &response.attachments).await;
                            Ok(ModuleData::Attachments(
                                response.attachments,
                                previews,
                                non_empty(response.next_page_token),
                            ))
                        }
                        Err(error) => Err(error),
                    },
                    _ => unreachable!("module loader called for timeline route"),
                };
            _ = this.update(cx, |this, cx| {
                this.module_loading = false;
                match result {
                    Ok(ModuleData::Views(views)) => this.memo_views = views,
                    Ok(ModuleData::Notifications(notifications, next_page_token)) => {
                        this.notifications = notifications;
                        this.next_notification_page_token = next_page_token;
                    }
                    Ok(ModuleData::Attachments(attachments, previews, next_page_token)) => {
                        this.library_attachments = attachments;
                        this.attachment_previews.extend(previews);
                        this.next_attachment_page_token = next_page_token;
                    }
                    Err(error) => this.error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn load_more_notifications(&mut self, cx: &mut Context<Self>) {
        let (Some(session), Some(page_token), Some(user_name)) = (
            self.session.clone(),
            self.next_notification_page_token.clone(),
            self.current_user
                .as_ref()
                .and_then(|user| user.name.clone()),
        ) else {
            return;
        };
        self.loading_more = true;
        cx.spawn(async move |this, cx| {
            let result = session
                .list_notifications_page(user_name, Some(page_token))
                .await;
            _ = this.update(cx, |this, cx| {
                this.loading_more = false;
                match result {
                    Ok(response) => {
                        this.notifications.extend(response.notifications);
                        this.next_notification_page_token = non_empty(response.next_page_token);
                    }
                    Err(error) => this.error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn load_more_attachments(&mut self, cx: &mut Context<Self>) {
        let (Some(session), Some(page_token)) = (
            self.session.clone(),
            self.next_attachment_page_token.clone(),
        ) else {
            return;
        };
        self.loading_more = true;
        cx.spawn(async move |this, cx| {
            let result = async {
                let response = session.list_attachments_page(Some(page_token)).await?;
                let previews = cache_attachment_previews(&session, &response.attachments).await;
                Ok::<_, crate::api::ApiError>((response, previews))
            }
            .await;
            _ = this.update(cx, |this, cx| {
                this.loading_more = false;
                match result {
                    Ok((response, previews)) => {
                        this.library_attachments.extend(response.attachments);
                        this.attachment_previews.extend(previews);
                        this.next_attachment_page_token = non_empty(response.next_page_token);
                    }
                    Err(error) => this.error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn effective_memo_filter(&self) -> Option<String> {
        let mut terms = Vec::new();
        match self.route {
            Route::Explore => terms.push(r#"visibility == "PUBLIC""#.to_string()),
            Route::Timeline | Route::Archive => {
                if let Some(creator) = self
                    .current_user
                    .as_ref()
                    .and_then(|user| user.name.as_deref())
                {
                    terms.push(format!("creator == \"{}\"", escape_cel_string(creator)));
                }
            }
            Route::Profile => {
                if let Some(creator) = self
                    .profile_user
                    .as_ref()
                    .and_then(|user| user.name.as_deref())
                {
                    terms.push(format!("creator == \"{}\"", escape_cel_string(creator)));
                }
            }
            _ => {}
        }
        if let Some(filter) = self.active_server_filter.as_deref()
            && !filter.trim().is_empty()
        {
            terms.push(format!("({filter})"));
        }
        match self.quick_filter {
            QuickFilter::All => {}
            QuickFilter::Pinned => terms.push("pinned == true".into()),
            QuickFilter::Tasks => terms.push("has_task_list == true".into()),
            QuickFilter::Links => terms.push("has_link == true".into()),
            QuickFilter::Code => terms.push("has_code == true".into()),
        }
        let query = self.search_query.trim();
        if !query.is_empty() {
            terms.push(format!(
                "content.contains(\"{}\")",
                escape_cel_string(query)
            ));
        }
        (!terms.is_empty()).then(|| terms.join(" && "))
    }

    fn reload_memos(&mut self, cx: &mut Context<Self>) {
        let Some(session) = self.session.clone() else {
            cx.notify();
            return;
        };
        let archived = self.route == Route::Archive;
        let filter = self.effective_memo_filter();
        let selected_before = self.selected_memo_name.clone();
        self.loading = true;
        self.next_memo_page_token = None;
        cx.notify();

        cx.spawn(async move |this, cx| {
            let result = session
                .list_memos_page(
                    filter,
                    Some("pinned desc, create_time desc".into()),
                    50,
                    None,
                    archived,
                )
                .await;
            _ = this.update(cx, |this, cx| {
                this.loading = false;
                match result {
                    Ok(response) => {
                        this.next_memo_page_token = non_empty(response.next_page_token);
                        this.selected_memo_name = selected_before.and_then(|selected| {
                            response
                                .memos
                                .iter()
                                .find(|memo| memo.name.as_deref() == Some(selected.as_str()))
                                .and_then(|memo| memo.name.clone())
                        });
                        if this.selected_memo_name.is_none() {
                            this.selected_memo_name =
                                response.memos.first().and_then(|memo| memo.name.clone());
                        }
                        this.memos = response.memos;
                        if let Some(name) = this.selected_memo_name.clone() {
                            this.load_detail(name, cx);
                        }
                        this.error = None;
                        this.refresh_memo_assets(cx);
                    }
                    Err(error) => this.error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn load_more_memos(&mut self, cx: &mut Context<Self>) {
        let (Some(session), Some(page_token)) =
            (self.session.clone(), self.next_memo_page_token.clone())
        else {
            return;
        };
        let archived = self.route == Route::Archive;
        let filter = self.effective_memo_filter();
        self.loading_more = true;
        cx.notify();
        cx.spawn(async move |this, cx| {
            let result = session
                .list_memos_page(
                    filter,
                    Some("pinned desc, create_time desc".into()),
                    50,
                    Some(page_token),
                    archived,
                )
                .await;
            _ = this.update(cx, |this, cx| {
                this.loading_more = false;
                match result {
                    Ok(response) => {
                        this.next_memo_page_token = non_empty(response.next_page_token);
                        this.memos.extend(response.memos);
                        this.error = None;
                        this.refresh_memo_assets(cx);
                    }
                    Err(error) => this.error = Some(error.to_string()),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn open_memo_view(&mut self, filter: String, cx: &mut Context<Self>) {
        self.route = Route::Timeline;
        self.quick_filter = QuickFilter::All;
        self.active_server_filter = Some(filter);
        self.reload_memos(cx);
    }

    fn set_quick_filter(&mut self, filter: QuickFilter, cx: &mut Context<Self>) {
        self.quick_filter = filter;
        if self.demo_mode {
            cx.notify();
        } else {
            self.reload_memos(cx);
        }
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
            self.detail_loading = false;
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

    fn update_memo_content(
        &mut self,
        memo_name: String,
        content: String,
        create_time: Option<chrono::DateTime<Utc>>,
        cx: &mut Context<Self>,
    ) {
        let Some(index) = self.memo_index(&memo_name) else {
            return;
        };
        let mut memo = self.memos[index].clone();
        memo.content = content;
        let update_mask = if create_time.is_some() {
            memo.create_time = create_time;
            "content,create_time"
        } else {
            "content"
        };
        self.update_memo(index, memo, update_mask, cx);
    }

    fn update_memo_visibility(
        &mut self,
        memo_name: String,
        visibility: MemoVisibility,
        cx: &mut Context<Self>,
    ) {
        let Some(index) = self.memo_index(&memo_name) else {
            return;
        };
        let mut memo = self.memos[index].clone();
        memo.visibility = visibility;
        self.update_memo(index, memo, "visibility", cx);
    }

    fn toggle_memo_task(&mut self, memo_name: String, line_index: usize, cx: &mut Context<Self>) {
        let Some(index) = self.memo_index(&memo_name) else {
            return;
        };
        let mut memo = self.memos[index].clone();
        let had_trailing_newline = memo.content.ends_with('\n');
        let mut lines = memo.content.lines().map(str::to_string).collect::<Vec<_>>();
        let Some(line) = lines.get_mut(line_index) else {
            return;
        };
        if let Some(position) = line.find("[ ]") {
            line.replace_range(position..position + 3, "[x]");
        } else if let Some(position) = line.find("[x]").or_else(|| line.find("[X]")) {
            line.replace_range(position..position + 3, "[ ]");
        } else {
            return;
        }
        memo.content = lines.join("\n");
        if had_trailing_newline {
            memo.content.push('\n');
        }
        self.update_memo(index, memo, "content", cx);
    }

    fn update_memo_location(
        &mut self,
        memo_name: String,
        location: Option<memos_api::types::Location>,
        cx: &mut Context<Self>,
    ) {
        let Some(index) = self.memo_index(&memo_name) else {
            return;
        };
        let mut memo = self.memos[index].clone();
        memo.location = location;
        self.update_memo(index, memo, "location", cx);
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

fn extract_http_urls(content: &str) -> Vec<String> {
    let mut urls = content
        .split_whitespace()
        .map(|part| {
            part.trim_matches(|character: char| {
                matches!(
                    character,
                    '<' | '>' | '(' | ')' | '[' | ']' | '"' | '\'' | ',' | ';'
                )
            })
        })
        .filter(|part| part.starts_with("http://") || part.starts_with("https://"))
        .filter_map(|part| url::Url::parse(part).ok().map(|url| url.to_string()))
        .collect::<Vec<_>>();
    urls.sort();
    urls.dedup();
    urls
}

fn upsert_named<T, F>(items: &mut Vec<T>, item: T, name: F)
where
    F: for<'a> Fn(&'a T) -> Option<&'a str>,
{
    let item_name = name(&item).map(str::to_string);
    if let Some(index) = item_name.as_deref().and_then(|item_name| {
        items
            .iter()
            .position(|existing| name(existing) == Some(item_name))
    }) {
        items[index] = item;
    } else {
        items.push(item);
    }
}

fn non_empty(value: Option<String>) -> Option<String> {
    value.filter(|value| !value.is_empty())
}

fn escape_cel_string(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

async fn cache_attachment_previews(
    session: &ApiSession,
    attachments: &[Attachment],
) -> HashMap<String, PathBuf> {
    let downloads = attachments
        .iter()
        .filter(|attachment| is_previewable_image(attachment))
        .cloned()
        .map(|attachment| {
            let session = session.clone();
            async move {
                let name = attachment.name.clone()?;
                session
                    .cache_attachment(attachment, true)
                    .await
                    .ok()
                    .map(|path| (name, path))
            }
        });
    futures::future::join_all(downloads)
        .await
        .into_iter()
        .flatten()
        .collect()
}

async fn cache_user_avatars(session: &ApiSession, users: &[User]) -> HashMap<String, PathBuf> {
    let downloads = users.iter().cloned().map(|user| {
        let session = session.clone();
        async move {
            let name = user.name.clone()?;
            session
                .cache_user_avatar(user)
                .await
                .ok()
                .flatten()
                .map(|path| (name, path))
        }
    });
    futures::future::join_all(downloads)
        .await
        .into_iter()
        .flatten()
        .collect()
}

fn is_previewable_image(attachment: &Attachment) -> bool {
    attachment.type_.starts_with("image/")
        && !matches!(
            attachment.type_.as_str(),
            "image/vnd.adobe.photoshop" | "image/x-photoshop" | "image/photoshop"
        )
}

fn external_attachment_url(attachment: &Attachment) -> Option<String> {
    attachment
        .external_link
        .clone()
        .filter(|link| !link.trim().is_empty())
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
