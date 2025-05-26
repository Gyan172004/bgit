# 📘 Git Rule Specification: Prevent Committing Files Not Listed in .gitignore

**Rule ID**: `RULE_file-not-gitignored`  
**Status**: Draft  
**Author**: Himanshu Sharma | bgit Team  
**Created**: 2025-04-26  
**Updated**: 2025-04-26  
**Version**: v1.0.0  
**RuleLevel**: Error

---

## 1. Summary

> Block commits of files or directories that should be excluded via `.gitignore` but are currently tracked or staged.

---

## 2. Scope

### Applies To:
- [x] Developers (local)  
- [x] CI/CD pipelines  
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
During staging or commit operations (e.g., in a pre-commit hook).

---

## 3. Motivation

### Problem Statement:
Developers sometimes accidentally stage and commit files that should be ignored—build artifacts, temporary files, credentials, OS-specific files—leading to repository clutter, merge conflicts, and potential leaks.

### Objectives:
- Enforce repository cleanliness by preventing unintended files from being versioned.  
- Encourage upkeep of the `.gitignore` to reflect project needs.  
- Reduce noise in diffs and avoid tracking large or sensitive files.

### Common Pitfall:
A developer generates compiled binaries or IDE workspace files and stages them without updating `.gitignore`, then pushes these unwanted files to the central repo.

---

## 4. Rule Definition

### Description:
For each file staged for commit, check whether it matches any pattern in `.gitignore`. If a staged file matches but is still tracked (or newly staged), block the commit.

**Allowed:**  
- Only files not matched by any `.gitignore` entry.

**Forbidden:**  
- Staging or committing files/directories that match `.gitignore` patterns.

---

## 5. Examples

### ✅ Correct Usage
```bash
# .gitignore contains: 
#   dist/
#   *.log

$ git status
Changes not staged for commit:
  modified: src/index.js
# dist/ is ignored, so no dist/ files appear.
```

### ❌ Incorrect Usage
```bash
# .gitignore contains: 
#   node_modules/
#   .env

$ git add node_modules/package-a/index.js
# Staging a file under node_modules even though it's ignored
$ git commit -m "Oops"
Error: Attempted to commit ignored file 'node_modules/package-a/index.js'.  
Please remove it from staging or update .gitignore appropriately.
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
- [x] Medium (requires cleanup)  
- [ ] High (code breakage, data loss)  
- [ ] Critical (security/legal risk)  

---

## 7. Enforcement Strategy

### Pseudocode / Workflow
```bash
# In pre-commit hook
STAGED=$(git diff --cached --name-only --diff-filter=ACM)

for FILE in $STAGED; do
  if git check-ignore -q "$FILE"; then
    echo "Error: '$FILE' is matched by .gitignore but staged."
    echo "Remove it from staging with: git reset $FILE"
    exit 1
  fi
done

exit 0
```

### Suggested Tooling:
- Git built-in `git check-ignore` in pre-commit hooks  
- Husky, Lefthook, or other Git hook managers  
- CI pipeline lint step  

---

## 8. Possible Fixes

### Manual Fix:
```bash
git reset path/to/ignored-file
echo "path/to/ignored-file" >> .gitignore
```

### Automated Fix Suggestions:
- Prompt user to run `git restore --staged <file>` or `git reset <file>`.  
- Provide a CLI command `bgit ignore-fix` to auto-add staged ignored files to `.gitignore` and unstage them.

---

## 9. Exceptions & Edge Cases

- Some legacy files tracked intentionally despite matching ignore patterns—projects can whitelist specific paths in a `.gitignore.allow` file.  
- Generated documentation or compiled assets may be tracked by design; these can be exempted via explicit hook configuration.

---

## 10. Drawbacks

> May block commits in repositories where historically ignored files remain tracked; requires cleanup or whitelist configuration before enforcement.

---

## 11. Related Rules / RFCs

- `RULE_no-secrets-staged`  
- `RULE_gitignore-best-practices`  
- `RULE_big-repo-size`

---

## 12. Revision History

| Date       | Version | Author           | Notes                        |
|------------|---------|------------------|------------------------------|
| 2025-04-26 | 1.0.0   | Himanshu Sharma  | Initial draft                |

---

## 13. Glossary

| Term            | Definition                                                 |
|-----------------|-------------------------------------------------------------|
| `.gitignore`    | File listing patterns for Git to ignore                     |
| `git check-ignore` | Git command to test whether a path is ignored             |
| Staging Area    | Files added via `git add` awaiting commit                   |

---

## 14. References

- https://git-scm.com/docs/git-check-ignore  
- https://git-scm.com/docs/gitignore  
- https://github.com/typicode/husky#readme  
