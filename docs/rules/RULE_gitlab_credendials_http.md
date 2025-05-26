# 📘 Git Rule Specification: Enforce HTTP(S) Credentials for GitLab

**Rule ID**: `RULE_gitlab-credentials-http`  
**Status**: Draft  
**Author**: Himanshu Sharma | bgit Team  
**Created**: 2025-04-26  
**Updated**: 2025-04-26  
**Version**: v1.0.0  
**RuleLevel**: Error

---

## 1. Summary

> Ensure GitLab HTTP(S) credentials (Personal Access Token or credential helper) are configured before performing operations that require authentication.

---

## 2. Scope

### Applies To:
- [x] Developers (local)  
- [x] CI/CD pipelines  
- [ ] GitHub/GitLab Web UI  
- [x] Hooks (pre-push, pre-fetch)  
- [ ] Git config/templates  

### Affects:
- [x] Pushes  
- [x] Fetches (private repos)  
- [x] Clones (private repos)  
- [ ] Commits  
- [ ] Merges  
- [ ] Other Git operations  

### Trigger Point (When to Check):
Before any Git operation against a GitLab remote URL using HTTP(S) (e.g., `https://gitlab.com/...`).

---

## 3. Motivation

### Problem Statement:
GitLab requires PAT-based or helper-based authentication for HTTPS Git operations. Without properly configured credentials, operations fail with `HTTP 401 Unauthorized` or repeated credential prompts.

### Objectives:
- Prevent authentication failures and stalled workflows.  
- Guide users to securely configure HTTP(S) credentials.  
- Align with GitLab’s deprecation of password-based authentication.

### Common Pitfall:
A developer attempts `git push` via HTTPS on a private GitLab repo without setting up a PAT or credential helper, leading to endless username/password prompts.

---

## 4. Rule Definition

### Description:
Validate that at least one HTTP(S) authentication method is configured:
- A Git credential helper (`cache`, `store`, `osxkeychain`, `wincred`, etc.)  
- A `GITLAB_TOKEN` or `GL_TOKEN` environment variable  
- An authenticated GitLab CLI session (`glab auth status` succeeds)

SSH-based remotes do **not** satisfy this rule.

**Allowed:**  
- HTTP(S) GitLab interactions when credentials are present.

**Forbidden:**  
- Any HTTPS operation on GitLab without valid credentials.

---

## 5. Examples

### ✅ Correct Usage
```bash
$ git config --get credential.helper
store

$ git remote -v
origin https://gitlab.com/group/project.git (fetch)
origin https://gitlab.com/group/project.git (push)

$ git push origin main
# Push succeeds using stored credentials
```

### ❌ Incorrect Usage
```bash
$ git remote -v
origin https://gitlab.com/group/project.git (fetch)
origin https://gitlab.com/group/project.git (push)

$ git push origin main
Username for 'https://gitlab.com': 
Password for 'https://username@gitlab.com': 
remote: HTTP Basic: Access denied
fatal: Authentication failed for 'https://gitlab.com/group/project.git/'
```

---

## 6. Impact Assessment

### Frequency of Violation:
- [ ] Rare  
- [x] Occasional  
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
if git remote -v | grep -q 'https://gitlab.com'; then
  if ! git config --get credential.helper \
     && [ -z "$GITLAB_TOKEN" ] && [ -z "$GL_TOKEN" ] \
     && ! glab auth status &> /dev/null; then
    echo "Error: No HTTP(S) credentials configured for GitLab."
    echo "Configure a credential helper or set GITLAB_TOKEN/GL_TOKEN."
    exit 1
  fi
fi
```

### Suggested Tooling:
- Pre-push hook  
- CI pipeline credential check step  
- GitLab CLI (`glab auth status`)  

---

## 8. Possible Fixes

### Manual Fix:
```bash
# Option 1: Configure Git credential helper
git config --global credential.helper cache

# Option 2: Set environment variable
export GITLAB_TOKEN="your_personal_access_token"

# Option 3: Authenticate via GitLab CLI
glab auth login
```

### Automated Fix Suggestions:
None (requires secure input of PAT).

---

## 9. Exceptions & Edge Cases

- Public repository clones via HTTPS do not require credentials.  
- SSH-based workflows are exempt but should be covered by `RULE_gitlab-credentials-ssh`.  
- CI environments with injected tokens can disable this rule via configuration.

---

## 10. Drawbacks

> May require additional setup for new contributors unfamiliar with PATs or credential helpers, increasing onboarding complexity.

---

## 11. Related Rules / RFCs

- `RULE_gitlab-credentials-ssh`  
- `RULE_gitlab-credentials-http`  

---

## 12. Revision History

| Date       | Version | Author           | Notes                        |
|------------|---------|------------------|------------------------------|
| 2025-04-26 | 1.0.0   | Himanshu Sharma  | Initial draft                |

---

## 13. Glossary

| Term               | Definition                                                   |
|--------------------|---------------------------------------------------------------|
| PAT                | Personal Access Token                                        |
| Credential Helper  | Git mechanism for caching or storing credentials securely    |
| HTTPS              | Hypertext Transfer Protocol Secure                           |
| GLab               | GitLab CLI tool                                              |

---

## 14. References

- https://docs.gitlab.com/ee/user/gitlab_com/#http-basic-authentication  
- https://docs.gitlab.com/ee/user/project/deploy_tokens/  
- https://github.com/profclems/glab#authentication  
