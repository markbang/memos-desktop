# Design System

## Direction

Memos Desktop uses the official Memos v0.30 Web client as the visual and interaction baseline. The native implementation follows the same information architecture, content density, memo-card rhythm, editor hierarchy, and navigation vocabulary while keeping GPUI-native rendering and desktop behavior.

This is an incremental parity project, not a claim that every Web feature is already reproduced. The feature matrix records the remaining gaps.

## Layout

The default workspace follows the Web client's desktop proportions:

1. A 64px navigation rail for Memos, Explore, Attachments, Inbox, and secondary destinations.
2. A 256px explorer for search, recent activity, quick filters, tags, and connection state.
3. A centered memo feed capped at a readable column width, with the composer above the list.
4. A contextual 320px inspector that opens only after a memo is selected.

The inspector is an intentional native enhancement of the Web detail-sidebar pattern. It does not consume first-visit feed space, and closing it returns the feed to its centered reading width.

## Web Baseline

The local v0.30 Web source is the source of truth for:

- Navigation order, labels, active and hover states, and sidebar proportions
- Search, activity, tag, shortcut, and filter placement
- Composer spacing, visibility controls, Markdown content, and save actions
- Memo headers, author identity, relative time, visibility, pin state, content, and media
- Detail, profile, archive, Explore, Inbox, settings, and administration page hierarchy

Native controls may differ in implementation, but their state model and visual meaning should remain recognizable to an existing Memos user.

## Color

- Warm paper: application canvas and explorer background
- White or elevated graphite: memo cards and working surfaces
- Cobalt: primary actions, links, focus, and selected state
- Vermilion: destructive or high-attention intent
- Signal green: connected and confirmed state
- Amber: protected visibility and warnings

Color is semantic. It is not used as ambient decoration. Light and dark palettes preserve the same semantic roles.

## Shape

- General radius: 6-8px, matching the Web cards and controls
- Borders and dividers carry most hierarchy
- Shadows are disabled by default
- Repeated cards are individually framed; page sections are not nested cards
- Fixed rails, panels, cards, and controls use stable dimensions so content cannot shift them

## Typography

System UI typography is used for interface text. Monospace is reserved for resource IDs, versions, counters, filters, and timestamps. Heading scale stays compact inside the workspace; memo Markdown owns the primary reading hierarchy.

## Interaction

- Global destinations use icons with tooltips.
- Search and tag filters update the active feed; server sessions use the corresponding CEL filter.
- Memo selection opens the contextual inspector without losing the feed position.
- Attachments open in inline galleries and native previews where supported.
- Destructive operations require confirmation.
- `Ctrl+Enter` saves a memo from the composer.
- Network operations preserve layout and expose explicit loading and error state.

## Product Boundary

The desktop client preserves Memos server concepts and API contracts: Memo, Attachment, Relation, Reaction, Share, Shortcut, Notification, User, Identity Provider, and Instance Setting. Desktop-only behavior such as secure credential storage, native previews, and the contextual inspector may extend the Web workflow, but must not change the meaning of server data.
