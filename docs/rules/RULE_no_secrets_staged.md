# 📘 Git Rule Specification: Prevent Staging of Secrets

**Rule ID**: `RULE_no-secrets-staged`  
**Status**: Draft  
**Author**: Himanshu Sharma | bgit Team  
**Created**: 2025-04-26  
**Updated**: 2025-04-26  
**Version**: v1.0.0  
**RuleLevel**: Critical

---

## 1. Summary

> Ensure that no sensitive information like secrets, API keys, private keys, passwords, or tokens are ever staged or committed into the Git repository.

---

## 2. Scope

### Applies To:
- [x] Developers (local)
- [x] CI/CD pipelines
- [ ] GitHub/GitLab Web UI
- [x] Hooks (pre-commit, pre-push)
- [x] Git staging area

### Affects:
- [ ] Pushes
- [ ] Fetches
- [ ] Clones
- [x] Commits
- [ ] Merges
- [ ] Other Git operations

### Trigger Point (When to Check):
During staging or committing files.

---

## 3. Motivation

### Problem Statement:
Accidental commits of secrets expose systems to security breaches, credential leaks, and data loss. History rewriting to remove secrets is cumbersome and dangerous.

### Objectives:
- Stop sensitive data from being version-controlled.
- Enforce a security-first mindset during Git operations.
- Protect systems and users from credential leaks.

### Common Pitfall:
Developers accidentally stage `.env` files, private keys (`id_rsa`), or config files containing tokens without realizing the impact.

---

## 4. Rule Definition

### Description:
Before any commit is finalized, scan staged files for patterns indicating secrets. If found, block the commit and alert the user.

| Item to Detect | Example |
|----------------|---------|
| API keys | `api_key = "AKIA..."` |
| Passwords | `password: mypassword123` |
| Private keys | `-----BEGIN PRIVATE KEY-----` |
| Tokens | `ghp_xxx`, `slackToken`, etc. |

**Allowed:**  
- No detected secret-like patterns in staged files.

**Forbidden:**  
- Any file staged with secrets matching defined regex patterns.

---

## 5. Examples

### ✅ Correct Usage
- Environment variables stored in `.env` (but `.env` is **gitignored**).
- Config files reference environment variables, not hardcoded credentials.

### ❌ Incorrect Usage
```bash
# Staging a file with a secret token inside
git add config.js  # where config.js contains an API token
git commit -m "Add config"
```

---

## 6. Impact Assessment

### Frequency of Violation:
- [ ] Rare  
- [x] Occasional  
- [x] Frequent  

### Severity When Violated:
- [ ] Pedantic (minor)  
- [ ] Low  
- [ ] Medium  
- [x] High  
- [x] Critical  

---

## 7. Enforcement Strategy

### Pseudocode / Workflow
```bash
# Scan staged files for secrets
FILES=$(git diff --cached --name-only)

for FILE in $FILES; do
  if grep -E -i '(api_key|secret|password|token|PRIVATE KEY)' "$FILE"; then
    echo "❌ Potential secret detected in $FILE. Commit aborted."
    exit 1
  fi
done

# Allow commit if no matches
exit 0
```

### Suggested Tooling:
- Pre-commit hooks (Husky, Lefthook)
- Secret scanning tools (git-secrets, truffleHog, gitleaks)

---

## 8. Possible Fixes

### Manual Fix:
- Remove secrets from code.
- Store credentials securely (e.g., environment variables, secret managers).
- Add `.env`, `*.pem`, and sensitive config files to `.gitignore`.

### Automated Fix Suggestion:
Integrate automatic secret scanning tools in local Git hooks and CI/CD pipelines.

---

## 9. Exceptions & Edge Cases

- Dummy test credentials that are safe and explicitly allowed.
- Repositories dedicated solely for examples and testing (with caution).

---

## 10. Drawbacks

> Secret scanning can produce **false positives** (e.g., random test data flagged as a secret).

---

## 11. Related Rules / RFCs

- `RULE_no-sensitive-files`
- `RULE_gitignore-best-practices`

---

## 12. Revision History

| Date       | Version | Author           | Notes                        |
|------------|---------|------------------|------------------------------|
| 2025-04-26 | 1.0.0   | Himanshu Sharma   | Initial draft                |

---

## 13. Glossary

| Term          | Definition                                                  |
|---------------|--------------------------------------------------------------|
| Secrets       | Any confidential information like passwords, API keys, tokens |
| Git Hooks     | Scripts triggered by Git actions like commit, push, etc.       |

---

## 14. References

- https://github.com/awslabs/git-secrets  
- https://github.com/zricethezav/gitleaks  
- https://docs.gitlab.com/ee/user/application_security/secret_detection/
