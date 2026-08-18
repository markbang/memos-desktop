# Feature Matrix

Status values:

- **Done**: implemented in the desktop UI and wired to the API
- **Foundation**: typed API and navigation surface exist; complete UI workflow remains
- **Planned**: not implemented yet

| Area | Capability | Status |
|---|---|---|
| Connection | Instance profile and version discovery | Done |
| Authentication | Password sign-in | Done |
| Authentication | Personal access token session | Done |
| Authentication | Anonymous/public browsing | Done |
| Authentication | Refresh-cookie token renewal | Done |
| Authentication | SSO with PKCE callback | Foundation |
| Authentication | Sign-out and disconnect | Done |
| Timeline | Normal and archived feeds | Done |
| Timeline | Pagination and incremental loading | Planned |
| Timeline | Local content/tag search | Done |
| Timeline | Server-side CEL filters and ordering | Foundation |
| Timeline | Pinned, task, link, and code filters | Done |
| Memo | Create Markdown memo | Done |
| Memo | Private, Protected, and Public visibility | Done |
| Memo | Rich Markdown display | Done |
| Memo | Edit content and creation time | Planned |
| Memo | Pin and unpin | Done |
| Memo | Archive and restore | Done |
| Memo | Permanent delete with confirmation | Done |
| Memo | Location metadata | Foundation |
| Memo | Task checkbox mutation | Planned |
| Memo | Link previews and metadata | Foundation |
| Relations | References and backlinks | Foundation |
| Comments | Create, list, paginate, and delete comments | Foundation |
| Reactions | List, add, replace, and remove reactions | Foundation |
| Sharing | Create, expire, list, and revoke share links | Foundation |
| Sharing | Open shared memo without authentication | Foundation |
| Attachments | Attachment library navigation | Foundation |
| Attachments | Upload inline data and external links | Foundation |
| Attachments | Image, video, audio, and document previews | Planned |
| Attachments | Media metadata and motion-photo groups | Foundation |
| Views | Built-in desktop quick views | Done |
| Views | List/create/edit/delete server Memo Views | Foundation |
| Inbox | Notification destination and empty state | Foundation |
| Inbox | Mention/comment list, read/archive/delete | Foundation |
| Users | User profile and public memo feed | Foundation |
| Users | Account profile, username, email, avatar, password | Foundation |
| Tokens | List/create/delete personal access tokens | Foundation |
| Webhooks | List/create/edit/delete and signing secret | Foundation |
| Settings | User locale, theme, memo layout, and tag metadata | Foundation |
| Admin | User management and statistics | Foundation |
| Admin | General instance policy and branding | Foundation |
| Admin | Storage backends and upload limits | Foundation |
| Admin | Memo policy and reaction configuration | Foundation |
| Admin | SMTP notification configuration and test | Foundation |
| Admin | OAuth identity providers | Foundation |
| Admin | AI providers and transcription policy | Foundation |
| Admin | Instance resource statistics | Foundation |
| AI | Audio transcription request | Foundation |
| Live updates | SSE memo and notification refresh | Planned |
| Desktop | Multi-profile connection switcher | Planned |
| Desktop | System tray and global quick capture | Planned |
| Desktop | Protocol/deep-link handling | Planned |
| Desktop | Offline cache and outbox | Planned |

The generated `memos-api` crate already contains typed methods and models for every v0.30 OpenAPI operation. “Foundation” therefore means presentation and workflow work, not missing transport coverage.
