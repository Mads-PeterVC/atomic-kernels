---
name: update-agent-development-notes
description: Update `docs/agent-development-notes.md` using the workflow and entry format defined in `docs/agent-development.md`. Use when Codex has just made or inspected a git commit for agent-driven implementation work and must record it in the development log, append a new development-note entry for a completed feature or fix, replace a placeholder commit hash in the notes, or decide whether a recent committed change should be documented in the agent development log.
---

# Update Agent Development Notes

## Overview

Follow the repository workflow for durable agent-development logging. Prefer triggering this skill after a relevant implementation commit exists, but if it is invoked early, first create or confirm the feature commit, then inspect the commit history and append or correct an entry in `docs/agent-development-notes.md` without inventing hashes or implementation details.

## Workflow

1. Read `docs/agent-development.md` before editing anything.
2. Decide whether the change belongs in the log by checking the documented triggers:
   - public Python API changes
   - Rust crate or major module boundary changes
   - viewer interaction behavior changes
   - data model assumptions or format conventions
   - development workflows future agent sessions should preserve
   - difficult implementation paths that future work should not rediscover
3. Confirm the implementation commit already exists.
4. If no implementation commit exists yet:
   - make the feature commit yourself when your current permissions and workflow allow it
   - otherwise ask the user whether you should create the feature commit before continuing
   - do not abandon the task only because the skill was invoked before the commit step
5. Inspect the feature commit with `git show --stat --summary <hash>` and read the relevant touched files if the summary is not enough.
6. Insert a new note near the top of `docs/agent-development-notes.md` so the file stays in reverse chronological order (newest notes first), unless you are correcting a factually wrong existing entry.
7. Commit the documentation update separately from the feature commit.

Do not guess commit hashes. If you must create the feature commit first, finish that step before writing the log entry so the note can reference the real hash.

## Entry Rules

Use this structure unless the existing file proves a narrow exception is already established:

```md
## YYYY-MM-DD - Feature or Decision

- Commit: `abcdef1`
- Context: What changed and why.
- Implementation: Which modules, scripts, or crates were affected.
- Difficulty: Where the work was unexpectedly hard, required several iterations, or exposed a pain point in the current design or workflow.
- Constraints: Important limitations, assumptions, or non-goals.
- Follow-up: What should be documented, tested, or cleaned up next.
```

Prefer a short hash. Use concrete file, module, crate, script, and behavior references. Describe the final behavior and the hard parts of reaching it, not just that files changed.

## Writing Guidance

- Derive the note from the committed diff and surrounding source, not from vague memory.
- Keep the notes page in reverse chronological order (newest notes first) unless an existing entry is factually wrong.
- Preserve the repository’s current Markdown style and line wrapping.
- Treat pre-commit invocation as a recoverable workflow state: create the implementation commit when allowed, or explicitly ask the user whether to do that next.
- Call out constraints that matter to future implementation work, especially temporary workflow decisions and script-led behavior.
- Record genuine friction in `Difficulty`, including dead ends, misleading abstractions, or repeated iterations.
- If updating an older placeholder entry such as `Commit: TBD`, replace only the missing factual data and tighten the wording only where needed for accuracy.

## Trigger Examples

- "I just committed a viewer change; use $update-agent-development-notes for that commit."
- "Use $update-agent-development-notes to add a note for commit `abc1234`."
- "Update `docs/agent-development-notes.md` to reflect the viewer interaction change I just committed."
- "Check whether this Rust module split should be recorded in the agent development log, and add the entry if it should."
- "Replace the placeholder hash in the documentation-system entry with the real commit."
