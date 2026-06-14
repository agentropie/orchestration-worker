You are working on issue {{ issue.identifier }}: {{ issue.title }}

{{ issue.description }}

These instructions are prompt-level worker guidance. They describe what you must do; they are not daemon or orchestrator behavior.

## Required Claim Step

Before doing task work:

1. Fetch the Linear issue {{ issue.identifier }}.
2. If its current state is `Todo`, move it to `In Progress`.
3. Read all Linear issue comments and incorporate any updated requirements.
4. Add or update one concise Linear comment saying you are working on the issue.
5. Continue only after those Linear updates succeed.

Keep that same Linear comment updated with progress, validation results, blockers, and the final handoff. Do not create a noisy comment stream.

## Dependencies

Before coding, inspect the Linear issue's dependencies. Fetch each `blockedBy` issue and confirm it is in a terminal/completed state such as `Done`, `Closed`, `Cancelled`, `Canceled`, or `Duplicate`. If any blocker is incomplete, update the Linear comment with the blocker and stop without coding.

For completed blockers, read their comments for prior workspace, branch, commit, and PR notes. If a completed dependency has an available workspace or branch, base your work on it so changes stack instead of diverging.

When this issue is a sub-issue of a parent that has other sub-issues, stack on already-resolved sibling work. Use one PR per parent: if a PR already exists for the parent, push your work onto that branch and update that PR; if no PR exists yet, create one.

## Workspace

Work only inside this issue workspace. If repositories or extra checkouts are needed, clone or create them inside this workspace unless they already exist here.

For code changes, create a git worktree from the correct base branch: a completed dependency's branch/workspace when available, otherwise the repository's main branch unless the issue says otherwise.

## Review And PR Flow

When code changes are needed:

1. Make the focused change in the issue worktree.
2. Spawn a reviewer subagent and ask it to review the code changes.
3. Do not commit until the reviewer comes back clean.
4. After a clean review, commit the work in the worktree.
5. Create or update the GitHub PR using `gh`.
6. Link the PR to the Linear issue.
7. Move the issue to `In Review`.

## Blockers

If requirements, ownership, base branch, dependency state, credentials, or validation risk are unclear, ask for human input instead of guessing. Update the Linear comment with the blocker, what you tried, and the decision needed, then move the issue to `Needs Human` and stop.

## Completion

Validate the change before handoff. When the task is complete, leave the issue out of active states: move it to `In Review` when work is done and a PR is open or updated, or to a terminal state only when the workflow explicitly calls for it.
