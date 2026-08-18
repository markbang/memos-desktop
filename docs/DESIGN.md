# Design System

## Direction

Memos Desktop is an operational writing tool, not a web dashboard and not a replica of the Memos web client. The interface is optimized for repeated capture, scanning, and keyboard-driven action.

The visual idea is **signal ledger**: a quiet working surface with a dark structural spine and a small number of high-information colors.

## Layout

The default workspace has four stable regions:

1. A 72px navigation rail for global destinations.
2. A 272px context panel for search, tags, views, and filters.
3. A flexible timeline surface for capture and review.
4. A 320px inspector for the selected memo.

The inspector is contextual and disappears when no memo is selected. Dimensions are stable so loading states, counters, and controls do not shift the workspace.

## Color

- Graphite `#17191D`: navigation and primary text
- Paper `#F6F7F4`: application canvas
- White `#FFFFFF`: active working surfaces
- Vermilion `#E65335`: creation and destructive intent
- Cobalt `#246BFE`: navigation, links, and selected state
- Signal green `#1F9D73`: public and confirmed state
- Amber `#D7911E`: protected visibility and warnings

Color is semantic. It is not used as ambient decoration.

## Shape

- General radius: 4px
- Dialog radius: 6px
- Borders and dividers carry most hierarchy
- Shadows are disabled by default
- Cards are not nested

## Typography

System UI typography is used for interface text. Monospace is reserved for resource IDs, versions, counters, filters, and timestamps. Hero-scale typography is not used inside the workspace.

## Interaction

- Global destinations use icons with tooltips.
- Visibility uses a compact segmented control.
- Destructive operations require confirmation.
- `Ctrl+Enter` saves a memo from the composer.
- Selection opens the inspector without navigating away from the timeline.
- Network operations preserve layout and expose explicit loading/error state.

## Product boundary

The desktop client should expose the complete Memos capability set without inheriting the web client's page composition. Server concepts remain unchanged: Memo, Attachment, Relation, Reaction, Share, View, Notification, User, Identity Provider, and Instance Setting.
