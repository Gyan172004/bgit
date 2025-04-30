# 📘 Git Rule Specification: `is_git_installed`

**Rule ID**: `RULE_is-git-installed`  
**Status**: Draft  
**Author**: Himanshu Sharma <>
**Created**: 2025-04-26  
**Updated**: 2025-04-26  
**Version**: v1.0.0  
**RuleLevel**: Error

---

## 1. Summary

Ensures that Git is installed on the system before running any Git-related commands.

## 2. Scope

### Applies To:
- [x] Developers (local)
- [ ] CI/CD pipelines
- [ ] GitHub/GitLab Web UI
- [ ] Hooks (pre-commit, pre-push, etc.)
- [ ] Git config/templates

### Affects:
- [x] Commits  
- [x] Branching  
- [x] Merges  
- [x] Pushes  
- [x] Repository layout
- [x] Miscellaneous

### Trigger Point (When to Check):
Before performing any Git-related actions, especially in automated scripts or workflows.

---

## 3. Motivation

### Problem Statement:
Users may attempt to run Git-related operations in environments where Git is not installed. This leads to failed commands, cryptic errors like `command not found`, and disrupted workflows.

### Objectives:
- Prevent failures due to missing Git.
- Ensure that environments are set up properly before running Git commands.
- Provide immediate feedback when Git is absent.

### Common Pitfall:
A developer may clone a repository or run a script assuming Git is installed, only to encounter vague errors due to the tool being unavailable.

---

## 4. Rule Definition

### Description:
This rule requires that Git must be installed on the system before any Git commands can be executed. It will enforce checking for the `git` command and throw an error if Git is not present.

**Allowed:**  
- Running Git commands only in environments where Git is installed.

**Forbidden:**  
- Running Git commands in environments where Git is not installed.

---

## 5. Examples

### ✅ Correct Usage
```bash
$ git --version
git version 2.42.0
```

### ❌ Incorrect Usage
```bash
$ git commit -m "initial commit"
zsh: command not found: git
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
if ! command -v git &> /dev/null
then
    echo "Error: Git is not installed."
    exit 1
fi
```

### Suggested Tooling:
- Shell scripts (for local checks)
- CI pipeline checks
- Custom CLI tools

---

## 8. Possible Fixes

### Manual Fix:
Install Git on the system by following the appropriate installation instructions for your OS:
- For Windows: [Git for Windows](https://git-scm.com/download/win)
- For macOS: `brew install git`
- For Linux: Use your distribution's package manager, e.g., `sudo apt-get install git` for Ubuntu.

### Automated Fix Suggestions:
None (Git installation needs manual intervention).

---

## 9. Exceptions & Edge Cases

- This rule can be bypassed in environments where Git is intentionally not required (e.g., certain isolated development environments).
- Developers may bypass this rule if they are working on non-Git-based projects.

---

## 10. Drawbacks

- Requires users to install Git before running any commands.
- This rule could cause friction in environments where Git is not essential, leading to additional steps or configuration.

---

## 11. Related Rules / RFCs

- None

---

## 12. Revision History

| Date       | Version | Author        | Notes                         |
|------------|---------|---------------|-------------------------------|
| 2025-04-26 | 1.0.0   | Himanshu Sharma | Initial draft                 |

---

## 13. Glossary

| Term | Definition |
|------|------------|
| CI   | Continuous Integration |
| Hook | Git hook script (e.g. `pre-commit`, `pre-push`) |

---

## 14. References

- [Git Installation Documentation](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)
