# 📘 Git Rule Specification: Ensure Proper Remote URL Scheme (HTTP vs SSH)

**Rule ID**: `RULE_git-remote-http-ssh`  
**Status**: Draft  
**Author**: Himanshu Sharma | bgit Team  
**Created**: 2025-04-26  
**Updated**: 2025-04-26  
**Version**: v1.0.0  
**RuleLevel**: Warning

---

## 1. Summary

> Ensure that the remote origin URL of a Git repository uses the correct scheme (`https` or `ssh`) according to project or organization guidelines.

---

## 2. Scope

### Applies To:
- [x] Developers (local)
- [x] CI/CD pipelines
- [ ] GitHub/GitLab Web UI
- [ ] Hooks (pre-push, pre-fetch)
- [x] Git remote configurations

### Affects:
- [x] Pushes
- [x] Fetches
- [x] Clones
- [ ] Commits
- [ ] Merges
- [ ] Other Git operations

### Trigger Point (When to Check):
When a remote is added, set, or modified.

---

## 3. Motivation

### Problem Statement:
Incorrect remote URL types (HTTP instead of SSH or vice-versa) can cause authentication issues, automation failures, or manual password prompts.

### Objectives:
- Prevent Git authentication errors.
- Enforce consistency across teams and automation systems.
- Ensure secure and reliable Git operations.

### Common Pitfall:
Developers cloning with HTTPS have to manually enter username/password or token repeatedly instead of using SSH keys for seamless auth.

---

## 4. Rule Definition

### Description:
The Git remote `origin` must use the preferred protocol (`ssh` or `https`) based on organizational policy.

| Protocol Preference | Example Remote URL |
|----------------------|--------------------|
| `https`              | `https://github.com/user/repo.git` |
| `ssh`                | `git@github.com:user/repo.git`     |

**Allowed:**  
- Only the configured preferred remote type (either HTTPS or SSH).

**Forbidden:**  
- Mixing different remote URL schemes within the same organization or violating the expected protocol.

---

## 5. Examples

### ✅ Correct Usage (SSH preferred)
```bash
git remote set-url origin git@github.com:user/repo.git
```

### ✅ Correct Usage (HTTPS preferred)
```bash
git remote set-url origin https://github.com/user/repo.git
```

### ❌ Incorrect Usage (when SSH is enforced)
```bash
git remote set-url origin https://github.com/user/repo.git
```

### ❌ Incorrect Usage (when HTTPS is enforced)
```bash
git remote set-url origin git@github.com:user/repo.git
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
# Get the current remote URL
REMOTE_URL=$(git remote get-url origin)

# Preferred protocol (example: ssh)
PREFERRED_PROTOCOL="ssh"

if [ "$PREFERRED_PROTOCOL" = "ssh" ]; then
  if [[ "$REMOTE_URL" != git@* ]]; then
    echo "Warning: Remote URL is not using SSH. Please update it."
  fi
elif [ "$PREFERRED_PROTOCOL" = "https" ]; then
  if [[ "$REMOTE_URL" != https://* ]]; then
    echo "Warning: Remote URL is not using HTTPS. Please update it."
  fi
fi
```

### Suggested Tooling:
- Pre-clone or post-clone script checks
- Onboarding validation scripts
- Git hooks (optional)

---

## 8. Possible Fixes

### Manual Fix:
Update the remote URL manually:
```bash
git remote set-url origin git@github.com:user/repo.git  # for SSH
# or
git remote set-url origin https://github.com/user/repo.git  # for HTTPS
```

### Automated Fix Suggestion:
Provide a script that detects and fixes the remote URL automatically during setup.

---

## 9. Exceptions & Edge Cases

- External public repositories where SSH access is unavailable.
- Personal forks where developers might prefer HTTPS for simplicity.
- Temporary overrides during CI/CD where token authentication is required.

---

## 10. Drawbacks

> Strict enforcement might be inconvenient for personal projects where the other protocol is more practical.

---

## 11. Related Rules / RFCs

- `RULE_github-credentials-http`
- `RULE_github-credentials-ssh`
- `RULE_git-default-config`

---

## 12. Revision History

| Date       | Version | Author           | Notes                        |
|------------|---------|------------------|------------------------------|
| 2025-04-26 | 1.0.0   | Himanshu Sharma   | Initial draft                |

---

## 13. Glossary

| Term          | Definition                                                  |
|---------------|--------------------------------------------------------------|
| Remote URL    | The location of the Git repository that a local repo tracks  |
| SSH           | Secure Shell protocol used for authenticating with Git servers |
| HTTPS         | Hypertext Transfer Protocol Secure, used for Git authentication |

---

## 14. References

- https://git-scm.com/docs/git-remote  
- https://docs.github.com/en/get-started/getting-started-with-git/about-remote-repositories  
- https://docs.github.com/en/authentication/connecting-to-github-with-ssh  
