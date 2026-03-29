---
phase: quick-260329-pdh
plan: 01
type: execute
wave: 1
depends_on: []
files_modified: [scripts/release.sh]
autonomous: true
must_haves:
  truths:
    - "Script reads version from Cargo.toml automatically"
    - "Script creates git tag with v{version} format"
    - "Script pushes tag to origin to trigger release workflow"
  artifacts:
    - path: "scripts/release.sh"
      provides: "Release script that creates and pushes tags"
      min_lines: 15
  key_links:
    - from: "scripts/release.sh"
      to: "Cargo.toml"
      via: "grep version"
---

<objective>
Create a release script that automates tag creation and pushing to trigger the GitHub Actions release workflow.

Purpose: Simplify the release process by automating version extraction, tag creation, and pushing.
Output: A `scripts/release.sh` file that reads version from Cargo.toml, creates tag v{version}, and pushes to origin.
</objective>

<execution_context>
@$HOME/.claude/get-shit-done/workflows/execute-plan.md
@$HOME/.claude/get-shit-done/templates/summary.md
</execution_context>

<context>
@Cargo.toml
@.github/workflows/release.yml

The release workflow triggers on tags matching "v*":
```yaml
on:
  push:
    tags:
      - "v*"
```

Current version in Cargo.toml: 0.1.0
</context>

<tasks>

<task type="auto">
  <name>Create release script</name>
  <files>scripts/release.sh</files>
  <action>
Create a shell script at `scripts/release.sh` that:

1. Extracts the version from Cargo.toml using grep/cut or similar
2. Validates the version was found
3. Creates a git tag in format v{version} (e.g., v0.1.0)
4. Pushes the tag to origin to trigger the release workflow
5. Outputs confirmation messages at each step

The script should:
- Start with `#!/bin/bash` and `set -e` to fail on errors
- Handle errors gracefully with informative messages
- Use `git describe --tags` or similar to verify tag doesn't already exist
- Push with `git push origin v{version}`

Make the script executable with chmod +x.
  </action>
  <verify>
    <automated>bash -n scripts/release.sh && test -x scripts/release.sh</automated>
  </verify>
  <done>
    - scripts/release.sh exists and is executable
    - Script correctly extracts version from Cargo.toml
    - Script creates tag v{version} and pushes to origin
    - Script has proper error handling
  </done>
</task>

</tasks>

<verification>
- Run `bash -n scripts/release.sh` to verify syntax
- Verify script is executable with `test -x scripts/release.sh`
- Script correctly reads version from Cargo.toml (e.g., 0.1.0)
- Script creates tag in v{version} format
- Script pushes to origin
</verification>

<success_criteria>
- scripts/release.sh exists and is executable
- Script extracts version from Cargo.toml
- Script creates tag v{version}
- Script pushes tag to origin
- Script has informative output and error handling
</success_criteria>

<output>
After completion, create `.planning/quick/260329-pdh-release-tag-push/260329-pdh-SUMMARY.md`
</output>
