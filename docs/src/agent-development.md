# Agentic Development

This page defines how agent-driven work should be documented in this repository.

## Purpose

Keep a durable process for recording:

- architecture decisions made during implementation
- conventions the agent should continue following
- feature-specific caveats that are easy to lose in commit history
- documentation debt created by fast iteration
- where implementation was difficult or required multiple iterations

## Update triggers

Add a note to the agent development log when a change introduces or changes:

- a public Python API
- a Rust crate boundary or major internal module split
- viewer interaction behavior
- data model assumptions or format conventions
- a development workflow that future agent sessions should preserve
- a difficult implementation path that future work should not rediscover from scratch

## Required workflow

When agent work should be recorded in the log, use this sequence:

1. Implement the feature or fix.
2. Create a git commit for that change.
3. Capture the resulting commit hash.
4. Update the agent development notes with that commit hash and the relevant context.
5. Commit the documentation update separately.

This two-step flow is required because the notes must reference a concrete commit hash.
If the notes were written before the feature commit existed, the hash would be missing or
guessed.

## Entry format

Add notes to [`docs/src/agent-development-notes.md`](/Users/au616397/Repositories/atomic-kernels/docs/src/agent-development-notes.md)
using this structure:

```md
## YYYY-MM-DD - Feature or Decision

- Commit: `abcdef1`
- Agent: `Codex (GPT-5, OpenAI)`
- Context: What changed and why.
- Implementation: Which modules, scripts, or crates were affected.
- Difficulty: Where the work was unexpectedly hard, required several iterations, or
  exposed a pain point in the current design or workflow.
- Constraints: Important limitations, assumptions, or non-goals.
- Follow-up: What should be documented, tested, or cleaned up next.
```

Use a short hash unless there is a reason to prefer the full hash.

## Working conventions

- State which model/provider produced the work, using a dedicated `Agent` field in each
  entry.
- Prefer concrete file and module references over vague summaries.
- Document behavior changes, not just that files were edited.
- Record where the implementation was painful, especially when multiple attempts were
  needed before the final approach worked.
- Record constraints while they are still known, especially when a feature is script-led
  before it becomes formal API.
- Keep the notes page in reverse chronological order, with newest notes first, unless
  an entry is factually wrong.
- If a feature commit is intentionally left undocumented, that should be a conscious
  exception rather than the default.
