
# 📘 Git Rule Specification: Ensure GitHub Username Is Configured

**Rule ID**: `RULE_github-username-setup`  
**Status**: Draft  
**Author**: Himanshu Sharma | bgit Team  
**Created**: 2025-04-26  
**Updated**: 2025-04-26  
**Version**: v1.0.0  
**RuleLevel**: Error

<!--  
RuleLevel determines how strictly the rule is enforced:

- `Skip`: The rule is not checked or enforced. Useful for opt-out rules.
- `Warning`: Violations produce a warning and optionally attempt auto-fix, but the operation continues.
- `Error`: Violations cause the operation to fail unless auto-fixed successfully.
-->

---

## 1. Summary

> Verify that a GitHub username is configured in Git settings before performing GitHub-specific operations.

## 2. Scope

### Applies To:
- [x] Developers (local)  
- [x] CI/CD pipelines  
- [ ] GitHub/GitLab Web UI  
- [x] Hooks (pre-commit, pre-push, etc.)  
- [ ] Git config/templates  

### Affects:
- [x] Pushes  
- [x] Repository layout  
- [ ] Commits  
- [ ] Branching  
- [ ] Merges  
- [ ] Miscellaneous  

### Trigger Point (When to Check):
Before any push or GitHub API interaction (e.g. creating a release, opening a PR).

---

## 3. Motivation

### Problem Statement:
GitHub-specific workflows (CI status reports, release tagging, contribution graphs) rely on a valid GitHub username. If `github.user` is unset, automations may fail or attribute actions to “ghost” users.

### Objectives:
- Ensure GitHub workflows correctly identify the actor.  
- Prevent automation failures due to missing `github.user`.  
- Maintain accurate contribution attribution in GitHub.

### Common Pitfall:
A developer sets up a new machine, configures `user.name` and `user.email`, but forgets to set `github.user`, causing CI scripts (e.g., `gh release create`) to error out with “No username found.”

---

## 4. Rule Definition

### Description:
This rule checks that `github.user` is set in Git configuration (`git config --get github.user`) or via the `GITHUB_USER` environment variable.

**Allowed:**  
- Operations when `github.user` (or `GITHUB_USER`) is non-empty.

**Forbidden:**  
- Pushes or GitHub API calls when `github.user` is unset or empty.

---

## 5. Examples

### ✅ Correct Usage
```bash
$ git config --get github.user
octocat
$ git push origin main
# Push succeeds, CI uses “octocat” for status reporting
```

### ❌ Incorrect Usage
```bash
$ git config --get github.user

$ gh release create v1.0.0
Error: GitHub username is not configured. Set it with:
  git config --global github.user "<your-github-username>"
```

---

## 6. Impact Assessment

### Frequency of Violation:
- [x] Rare  
- [ ] Occasional  
- [ ] Frequent  

### Severity When Violated:
- [ ] Pedantic (nice to have)  
- [ ] Low (minor inconvenience)  
- [ ] Medium (requires cleanup)  
- [x] High (code breakage, data loss)  
- [ ] Critical (security/legal risk)  

---

## 7. Enforcement Strategy

### Pseudocode / Workflow
```bash
# In pre-push hook or CI setup
if [ -z "$(git config --get github.user)" ] && [ -z "$GITHUB_USER" ]; then
  echo "Error: GitHub username is not configured."
  echo "Set it via:"
  echo "  git config --global github.user \"<your-github-username>\""
  echo "or export GITHUB_USER environment variable."
  exit 1
fi
```

### Suggested Tooling:
- POSIX-shell pre-push hook  
- CI pipeline step before GitHub API calls  
- Custom CLI wrapper around `gh` or push commands

---

## 8. Possible Fixes

### Manual Fix:
```bash
git config --global github.user "your-github-username"
# or
export GITHUB_USER="your-github-username"
```

### Automated Fix Suggestions:
None (requires user input of personal GitHub username).

---

## 9. Exceptions & Edge Cases

- Bypass allowed in CI contexts where a machine/service account is pre-configured via environment variables.  
- Projects explicitly using a different environment variable name for GitHub identity may exempt this rule.

---

## 10. Drawbacks

> This rule may block automated workflows if not properly configured, adding an extra setup step for new environments.

---

## 11. Related Rules / RFCs

- `RULE_git-name-email-setup`  
- `RULE_git-version-consistency`

---

## 12. Revision History

| Date       | Version | Author           | Notes                        |
|------------|---------|------------------|------------------------------|
| 2025-04-26 | 1.0.0   | Himanshu Sharma  | Initial draft                |

---

## 13. Glossary

| Term | Definition                                      |
|------|-------------------------------------------------|
| CI   | Continuous Integration                          |
| Hook | Git hook script (e.g. `pre-commit`, `pre-push`) |
| GH   | GitHub CLI (`gh`)                               |

---

## 14. References

- https://cli.github.com/manual/gh_release_create  
- https://git-scm.com/docs/git-config#Documentation/git-config.txt---getltnamegt  
