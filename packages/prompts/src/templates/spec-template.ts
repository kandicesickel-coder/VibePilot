// packages/prompts/src/templates/spec-template.ts
// Spec document template — from spec-driven-development skill

export const SPEC_TEMPLATE = `# Spec: {project_name}

## Objective
[What we're building and why. User stories or acceptance criteria.]

## Tech Stack
[Framework, language, key dependencies with versions]

## Commands
\`\`\`
Build: {build_command}
Test: {test_command}
Lint: {lint_command}
Dev: {dev_command}
\`\`\`

## Project Structure
\`\`\`
{project_structure}
\`\`\`

## Code Style
[Example snippet + key conventions]

## Testing Strategy
[Framework, test locations, coverage requirements, test levels]

## Boundaries
- Always: [...]
- Ask first: [...]
- Never: [...]

## Success Criteria
[How we'll know this is done — specific, testable conditions]

## Open Questions
[Anything unresolved that needs human input]`;