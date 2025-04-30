# 📘 Git Rule Specification: Ensure GitLab Username Is Configured

**Rule ID**: `RULE_gitlab-username-setup`  
**Status**: Draft  
**Author**: Himanshu Sharma | bgit Team  
**Created**: 2025-04-26  
**Updated**: 2025-04-26  
**Version**: v1.0.0  
**RuleLevel**: Error

---

## 1. Summary

> Verify that a GitLab username is set in Git configuration or environment before performing GitLab-specific operations.

## 2. Scope

### Applies To:
- [x] Developers (local)  
- [x] CI/CD pipelines  
- [ ] GitHub/GitLab Web UI  
- [x] Hooks (pre-push, pre-fetch)  
- [ ] Git config/templates  

### Affects:
- [x] Pushes  
- [x] Repository layout  
- [ ] Commits  
- [ ] Branching  
- [ ] Merges  
- [ ] Miscellaneous  

### Trigger Point (When to Check):
Before any push or GitLab API interaction (e.g., creating releases via `glab`, tagging, or MR operations).

---

## 3. Motivation

### Problem Statement:
GitLab workflows and API calls (e.g., `glab release create`, merge request pipelines) rely on a valid GitLab username. If unset, automations fail or actions may be attributed to an anonymous or incorrect actor.

### Objectives:
- Ensure accurate attribution in GitLab.  
- Prevent CI/CD failures due to missing user identity.  
- Standardize configuration across environments.

### Common Pitfall:
A developer configures `user.name` and `user.email` but forgets to set `gitlab.user`, causing `glab` commands to error with “No username found.”

---

## 4. Rule Definition

### Description:
This rule checks that `gitlab.user` is defined in Git config (`git config --get gitlab.user`) or via `GITLAB_USER` environment variable.

**Allowed:**  
- Operations when `gitlab.user` (or `GITLAB_USER`) is non-empty.

**Forbidden:**  
- Pushes or GitLab CLI/API calls when no GitLab username is configured.

---

## 5. Examples

### ✅ Correct Usage
```bash
$ git config --get gitlab.user
mygitlabuser
$ git push origin main
# Push succeeds and CI uses “mygitlabuser” for reports
```

### ❌ Incorrect Usage
```bash
$ git config --get gitlab.user

$ glab release create v1.2.0
Error: GitLab username is not configured.
Set it with:
  git config --global gitlab.user "<your-gitlab-username>"
```

---

## 6. Impact Assessment

### Frequency of Violation:
- [x] Rare  
- [ ] Occasional  
- [ ] Frequent  

### Severity When Violated:
- [ ] Pedantic (minor)  
- [ ] Low  
- [x] Medium (workflow blocked)  
- [ ] High  
- [ ] Critical  

---

## 7. Enforcement Strategy

### Pseudocode / Workflow
```bash
# In pre-push hook or CI setup
if [ -z "$(git config --get gitlab.user)" ] && [ -z "$GITLAB_USER" ]; then
  echo "Error: GitLab username is not configured."
  echo "Set it via:"
  echo "  git config --global gitlab.user \"<your-gitlab-username>\""
  echo "or export GITLAB_USER environment variable."
  exit 1
fi
```

### Suggested Tooling:
- Shell-based pre-push hook  
- CI pipeline validation step  
- Custom wrapper around `glab` CLI

---

## 8. Possible Fixes

### Manual Fix:
```bash
git config --global gitlab.user "your-gitlab-username"
# or
export GITLAB_USER="your-gitlab-username"
```

### Automated Fix Suggestions:
None (requires manual input of user identity).

---

## 9. Exceptions & Edge Cases

- CI environments with machine/service accounts set via env var may bypass this rule.  
- Public read-only operations (fetch/clones) can proceed without a username.

---

## 10. Drawbacks

> May block legitimate operations in ephemeral or minimal environments where username is managed outside Git config.

---

## 11. Related Rules / RFCs

- `RULE_github-username-setup`  
- `RULE_gitlab-credentials-http`  
- `RULE_gitlab-credentials-ssh`

---

## 12. Revision History

| Date       | Version | Author           | Notes                        |
|------------|---------|------------------|------------------------------|
| 2025-04-26 | 1.0.0   | Himanshu Sharma  | Initial draft                |

---

## 13. Glossary

| Term | Definition                             |
|------|----------------------------------------|
| CI   | Continuous Integration                 |
| glab | GitLab CLI tool                        |
| Env  | Environment variable (e.g., GITLAB_USER) |

---

## 14. References

- https://github.com/profclems/glab#authentication  
- https://docs.gitlab.com/ee/user/merge_requests/creating_merge_requests.html  
