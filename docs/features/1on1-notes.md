# 1-on-1 Notes Feature Specification

## Implementation Status

| Requirement | Status | Notes |
|-------------|--------|-------|
| FR-1: Free-Form Markdown Notes | ✅ Implemented | External editor handles full markdown |
| FR-2: Notes Associated with Meetings | ✅ Implemented | One file per meeting (YYYY-MM-DD.md) |
| FR-3: Notes History per Engineer | ✅ Implemented | Chronological list in detail view |
| FR-4: Search Within Notes | 📋 Planned | Not yet implemented |
| FR-5: Auto-Save Functionality | ✅ Implemented | Delegated to external editor |
| FR-6: Action Item Tracking | 📋 Planned | Not yet implemented |
| FR-7: Optional Templates | 📋 Planned | Not yet implemented |
| Delete Meeting with Confirmation | ✅ Implemented | Del key with confirmation modal |

---

## Feature Overview

The 1-on-1 Notes feature provides a **free-form, markdown-native** note-taking experience for engineering managers during and after 1-on-1 meetings. It is designed to complement existing workflows (Obsidian, Notion) rather than replace them, giving managers full flexibility in how they structure and organize their notes.

### Core Philosophy

- **Free-form by default** - No forced structure; write notes the way you naturally do
- **Markdown native** - Full markdown support for users who live in markdown editors
- **Meeting-centric** - Notes are associated with specific meetings for easy chronological reference
- **Privacy first** - All notes stored locally; never synced to external servers
- **Portable** - Stored as plain markdown, inherently compatible with other tools

---

## User Needs

### Primary Use Cases

1. **During Meeting Note-Taking**
   - Capture discussion points, decisions, and observations in real-time
   - Reference previous notes for context
   - Quick entry without disrupting conversation flow

2. **Meeting Preparation**
   - Review notes from previous 1-on-1s before the next meeting
   - Check outstanding action items
   - Refresh memory on ongoing topics and commitments

3. **Follow-Up and Accountability**
   - Track what was discussed and agreed upon
   - Mark action items as complete
   - Maintain continuity across meetings

4. **Historical Reference**
   - Search across all notes for a specific engineer
   - Find when topics were discussed
   - Track evolution of discussions over time

### User Expectations

Given the target user is markdown-native and currently uses Obsidian/Notion:

- Expects standard markdown syntax to work (headers, lists, bold, links, code blocks)
- Expects to structure notes their own way
- Expects notes stored as plain markdown (portable by nature)
- Expects auto-save (no manual "save" button)
- Does NOT expect rigid templates or mandatory fields

---

## Functional Requirements

### FR-1: Free-Form Markdown Notes

The note editor must support full markdown editing with the user deciding their own structure.

**Requirements:**
- Full markdown syntax support (CommonMark standard)
- Live preview or split-pane view (optional toggle)
- Syntax highlighting in edit mode
- No mandatory fields or structure
- No character limits on note content
- Support for:
  - Headings (H1-H6)
  - Bullet and numbered lists
  - Task lists (`- [ ]` and `- [x]`)
  - Bold, italic, strikethrough
  - Code blocks and inline code
  - Links
  - Blockquotes
  - Horizontal rules
  - Tables (basic support)

**Non-Requirements:**
- Embedded images (out of scope for MVP)
- Bi-directional linking (not an Obsidian replacement)
- WYSIWYG editing (markdown-native users prefer source editing)

### FR-2: Notes Associated with Meetings

Each note is tied to a specific 1-on-1 meeting instance.

**Requirements:**
- One `MeetingNote` per `Meeting` entity
- Note inherits meeting date for chronological ordering
- Meeting context displayed when viewing/editing note:
  - Engineer name
  - Meeting date
  - Meeting status (scheduled/completed)
- Can create note before meeting starts (preparation)
- Can edit note after meeting is marked complete

**Relationships:**
```
Meeting (1) -----> (1) MeetingNote
   |
   v
Engineer
```

### FR-3: Notes History per Engineer

Users must be able to view all notes for a specific engineer in chronological order.

**Requirements:**
- List view of all notes for an engineer
- Sorted by meeting date (newest first by default)
- Toggleable sort order (newest/oldest first)
- Display note preview (first ~100 characters or first line)
- Display meeting date
- Quick navigation to full note
- Visual indicator for notes with action items
- Pagination or infinite scroll for engineers with many notes

**UI Considerations:**
- Optimized for 1-2 years of weekly meetings (~50-100 notes)
- Support for longer histories without performance degradation

### FR-4: Search Within Notes

Full-text search capability across all notes.

**Requirements:**
- Search across all notes for a specific engineer
- Search across all notes globally (all engineers)
- Search within note content
- Results show:
  - Engineer name
  - Meeting date
  - Matching text snippet with highlight
- Case-insensitive search
- Search as you type (debounced, 300ms)

**Nice to Have (Post-MVP):**
- Filter by date range
- Filter by presence of action items
- Advanced query syntax

### FR-5: Auto-Save Functionality

Notes must save automatically without user intervention.

**Requirements:**
- Auto-save triggered on:
  - Content change (debounced, 2 second delay)
  - Tab/window blur (immediate save)
  - Navigation away from note
- Visual indicator showing save status:
  - "Saving..." during save
  - "Saved" with timestamp after successful save
  - "Unsaved changes" if save fails
- No manual save button required (but can include for user comfort)
- Offline support - saves to local storage even without connection
- Conflict resolution not needed (single user, local storage)

**Implementation Notes:**
- Editing delegated to external editor ($EDITOR)
- External editor handles saving directly to disk
- File modification time serves as timestamp

### FR-6: Optional Action Item Tracking

Users may mark items in their notes as action items. This is opt-in and convention-based.

**Requirements:**
- Recognize markdown task list syntax:
  ```markdown
  - [ ] Uncompleted action item
  - [x] Completed action item
  ```
- Toggle completion directly in editor (click checkbox or keyboard shortcut)
- Extract action items for display in:
  - Note summary view
  - Engineer overview
  - (Future) Dashboard widget
- Action items are part of note content, not separate entities
- Optional: Parse action item metadata (owner, due date) from conventions:
  ```markdown
  - [ ] Review PR @me due:2024-02-01
  - [ ] Update documentation @sarah
  ```

**Non-Requirements:**
- Separate action item database
- Mandatory action item tracking
- Complex project management features

### FR-7: Optional Templates

Provide starting-point templates for users who want structure, without requiring them.

**Requirements:**
- Templates are purely optional starting points
- Users can ignore templates entirely
- Available templates:
  1. **Blank** (default) - Empty note
  2. **Simple** - Basic sections
  3. **Comprehensive** - Detailed structure
  4. **Career Focused** - For development discussions
- Template selection when creating new note
- Can switch from blank to template on existing empty note
- Templates are editable markdown (user customization)

**Sample Templates:**

*Simple Template:*
```markdown
## Discussion Topics
-

## Notes


## Action Items
- [ ]
```

*Comprehensive Template:*
```markdown
## Check-in
How are you doing?


## Since Last Time
- Follow-up on previous items
- Progress updates


## Discussion Topics
-


## Career & Growth


## Action Items
- [ ]


## Notes for Next Time

```

**Template Management:**
- Built-in templates provided
- User can create custom templates (future enhancement)
- Templates stored in settings/preferences

---

## Privacy Considerations

### Data Locality

All 1-on-1 notes are stored locally on the user's machine.

**Implementation:**
- Notes stored as plain markdown files in workspace folder
- No data transmitted to external servers
- No cloud sync functionality
- No analytics or telemetry on note content

### Data Sensitivity

1-on-1 notes often contain sensitive information:
- Personal discussions
- Performance observations
- Career aspirations
- Salary/promotion discussions

**Protections:**
- Local-only storage (device-specific)
- No automatic sharing
- Clear data deletion controls

### Data Ownership

- Notes stored as plain markdown (inherently portable)
- User can delete all data completely
- Local storage means user retains full control

---

## UI Components

### Note Viewer Component

The primary interface for viewing notes. Editing is delegated to the user's external editor.

**Elements:**
- Markdown content display with syntax highlighting
- Meeting context header:
  - Meeting date
  - Mood gauge (F1-F5 to set)
- Help line showing available actions

**Behavior:**
- View-only display of markdown content
- Press 'e' to open in external editor ($EDITOR)
- Escape key to return to engineer detail view

**Keyboard Shortcuts:**
| Shortcut | Action |
|----------|--------|
| `e` | Open in external editor |
| `F1-F5` | Set mood (1-5) |
| `Esc` | Back to engineer view |
| `q` | Quit application |
| `Del` | Delete meeting (with confirmation) |

**External Editor Delegation:**
- Follows UNIX philosophy: delegate editing to purpose-built tools
- Similar to how `git commit` spawns $EDITOR for commit message editing
- Respects `$EDITOR` → `$VISUAL` → fallback chain (nano, vim, vi)
- Supports GUI editors that support `--wait` flag (e.g., `code --wait`)

### Notes History View

List of all notes for an engineer.

**Elements:**
- Engineer name header with level badge
- Sort toggle (newest/oldest)
- Search box (search within this engineer's notes)
- Note list items:
  - Meeting date
  - Note preview (truncated content)
  - Action item indicator (count of incomplete items)
  - Click to open full note

**Behavior:**
- Lazy loading for long histories
- Smooth scroll
- Keyboard navigation support

### Search Interface

Global or per-engineer note search.

**Elements:**
- Search input field
- Scope selector (All Engineers / Specific Engineer)
- Results list:
  - Engineer name (color-coded)
  - Meeting date
  - Content snippet with match highlighted
- Empty state for no results
- Loading state during search

**Behavior:**
- Search as you type (debounced)
- Results update incrementally
- Click result to open note with search term highlighted

---

## Data Requirements

### MeetingNote Entity

As defined in the data model:

```typescript
interface MeetingNote {
  id: string;
  meetingId: string;             // Reference to Meeting
  engineerId: string;            // Reference to Engineer (denormalized)

  // Content
  content: string;               // Markdown content

  // Optional Structure (extracted from content)
  actionItems?: ActionItem[];    // Parsed action items

  // Metadata
  createdAt: Date;
  updatedAt: Date;
}

interface ActionItem {
  id: string;
  text: string;
  isCompleted: boolean;
  dueDate?: Date;
  owner?: string;                // "me" or engineer name
}
```

### Indexing Requirements

For search and filtering:

| Index | Fields | Purpose |
|-------|--------|---------|
| By Engineer | `engineerId`, `updatedAt` | Notes history view |
| By Meeting | `meetingId` | Retrieve note for meeting |
| Full-text | `content` | Search within notes |

### Storage Considerations

- Average note size: ~2-5KB of markdown text
- For 10 engineers with weekly 1-on-1s over 2 years: ~1000 notes
- Estimated storage: ~5MB total
- Filesystem storage more than sufficient for this scale

---

## Markdown-Native Storage

### Principle

Notes are stored as plain markdown files in the workspace folder. This approach means:
- Notes are human-readable in their stored form
- No transformation or export process needed
- Content is inherently portable
- Compatible with markdown workflows (Notion, Obsidian)

### Benefits

- **Simplicity** - No export/import complexity
- **Transparency** - What you see is what's stored
- **Portability** - Markdown is the source of truth
- **Durability** - Standard format that won't become obsolete

---

## Integration Points

### Meeting Entity

- Notes created when meeting is scheduled or when first editing
- Note accessible from meeting detail view
- Meeting completion can prompt for note finalization

### Engineer Profile

- Notes history accessible from engineer profile
- Recent notes summary on engineer dashboard card
- Action items rollup from notes

### Dashboard

- Upcoming meetings with quick note access
- Incomplete action items across all engineers (future)

---

## Implementation Notes

### External Editor Delegation

The TUI delegates text editing to external editors following the UNIX philosophy:

**Design Rationale:**
- Do one thing well: TUI focuses on organization and tracking, not text editing
- Users already have preferred markdown editors (vim, emacs, VS Code, etc.)
- Simplicity over features: don't reinvent editors
- Terminal-first users are comfortable with `$EDITOR` workflows

**Editor Resolution Chain:**
1. `$EDITOR` environment variable (standard for line-based editors)
2. `$VISUAL` environment variable (standard for full-screen editors)
3. Fallback: nano → vim → vi

**Workflow:**
1. User presses 'e' in note viewer
2. TUI suspends (leaves alternate screen, disables raw mode)
3. External editor spawns with meeting file path
4. User edits and saves in their preferred editor
5. TUI resumes and reloads content from disk
6. Status message shows whether changes were detected

### Search Implementation

Options for full-text search (planned):
- Rust-based text search (e.g., tantivy)
- Simple string matching with grep-like semantics
- In-memory indexing for fast queries

### Performance Considerations

- Lazy load note content in history view
- Virtualized list for large note histories
- Debounce all auto-save and search operations
- Index action items on save for fast rollup queries

---

## Future Enhancements

The following are explicitly out of scope for MVP but worth considering:

1. **Custom templates** - User-created template library
2. **Note linking** - Link between notes or to engineer profiles
3. **Voice-to-text** - Speech transcription for note-taking
4. **Attachment support** - Embed images or files
5. **AI summarization** - Generate meeting summaries
6. **Calendar integration** - Pull meeting context from calendar

---

## Success Criteria

The 1-on-1 Notes feature is successful when:

1. **Adoption** - User consistently takes notes in Vibe Manager instead of separate tools
2. **Retrieval** - User can find past discussions within seconds
3. **Continuity** - Notes provide clear thread between meetings
4. **Flexibility** - Users with different note-taking styles all find it comfortable
5. **Trust** - User confident notes are saved and won't be lost
