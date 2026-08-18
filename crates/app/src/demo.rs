use chrono::{Duration, Utc};
use memos_api::types::{
    InstanceProfile, Memo, MemoProperty, MemoState, MemoVisibility, User, UserRole, UserState,
};

pub fn instance() -> InstanceProfile {
    InstanceProfile {
        version: Some("0.30.0".into()),
        instance_url: Some("https://memos.local".into()),
        demo: Some(true),
        needs_setup: Some(false),
        commit: Some("desktop-preview".into()),
        admin: Some(user()),
    }
}

pub fn user() -> User {
    User {
        avatar_url: None,
        create_time: Some(Utc::now() - Duration::days(320)),
        description: Some("Building small systems that stay useful.".into()),
        display_name: Some("Lin Chen".into()),
        email: Some("lin@example.com".into()),
        name: Some("users/lin".into()),
        password: None,
        role: UserRole::Admin,
        state: UserState::Normal,
        update_time: Some(Utc::now() - Duration::days(2)),
        username: "lin".into(),
    }
}

pub fn memos() -> Vec<Memo> {
    vec![
        memo(
            "memos/focus-week",
            0,
            true,
            MemoVisibility::Private,
            "# Focus for this week\n\nShip the desktop capture flow, keep the API boundary small, and leave enough room for keyboard-first navigation.\n\n- [x] Connection profile\n- [x] Timeline shell\n- [ ] Attachment upload\n\n#build #weekly",
            &["build", "weekly"],
        ),
        memo(
            "memos/gpui-notes",
            1,
            false,
            MemoVisibility::Protected,
            "GPUI keeps the important loop direct: state, render, pixels. The useful constraint is that every interaction has to earn its place.\n\n```rust\ndiv().flex().gap_2().child(\"capture first\")\n```\n\n#rust #gpui",
            &["rust", "gpui"],
        ),
        memo(
            "memos/reading",
            2,
            false,
            MemoVisibility::Private,
            "Reading note: tools become durable when capture is easier than postponing the thought. Organization can happen later.\n\n#reading",
            &["reading"],
        ),
        memo(
            "memos/design-language",
            4,
            false,
            MemoVisibility::Public,
            "Design language: white working surfaces, a graphite navigation spine, vermilion for intent, cobalt for navigation, and green only for confirmed state.\n\nAvoid decorative containers. Let rules, rhythm, and typography carry the hierarchy.\n\n#design #system",
            &["design", "system"],
        ),
        memo(
            "memos/api-boundary",
            7,
            false,
            MemoVisibility::Protected,
            "The generated client now covers every Memos v1 service: auth, memos, comments, reactions, shares, attachments, views, notifications, users, webhooks, identity providers, instance settings, and transcription.\n\n#api #architecture",
            &["api", "architecture"],
        ),
    ]
}

fn memo(
    name: &str,
    age_days: i64,
    pinned: bool,
    visibility: MemoVisibility,
    content: &str,
    tags: &[&str],
) -> Memo {
    Memo {
        attachments: Vec::new(),
        content: content.into(),
        create_time: Some(Utc::now() - Duration::days(age_days)),
        creator: Some("users/lin".into()),
        location: None,
        name: Some(name.into()),
        parent: None,
        pinned: Some(pinned),
        property: Some(MemoProperty {
            has_code: Some(content.contains("```")),
            has_incomplete_tasks: Some(content.contains("- [ ]")),
            has_link: Some(content.contains("http")),
            has_task_list: Some(content.contains("- [")),
            title: content
                .lines()
                .next()
                .and_then(|line| line.strip_prefix("# "))
                .map(str::to_string),
        }),
        reactions: Vec::new(),
        relations: Vec::new(),
        snippet: Some(
            content
                .lines()
                .next()
                .unwrap_or_default()
                .trim_start_matches("# ")
                .into(),
        ),
        state: MemoState::Normal,
        tags: tags.iter().map(|tag| (*tag).to_string()).collect(),
        update_time: Some(Utc::now() - Duration::days(age_days)),
        visibility,
    }
}
