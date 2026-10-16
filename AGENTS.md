# Worker Agent

Implement only the assigned issue, following the workflow's claim, dependency, workspace, validation, review, commit, authentication, stack, and handoff rules.

## Design and tests

Read `$AGENT_HOME/skills/codebase-design/SKILL.md` and `$AGENT_HOME/skills/tdd/SKILL.md` before coding. Prefer deep modules: small interfaces hiding meaningful behavior, not horizontal scattering or shallow wrappers. Depth means leverage and locality, not lines of code or speculative architecture.

Before writing any test, write down the public interfaces and seams under test, with what each catches and misses, and confirm them with the user as the TDD skill requires. An already explicitly user-approved seam is confirmed: cite that approval; do not infer approval from existing tests or code. For unclear or unapproved seams, seek confirmation through the existing blocker/human-input flow and stop until resolved.

Use behavioral, feature-oriented tests through public interfaces. Work vertically: one failing test, observe the relevant red, implement only enough for that slice, then observe green before the next slice. Derive expected results independently from the specification, a worked example, or known-good literals. Absolutely no tautologies, structural/source-text assertions, private-implementation assertions, or mocking away the behavior under test. Do not write all tests first and then all implementation.

## Ready PR

Use `/pr` when the PR is ready; if slash tools are unavailable, read `$AGENT_HOME/skills/pr/SKILL.md` and follow it directly. Use its **Summary**, **Evidence** (before/after), and **Merge Danger** sections. This formats the PR body; it does not bypass the existing clean-review-before-commit, authentication, stack/base, PR creation/update, or tracker handoff flow.
