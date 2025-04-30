# 📘 Git Rule Specification: Enforce SSH Credentials for GitLab

**Rule ID**: `RULE_gitlab-credentials-ssh`  
**Status**: Draft  
**Author**: Himanshu Sharma | bgit Team  
**Created**: 2025-04-26  
**Updated**: 2025-04-26  
**Version**: v1.0.0  
**RuleLevel**: Error

---

## 1. Summary

> Ensure GitLab SSH credentials are properly configured before attempting Git operations against GitLab repositories over SSH.

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
- [x] Fetches  
- [x] Clones  
- [ ] Commits  
- [ ] Merges  
- [ ] Other Git operations  

### Trigger Point (When to Check):
At the start of any Git operation using a GitLab SSH remote URL (`git@gitlab.com:...`).

---

## 3. Motivation

### Problem Statement:
SSH access to GitLab requires a valid SSH key and agent setup. Without properly configured SSH credentials, operations fail with `Permission denied (publickey)` and block development workflows.

### Objectives:
- Prevent authentication failures with GitLab over SSH.  
- Encourage secure SSH key practices.  
- Streamline developer setup for SSH-based GitLab access.

### Common Pitfall:
A developer clones a GitLab repo via SSH but hasn’t added their SSH public key to their GitLab profile or loaded their private key into the SSH agent.

---

## 4. Rule Definition

### Description:
Verify that SSH credentials are correctly configured for GitLab by checking:
1. A registered public key in the user’s GitLab account.  
2. A loaded private key in the SSH agent (`ssh-add -l`).

**Allowed:**  
- GitLab SSH operations (`git@gitlab.com:group/project.git`) when a valid SSH key is registered and loaded.

**Forbidden:**  
- Any SSH operation against GitLab without a valid, loaded key.

---

## 5. Examples

### ✅ Correct Usage
```bash
$ git remote -v
origin git@gitlab.com:mygroup/myrepo.git (fetch)
origin git@gitlab.com:mygroup/myrepo.git (push)

$ ssh -T git@gitlab.com
Welcome to GitLab, @username!

$ git push origin main
# Push succeeds via SSH
```

### ❌ Incorrect Usage
```bash
$ git remote -v
origin git@gitlab.com:mygroup/myrepo.git (fetch)
origin git@gitlab.com:mygroup/myrepo.git (push)

$ git push origin main
git@gitlab.com: Permission denied (publickey).
fatal: Could not read from remote repository.
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
# Check for GitLab SSH remotes
if git remote -v | grep -q 'git@gitlab.com'; then
  # Test SSH connection
  if ! ssh -T git@gitlab.com 2>&1 | grep -q "Welcome to GitLab"; then
    echo "Error: SSH credentials for GitLab are not configured or loaded."
    echo "1. Add your public key to GitLab Profile → SSH Keys."
    echo "2. Load your key: ssh-add ~/.ssh/id_ed25519"
    exit 1
  fi
fi
```

### Suggested Tooling:
- Pre-push hook  
- CI pipeline SSH readiness check  
- SSH agent validation script

---

## 8. Possible Fixes

### Manual Fix:
```bash
# Generate or use existing key
ssh-keygen -t ed25519 -C "you@example.com"

# Add to SSH agent
eval "$(ssh-agent -s)"
ssh-add ~/.ssh/id_ed25519

# Copy and paste public key
cat ~/.ssh/id_ed25519.pub
# Add it under GitLab → Profile Settings → SSH Keys
```

### Automated Fix Suggestions:
- Onboarding script to generate, load, and register SSH key via GitLab API.

---

## 9. Exceptions & Edge Cases

- CI runners with pre-provisioned SSH keys must set `RuleLevel: Skip` or configure agent automatically.  
- Public projects cloned via SSH still require a valid key—even for read operations.

---

## 10. Drawbacks

> Strict SSH enforcement may inconvenience developers preferring HTTPS or unfamiliar with SSH key management.

---

## 11. Related Rules / RFCs

- `RULE_github-credentials-ssh`  
- `RULE_git-remote-http-ssh`

---

## 12. Revision History

| Date       | Version | Author           | Notes                        |
|------------|---------|------------------|------------------------------|
| 2025-04-26 | 1.0.0   | Himanshu Sharma  | Initial draft                |

---

## 13. Glossary

| Term      | Definition                                                     |
|-----------|-----------------------------------------------------------------|
| SSH Key   | Public/private key pair used for GitLab authentication          |
| SSH Agent | Service that holds private keys in memory for SSH connections   |
| GitLab    | Self-hosted or SaaS Git platform (gitlab.com)                  |

---

## 14. References

- https://docs.gitlab.com/ee/user/ssh.html  
- https://docs.gitlab.com/ee/ssh/README.html  
- https://git-scm.com/book/en/v2/Git-on-the-Server-Generating-Your-SSH-Public-Key  
