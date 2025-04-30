# 📘 Git Rule Specification: Prevent Excessively Large Repositories

**Rule ID**: `RULE_big-repo-size`  
**Status**: Draft  
**Author**: Himanshu Sharma | bgit Team  
**Created**: 2025-04-26  
**Updated**: 2025-04-26  
**Version**: v1.0.0  
**RuleLevel**: Warning

---

## 1. Summary

> Warn when a repository’s total size exceeds a configurable threshold to avoid performance and storage issues.

---

## 2. Scope

### Applies To:
- [x] Developers (local)  
- [x] CI/CD pipelines  
- [ ] GitHub/GitLab Web UI  
- [x] Hooks (pre-push, pre-fetch)  
- [ ] Git config/templates  

### Affects:
- [ ] Commits  
- [ ] Branching  
- [ ] Merges  
- [ ] Pushes  
- [ ] Miscellaneous  
- [x] Repository layout  

### Trigger Point (When to Check):
After cloning or fetching, and before pushing or packaging operations.

---

## 3. Motivation

### Problem Statement:
Very large repositories (>100 MB) can slow down operations (clone, fetch, status), consume excessive disk space, and degrade CI/CD performance.

### Objectives:
- Alert users when repo size exceeds a safe limit.  
- Encourage use of Git LFS or history cleanup for large assets.  
- Prevent unexpected CI timeouts or storage overages.

### Common Pitfall:
A project accumulates binaries or dataset files in Git history until cloning takes minutes or fails due to size limits.

---

## 4. Rule Definition

### Description:
Compute repository size via `git count-objects -vH` or by inspecting `.git` folder. Compare “size-pack” or total size against a configured limit (`bigRepoSizeLimit`). If exceeded, emit a warning.

**Allowed:**  
- Repos under the configured size limit.

**Forbidden (Warning):**  
- Repos over the size limit produce a warning but do not block operations.

---

## 5. Examples

### ✅ Correct Usage
```bash
$ git count-objects -vH | grep size-pack
size-pack: 45.23 MiB
# Under default limit (100 MiB) – no warning
```

### ❌ Incorrect Usage
```bash
$ git count-objects -vH | grep size-pack
size-pack: 210.47 MiB
Warning: Repository size (210.47 MiB) exceeds configured limit of 100 MiB.
Consider using Git LFS or removing large files.
```

---

## 6. Impact Assessment

### Frequency of Violation:
- [ ] Rare  
- [x] Occasional  
- [ ] Frequent  

### Severity When Violated:
- [ ] Pedantic (minor)  
- [x] Low (performance warning)  
- [ ] Medium  
- [ ] High  
- [ ] Critical  

---

## 7. Enforcement Strategy

### Pseudocode / Workflow
```bash
# Determine repo pack size in MiB
SIZE=$(git count-objects -vH | awk '/size-pack:/ {print $2}')
LIMIT=100MiB  # default, configurable via bgit.bigRepoSizeLimit

if [[ $(echo "$SIZE > $LIMIT" | bc) -eq 1 ]]; then
  echo "Warning: Repository size ($SIZE) exceeds limit ($LIMIT)."
fi
```

### Suggested Tooling:
- Pre-fetch or post-checkout hook  
- CI pipeline size check step  
- Custom bgit command `bgit check-size`

---

## 8. Possible Fixes

### Manual Fix:
- Move large assets to Git LFS:  
  ```bash
  git lfs install
  git lfs track "*.bin"
  git add .gitattributes
  git commit -m "Track large binaries with LFS"
  ```
- Remove large files from history using `git filter-repo`.

### Automated Fix Suggestions:
- Offer `bgit migrate-lfs` to automatically track oversized files.
- Provide interactive cleanup tool to prune files above threshold.

---

## 9. Exceptions & Edge Cases

- Monorepos or data-centric repos may legitimately exceed limits—users can increase `bgit.bigRepoSizeLimit`.
- CI runners with ephemeral storage may ignore warnings.
- Archives or backup repos intended to be large can disable this rule (`RuleLevel: Skip`).

---

## 10. Drawbacks

> Frequent warnings in large-but-acceptable repos can desensitize users or clutter logs.

---

## 11. Related Rules / RFCs

- `RULE_git-ignore-best-practices`  
- `RULE_no-secrets-staged`  
- `RULE_git-default-config`

---

## 12. Revision History

| Date       | Version | Author           | Notes                         |
|------------|---------|------------------|-------------------------------|
| 2025-04-26 | 1.0.0   | Himanshu Sharma  | Initial draft                 |

---

## 13. Glossary

| Term             | Definition                                                     |
|------------------|-----------------------------------------------------------------|
| Git LFS          | Git Large File Storage extension for handling large files       |
| size-pack        | Size of packed objects in the repository, reported by Git       |
| Threshold        | Configurable size limit (e.g., `100MiB`)                        |

---

## 14. References

- https://git-scm.com/docs/git-count-objects  
- https://github.com/git-lfs/git-lfs  
- https://github.com/newren/git-filter-repo  