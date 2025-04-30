# 📘 Git Rule Specification: Require GPG-Signed Commits

**Rule ID**: `RULE_commit-gpg-sign`  
**Status**: Draft  
**Author**: Himanshu Sharma | bgit Team  
**Created**: 2025-04-26  
**Updated**: 2025-04-26  
**Version**: v1.0.0  
**RuleLevel**: Error

---

## 1. Summary

> Enforce that all commits are signed with a valid GPG key to guarantee author authenticity and integrity.

---

## 2. Scope

### Applies To:
- [x] Developers (local)  
- [x] CI/CD pipelines (when verifying history)  
- [ ] GitHub/GitLab Web UI  
- [x] Hooks (pre-commit, pre-push)  
- [ ] Git config/templates  

### Affects:
- [x] Commits  
- [ ] Branching  
- [ ] Merges  
- [ ] Pushes  
- [ ] Repository layout  
- [ ] Miscellaneous  

### Trigger Point (When to Check):
During commit creation (pre-commit or commit-msg hook) and optionally during pushes (pre-push) when verifying history.

---

## 3. Motivation

### Problem Statement:
Unsigned commits can be forged or come from unauthorized contributors. Without GPG signatures, it’s difficult to verify that a commit truly originates from the stated author.

### Objectives:
- Guarantee commit authenticity and non-repudiation.  
- Provide a cryptographic audit trail of authorship.  
- Protect against tampering and impersonation.

### Common Pitfall:
Developers forget to add `-S` to `git commit` or haven’t configured their GPG key, resulting in unsigned commits.

---

## 4. Rule Definition

### Description:
Every commit must include a GPG signature. Validate via `git verify-commit` or check `commit.gpgSign` configuration.

**Allowed:**  
- Commits signed with a GPG key listed in `user.signingKey` and trusted.

**Forbidden:**  
- Unsigned commits or commits signed with an untrusted key.

---

## 5. Examples

### ✅ Correct Usage
```bash
# Ensure signing is configured
git config --global user.signingKey "ABCD1234EFGH5678"
git config --global commit.gpgSign true

# Create a signed commit
git commit -S -m "Add feature X"
```

### ❌ Incorrect Usage
```bash
# Unsigned commit
git commit -m "Update README"
# Error:
#   ERROR: Commit is not GPG-signed. Please configure and use `-S`.
```

---

## 6. Impact Assessment

### Frequency of Violation:
- [ ] Rare  
- [x] Occasional  
- [ ] Frequent  

### Severity When Violated:
- [x] Pedantic (minor)  
- [ ] Low (minor inconvenience)  
- [ ] Medium (requires remediation)  
- [ ] High (security risk)  
- [ ] Critical (legal risk)  

---

## 7. Enforcement Strategy

### Pseudocode / Workflow
```bash
# In a pre-commit or commit-msg hook
if ! git rev-parse --verify HEAD >/dev/null 2>&1; then
  PARENT_HASH=$(git hash-object -t tree /dev/null)  # first commit
else
  PARENT_HASH=HEAD
fi

# After commit is made, verify signature
if ! git verify-commit HEAD >/dev/null 2>&1; then
  echo "Error: Commit is not GPG-signed or signature is invalid."
  exit 1
fi
```

### Suggested Tooling:
- git’s built-in `commit.gpgSign` config  
- `pre-commit` or `commit-msg` hook scripts  
- CI pipeline `git verify-*` checks

---

## 8. Possible Fixes

### Manual Fix:
```bash
# Configure and use signing
git config --global user.signingKey "ABCD1234EFGH5678"
git config --global commit.gpgSign true
git commit -S -m "Your message"
```

### Automated Fix Suggestions:
- Enable `commit.gpgSign` in global config so `-S` is automatic.  
- Provide an onboarding script to generate or import GPG keys and configure Git.

---

## 9. Exceptions & Edge Cases

- Exempt initial empty commits when no parent exists, if signature verification fails.  
- Machine-generated commits in CI may skip signing but should be signed with a CI-specific key.

---

## 10. Drawbacks

> Requires contributors to manage GPG keys and trust configurations, which can be complex for new users.

---

## 11. Related Rules / RFCs

- `RULE_git-name-email-setup`  
- `RULE_git-default-config`

---

## 12. Revision History

| Date       | Version | Author           | Notes                        |
|------------|---------|------------------|------------------------------|
| 2025-04-26 | 1.0.0   | Himanshu Sharma  | Initial draft                |

---

## 13. Glossary

| Term           | Definition                                                     |
|----------------|-----------------------------------------------------------------|
| GPG            | GNU Privacy Guard, a tool for encryption and signing            |
| commit.gpgSign | Git configuration to automatically sign commits                 |
| verify-commit  | Git command to verify a commit’s GPG signature                  |

---

## 14. References

- https://git-scm.com/docs/git-commit#Documentation/git-commit.txt---Sltgpgsigngt  
- https://git-scm.com/book/en/v2/Git-Tools-Signing-Your-Work  
- https://gnupg.org/documentation/  