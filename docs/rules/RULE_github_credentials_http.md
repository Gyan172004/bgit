# 📘 Git Rule Specification: Enforce HTTP(S) Credentials for GitHub

**Rule ID**: `RULE_github-credentials-http`  
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

> Ensure GitHub credentials (over HTTP/HTTPS) are configured before attempting operations that interact with GitHub repositories.

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
At the start of any operation that requires access to a GitHub repository via HTTP(S) (push, fetch, clone).

---

## 3. Motivation

### Problem Statement:
GitHub deprecates password authentication for HTTPS Git operations. Users must use Personal Access Tokens (PAT) or credential helpers. Without credentials, GitHub operations fail with authentication errors (`HTTP 401 Unauthorized`).

### Objectives:
- Prevent failed Git operations due to missing or outdated authentication.
- Educate users to securely set up their credentials over HTTPS.
- Support GitHub's security practices and token-based authentication.

### Common Pitfall:
Users trying to `git push` via HTTPS without a configured credential helper or token set encounter multiple password prompts or error messages.

---

## 4. Rule Definition

### Description:
The system must validate that **HTTP(S)** GitHub credentials are properly configured through:
- Git credential helpers (`cache`, `store`, `osxkeychain`, `wincred`, etc.)
- OR `GITHUB_TOKEN`/`GH_TOKEN` environment variables
- OR authenticated GitHub CLI session (`gh auth status`)

SSH configurations do **not** satisfy this rule (HTTP(S) only).

**Allowed:**  
- GitHub HTTPS interactions with a valid credential caching mechanism or environment token.

**Forbidden:**  
- Missing credential configuration for HTTPS-based operations.

---

## 5. Examples

### ✅ Correct Usage
```bash
$ git config --get credential.helper
cache --timeout=3600

$ git remote -v
origin https://github.com/username/repo.git (fetch)
origin https://github.com/username/repo.git (push)

$ git push origin main
# Successfully authenticates using stored credentials
```

### ❌ Incorrect Usage
```bash
$ git remote -v
origin https://github.com/username/repo.git (fetch)
origin https://github.com/username/repo.git (push)

$ git push origin main
Username for 'https://github.com': 
Password for 'https://username@github.com': 
remote: Invalid username or password.
fatal: Authentication failed for 'https://github.com/username/repo.git/'
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
- [ ] High (critical data loss)  
- [ ] Critical (security/legal risk)  

---

## 7. Enforcement Strategy

### Pseudocode / Workflow
```bash
# Check if HTTPS remotes exist
if git remote -v | grep -q 'https://github.com'; then
  # Check credential.helper
  if ! git config --get credential.helper \
     && [ -z "$GITHUB_TOKEN" ] && [ -z "$GH_TOKEN" ] \
     && ! gh auth status &> /dev/null; then
    echo "Error: No HTTP(S) GitHub credentials configured."
    echo "Set up a credential helper or a GITHUB_TOKEN environment variable."
    exit 1
  fi
fi
```

### Suggested Tooling:
- Pre-push hook script
- CI/CD pipeline pre-check step
- `gh auth status` checks

---

## 8. Possible Fixes

### Manual Fix:
```bash
# Option 1: Configure Git credential helper
git config --global credential.helper cache

# Option 2: Set GitHub token as environment variable
export GITHUB_TOKEN="your_personal_access_token"

# Option 3: Authenticate via GitHub CLI
gh auth login
```

### Automated Fix Suggestions:
Prompt users to authenticate or configure credentials during the setup stage.

---

## 9. Exceptions & Edge Cases

- Public repository **clones** via HTTPS can proceed without credentials.
- SSH-based operations (`git@github.com:username/repo.git`) are exempt (but also flagged for being outside HTTP(S) scope).
- Mirror repositories hosted elsewhere are out of scope.

---

## 10. Drawbacks

> Users might misconfigure short-lived environment variables (e.g., in temporary shell sessions) leading to temporary authentication failures.

---

## 11. Related Rules / RFCs

- `RULE_github-credentials`
- `RULE_git-name-email-setup`

---

## 12. Revision History

| Date       | Version | Author           | Notes                        |
|------------|---------|------------------|------------------------------|
| 2025-04-26 | 1.0.0   | Himanshu Sharma   | Initial draft                |

---

## 13. Glossary

| Term             | Definition                                                  |
|------------------|--------------------------------------------------------------|
| PAT              | Personal Access Token                                        |
| Credential Helper| Mechanism for storing/retrieving Git credentials securely    |
| HTTPS            | HyperText Transfer Protocol Secure (used for GitHub cloning) |

---

## 14. References

- https://docs.github.com/en/get-started/getting-started-with-git/caching-your-github-credentials-in-git  
- https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token  
- https://cli.github.com/manual/gh_auth_login  


