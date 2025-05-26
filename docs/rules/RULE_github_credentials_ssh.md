# 📘 Git Rule Specification: Enforce SSH Credentials for GitHub

**Rule ID**: `RULE_github_credentials_ssh`  
**Status**: Draft  
**Author**: Himanshu Sharma | bgit Team  
**Created**: 2025-04-26  
**Updated**: 2025-04-26  
**Version**: v1.0.0  
**RuleLevel**: Error

---

## 1. Summary

> Ensure GitHub SSH credentials are properly configured before attempting Git operations that interact with GitHub repositories over SSH.

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
At the start of any GitHub operation using an SSH remote URL (`git@github.com:...`).

---

## 3. Motivation

### Problem Statement:
SSH access to GitHub requires a properly configured SSH key and agent. Without valid SSH credentials, operations fail with authentication errors (`Permission denied (publickey)`).

### Objectives:
- Prevent blocked Git operations due to missing SSH keys.
- Enforce secure authentication using GitHub-approved SSH practices.
- Encourage pre-setup of SSH keys for seamless development workflows.

### Common Pitfall:
Developers clone a GitHub repo via SSH without having their SSH key added to their GitHub account or loaded into their SSH agent.

---

## 4. Rule Definition

### Description:
The system must verify that SSH credentials are properly configured through:
- A registered public key on the GitHub account.
- A loaded private key in the SSH agent (`ssh-add -l`).

**Allowed:**  
- GitHub SSH interactions (`git@github.com:username/repo.git`) using a loaded, authorized SSH key.

**Forbidden:**  
- Missing SSH private key or no corresponding authorized public key on GitHub.

---

## 5. Examples

### ✅ Correct Usage
```bash
$ git remote -v
origin git@github.com:username/repo.git (fetch)
origin git@github.com:username/repo.git (push)

$ ssh -T git@github.com
Hi username! You've successfully authenticated, but GitHub does not provide shell access.

$ git push origin main
# Successfully pushes using SSH
```

### ❌ Incorrect Usage
```bash
$ git remote -v
origin git@github.com:username/repo.git (fetch)
origin git@github.com:username/repo.git (push)

$ git push origin main
git@github.com: Permission denied (publickey).
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
- [ ] High (critical data loss)  
- [ ] Critical (security/legal risk)  

---

## 7. Enforcement Strategy

### Pseudocode / Workflow
```bash
# Check if SSH remotes exist
if git remote -v | grep -q 'git@github.com'; then
  # Test SSH connection to GitHub
  if ! ssh -T git@github.com 2>&1 | grep -q "successfully authenticated"; then
    echo "Error: No valid SSH credentials configured for GitHub."
    echo "Ensure your SSH key is added to GitHub and loaded into your SSH agent."
    exit 1
  fi
fi
```

### Suggested Tooling:
- Pre-push hook script
- CI/CD pipeline pre-check step
- SSH agent readiness check

---

## 8. Possible Fixes

### Manual Fix:
```bash
# Option 1: Generate an SSH key if not already available
ssh-keygen -t ed25519 -C "your_email@example.com"

# Option 2: Add SSH key to GitHub
# Copy public key
cat ~/.ssh/id_ed25519.pub

# Paste into GitHub Settings -> SSH and GPG Keys -> New SSH key

# Option 3: Load SSH key into agent
eval "$(ssh-agent -s)"
ssh-add ~/.ssh/id_ed25519
```

### Automated Fix Suggestions:
If missing, prompt user to generate SSH keys or load into agent automatically.

---

## 9. Exceptions & Edge Cases

- Public repository **clones** via SSH require valid credentials (unlike HTTPS).
- Some ephemeral CI runners might use temporary SSH keys – these must be pre-provisioned.
- If fallback authentication (HTTPS) is used intentionally, this rule can be disabled on that specific context.

---

## 10. Drawbacks

> Enforcing SSH only can cause friction for developers who prefer HTTPS workflows or lack key management experience.

---

## 11. Related Rules / RFCs

- `RULE_github_credentials_http`
- `RULE_git_name_email_setup`

---

## 12. Revision History

| Date       | Version | Author           | Notes                        |
|------------|---------|------------------|------------------------------|
| 2025-04-26 | 1.0.0   | Himanshu Sharma   | Initial draft                |

---

## 13. Glossary

| Term              | Definition                                                    |
|-------------------|----------------------------------------------------------------|
| SSH Key           | Public/private key pair used for authentication with GitHub   |
| SSH Agent         | Background process that stores private keys for SSH sessions  |
| Public Key        | Key uploaded to GitHub account to authorize access             |
| Private Key       | Secret key stored securely on the user's machine               |

---

## 14. References

- https://docs.github.com/en/authentication/connecting-to-github-with-ssh/about-ssh  
- https://docs.github.com/en/authentication/connecting-to-github-with-ssh/generating-a-new-ssh-key-and-adding-it-to-the-ssh-agent  
- https://docs.github.com/en/authentication/connecting-to-github-with-ssh/testing-your-ssh-connection  
