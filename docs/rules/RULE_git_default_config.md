# 📘 Git Rule Specification: Enforce Default Git Configurations

**Rule ID**: `RULE_git-default-config`  
**Status**: Draft  
**Author**: Himanshu Sharma | bgit Team  
**Created**: 2025-04-26  
**Updated**: 2025-04-26  
**Version**: v1.0.0  
**RuleLevel**: Warning

---

## 1. Summary

> Ensure essential Git default configurations are properly set to maintain consistent behavior across all environments and repositories.

## 2. Scope

### Applies To:
- [x] Developers (local)  
- [x] CI/CD pipelines  
- [ ] GitHub/GitLab Web UI  
- [ ] Hooks (pre-push, pre-fetch)  
- [x] Git config/templates  

### Affects:
- [x] Pushes  
- [x] Fetches  
- [x] Clones  
- [x] Commits  
- [x] Merges  
- [ ] Other Git operations

### Trigger Point (When to Check):
When initializing a repository or before performing Git operations.

---

## 3. Motivation

### Problem Statement:
Git operations can behave unpredictably if essential configurations (like default branch name, pull behavior, etc.) are not explicitly set.

### Objectives:
- Prevent inconsistent Git behaviors across environments.
- Ensure a standardized experience for all team members.
- Avoid merge errors or unexpected pull behaviors.

### Common Pitfall:
Developers face unexpected merge commits or conflict because `pull.rebase` isn't configured properly, or experience different default branches (`master` vs `main`).

---

## 4. Rule Definition

### Description:
The system must ensure the following Git default configurations are explicitly set:

| Config Key         | Recommended Value   |
|--------------------|----------------------|
| `init.defaultBranch` | `main`                |
| `pull.rebase`        | `false` or `true` (team standard) |
| `core.autocrlf`      | `input` (for Unix) or `true` (for Windows) |
| `core.safecrlf`      | `true`               |
| `color.ui`           | `auto`               |

**Allowed:**  
- Explicitly set these values in the global or repository config.

**Forbidden:**  
- Missing or default unset values leading to unpredictable behavior.

---

## 5. Examples

### ✅ Correct Usage
```bash
git config --global init.defaultBranch main
git config --global pull.rebase false
git config --global core.autocrlf input
git config --global core.safecrlf true
git config --global color.ui auto
```

### ❌ Incorrect Usage
```bash
git config --global --unset init.defaultBranch
# Or
git config --global pull.rebase  # not set
# Or
# core.autocrlf not set correctly (especially on Windows)
```

---

## 6. Impact Assessment

### Frequency of Violation:
- [ ] Rare  
- [x] Occasional  
- [ ] Frequent  

### Severity When Violated:
- [ ] Pedantic (minor)  
- [x] Low  
- [ ] Medium  
- [ ] High  
- [ ] Critical  

---

## 7. Enforcement Strategy

### Pseudocode / Workflow
```bash
# Check if default branch is set
if [ "$(git config --global init.defaultBranch)" != "main" ]; then
  echo "Warning: Default branch should be set to 'main'."
fi

# Check pull.rebase
if [ -z "$(git config --global pull.rebase)" ]; then
  echo "Warning: Git pull behavior (pull.rebase) is not configured."
fi

# Check core.autocrlf
if [ -z "$(git config --global core.autocrlf)" ]; then
  echo "Warning: core.autocrlf should be configured depending on OS."
fi
```

### Suggested Tooling:
- Post-install script
- Pre-init Git hook
- Linter tool for Git configuration

---

## 8. Possible Fixes

### Manual Fix:
```bash
git config --global init.defaultBranch main
git config --global pull.rebase false
git config --global core.autocrlf input  # or true if on Windows
git config --global core.safecrlf true
git config --global color.ui auto
```

### Automated Fix Suggestions:
Provide a one-click script or onboarding setup that sets all defaults at once.

---

## 9. Exceptions & Edge Cases

- Some legacy projects might require `master` instead of `main`.
- On Windows, `core.autocrlf true` may be necessary instead of `input`.
- Advanced users may customize `pull.rebase` to suit their branching strategies.

---

## 10. Drawbacks

> Enforcing strict defaults might conflict with user preferences in personal projects.

---

## 11. Related Rules / RFCs

- `RULE_git-name-email-setup`
- `RULE_github-credentials-http`
- `RULE_github-credentials-ssh`

---

## 12. Revision History

| Date       | Version | Author           | Notes                        |
|------------|---------|------------------|------------------------------|
| 2025-04-26 | 1.0.0   | Himanshu Sharma   | Initial draft                |

---

## 13. Glossary

| Term              | Definition                                                  |
|-------------------|--------------------------------------------------------------|
| Default Branch    | The initial branch name when creating a new Git repository   |
| Pull Rebase       | Whether `git pull` automatically rebases instead of merging  |
| Autocrlf          | Automatic conversion between LF and CRLF line endings        |

---

## 14. References

- https://git-scm.com/docs/git-config  
- https://docs.github.com/en/get-started/quickstart/set-up-git  
- https://docs.github.com/en/get-started/quickstart/configuring-git-to-handle-line-endings  
