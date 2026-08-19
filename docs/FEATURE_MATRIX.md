# Feature Matrix

Status values:

- **Done**: implemented in the desktop UI and wired to Memos v0.30
- **Foundation**: usable transport or presentation exists, but the native workflow is intentionally limited
- **Desktop roadmap**: desktop-only enhancement; not required for Memos v0.30 server parity

| Area | Capability | Status |
|---|---|---|
| Connection | Instance profile, version, URL validation, and demo discovery | Done |
| Authentication | Password, PAT, anonymous, and shared-link sessions | Done |
| Authentication | Registration and first-admin instance initialization | Done |
| Authentication | Isolated refresh-cookie renewal and sign-out | Done |
| Authentication | OAuth2/SSO with local PKCE callback and state validation | Done |
| Authentication | Link and unlink SSO identities | Done |
| Timeline | Normal, archived, public Explore, and user Profile feeds | Done |
| Timeline | Server CEL search/filter/order and incremental pagination | Done |
| Timeline | Pinned, task, link, code, tag, and visibility filters | Done |
| Memo | Create and render Markdown memos | Done |
| Memo | Edit content, creation time, visibility, state, pin, and location | Done |
| Memo | Task checkbox mutation and permanent delete confirmation | Done |
| Memo | Link metadata previews | Done |
| Relations | List, add, and remove references and backlinks | Done |
| Comments | Create, fully paginate, render, and delete comments | Done |
| Reactions | List, add, and remove reactions | Done |
| Sharing | Create expiring links, copy, list, and revoke | Done |
| Sharing | Open a shared memo without authentication | Done |
| Attachments | Paginated library, multi-file upload, external links, edit, and delete | Done |
| Attachments | Inline Memo galleries, authenticated thumbnails, image lightbox, and default-app opening | Done |
| Attachments | Motion-photo metadata grouping and native audio/video playback | Foundation |
| Shortcuts | List, create, edit, execute, and delete v0.30 saved CEL shortcuts | Done |
| Inbox | Paginated mention/comment notifications and related-Memo navigation | Done |
| Inbox | Archive, restore to unread, and delete | Done |
| Users | Public profiles, statistics, author navigation, and paginated memos | Done |
| Users | Registration plus account username/email/avatar/description/password editing | Done |
| Tokens | List, create, reveal-once/copy, and delete personal access tokens | Done |
| Webhooks | List, create, edit, delete, and copy signing secrets | Done |
| Settings | User general/tag/webhook setting resources | Done |
| Admin | User CRUD, public profiles, and cross-user statistics | Done |
| Admin | General, storage, memo, tag, notification, and AI instance settings | Done |
| Admin | SMTP test, OAuth2 identity-provider CRUD, and instance statistics | Done |
| AI | Select audio and insert server transcription into the composer | Done |
| Live updates | Authenticated SSE reconnect and memo/comment/reaction refresh | Done |
| Desktop | System/light/dark themes, user avatars, context menu, and secure auto-login | Done |
| Desktop | Multi-profile connection switcher | Desktop roadmap |
| Desktop | System tray and global quick capture | Desktop roadmap |
| Desktop | Protocol/deep-link registration | Desktop roadmap |
| Desktop | Offline cache and mutation outbox | Desktop roadmap |

The generated `memos-api` crate contains typed methods and models for all 73 operations in the official Memos v0.30.0 OpenAPI contract. The ignored live integration test additionally exercises registration, independent same-server sessions, refresh rotation, SSE, memo CRUD/state/location, comments, notifications, reactions, relations, shares, attachments and thumbnails, shortcuts, settings, PATs, webhooks, statistics, and cleanup against the official v0.30 binary.
