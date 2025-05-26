# 📘 Git Rule Specification: Enforce Git LFS Usage for Large Files

**Rule ID**: `RULE_git-lfs`  
**Status**: Draft  
**Author**: Himanshu Sharma | bgit Team  
**Created**: 2025-04-26  
**Updated**: 2025-04-26  
**Version**: v1.0.0  
**RuleLevel**: Warning

---

## 1. Summary

> Ensure that large files (e.g., > 5 MB) are tracked with Git LFS rather than committed directly to the repository.

## 2. Scope

### Applies To:
- [x] Developers (local)  
- [x] CI/CD pipelines  
- [ ] GitHub/GitLab Web UI  
- [x] Hooks (pre-commit, pre-push, etc.)  
- [ ] Git config/templates  

### Affects:
- [x] Commits  
- [ ] Branching  
- [ ] Merges  
- [ ] Pushes  
- [ ] Repository layout  
- [ ] Miscellaneous  

### Trigger Point (When to Check):
Before `git add` or during `pre-commit` / `pre-push` hooks when new or modified files are staged.

---

## 3. Motivation

### Problem Statement:
Committing large binary files directly bloats repository size, slows down clones and fetches, and degrades performance.

### Objectives:
- Prevent oversized files from entering Git history.  
- Offload large file storage to Git LFS.  
- Maintain a lean and performant repository.

### Common Pitfall:
A user stages an image or video over 5 MB without realizing it, then commits and pushes, causing unexpected repository growth.

---

## 4. Rule Definition

### Description:
Detect any staged file whose size exceeds the configured threshold and enforce using Git LFS to track it.

**Allowed:**  
- Staging files ≤ 5 MB by default (threshold configurable).  
- Files > 5 MB tracked via Git LFS (`git lfs track "*.psd"`).

**Forbidden:**  
- Directly committing files larger than the threshold without Git LFS tracking.

---

## 5. Examples

### ✅ Correct Usage
```bash
$ git lfs install
$ git lfs track "*.zip"
$ git add .gitattributes large-file.zip
$ git commit -m "Add large dataset via LFS"
```

### ❌ Incorrect Usage
```bash
$ git add large-video.mp4
$ git commit -m "Add video"
# Warning/Error: File 'large-video.mp4' (12.3 MB) exceeds threshold and is not tracked by Git LFS.
```

---

## 6. Impact Assessment

### Frequency of Violation:
- [ ] Rare  
- [x] Occasional  
- [ ] Frequent  

### Severity When Violated:
- [ ] Pedantic (nice to have)  
- [ ] Low (minor inconvenience)  
- [x] Medium (repository cleanup required)  
- [ ] High (critical data loss)  
- [ ] Critical (security/legal risk)  

---

## 7. Enforcement Strategy

### Pseudocode / Workflow
```bash
# In pre-commit or pre-push hook
THRESHOLD=$((5 * 1024 * 1024))  # 5 MB
for file in $(git diff --cached --name-only); do
  size=$(wc -c < "$file")
  if [ "$size" -gt "$THRESHOLD" ] && ! grep -q "$file" .gitattributes; then
    echo "Error: '$file' ($(numfmt --to=iec $size)) exceeds ${THRESHOLD} bytes and is not tracked by Git LFS."
    exit 1
  fi
done
```

### Suggested Tooling:
- Pre-commit hook script  
- CI pipeline LFS validation step  
- Custom bgit command for scanning large files

---

## 8. Possible Fixes

### Manual Fix:
1. Install Git LFS: `git lfs install`  
2. Track the file type or specific file:
   ```bash
   git lfs track "*.mp4"
   git add .gitattributes
   ```
3. Re-add and commit the large file.

### Automated Fix Suggestions:
- Offer an interactive hook that prompts to track the file with Git LFS and automatically updates `.gitattributes`.

---

## 9. Exceptions & Edge Cases

- Binary files intentionally kept in-repo (e.g., small project assets) below threshold.  
- Threshold may be overridden per-project via a config file.  
- Files already in history before adoption of this rule are exempt from hook enforcement.

---

## 10. Drawbacks

> Developers unfamiliar with Git LFS may face initial onboarding friction. Requires LFS support in CI and storage setup.

---

## 11. Related Rules / RFCs

- `RULE_git-default-config`  
- `RULE_protect-main-branch`  
- `RULE_git-lfs`  

---

## 12. Revision History

| Date       | Version | Author           | Notes                        |
|------------|---------|------------------|------------------------------|
| 2025-04-26 | 1.0.0   | Himanshu Sharma  | Initial draft                |

---

## 13. Glossary

| Term         | Definition                                           |
|--------------|------------------------------------------------------|
| Git LFS      | Git Large File Storage extension for handling large files |
| Threshold    | Maximum allowed file size before requiring LFS       |
| .gitattributes | Git config file specifying LFS-tracked patterns    |

---


