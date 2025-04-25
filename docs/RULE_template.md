# 📘 Git Rule Specification: <Rule Title>

**Rule ID**: `RULE_<kebab-case-rule-name>`  
**Status**: Draft / Final  
**Author**: <Your Name / Team>  
**Created**: YYYY-MM-DD  
**Updated**: YYYY-MM-DD  
**Version**: v1.0.0  

---

## 1. Summary

> A short, one-liner describing the intent and scope of this rule.

## 2. Scope

### Applies To:
- [ ] Developers (local)
- [ ] CI/CD pipelines
- [ ] GitHub/GitLab Web UI
- [ ] Hooks (pre-commit, pre-push, etc.)
- [ ] Git config/templates

### Affects:
- [ ] Commits  
- [ ] Branching  
- [ ] Merges  
- [ ] Pushes  
- [ ] Repository layout
- [ ] Miscellaneous

### Trigger Point (When to Check):
> _e.g., Before `git add`, After `git commit`, Before push, etc._

- **Example**: This rule must run **before** `git add` to prevent staging `.env` files.

- Refer to [`../src/events`](../src/events/) for the complete list of possible trigger events. If the relevant trigger is not listed, explicitly mention it and consider updating the reference list.

## 3. Motivation

### Problem Statement:
Explain the problem or risks this rule is designed to prevent.

### Objectives:
- Goal 1  
- Goal 2  
- Goal 3  

### Common Pitfall:
Example of how someone might accidentally break the rule.

## 4. Rule Definition

### Description:
Explain **what is required** and **what is prohibited**.

**Allowed:**  
- …

**Forbidden:**  
- …

## 5. Examples

### ✅ Correct Usage
```bash
# Example command or snippet that follows the rule
```

### ❌ Incorrect Usage
```bash
# Example of violating the rule
```

## 6. Impact Assessment

### Frequency of Violation:
- [ ] Rare  
- [ ] Occasional  
- [ ] Frequent  

### Severity When Violated:
- [ ] Pedantic (nice to have)  
- [ ] Low (minor inconvenience)  
- [ ] Medium (requires cleanup)  
- [ ] High (code breakage, data loss)  
- [ ] Critical (security/legal risk)

## 7. Enforcement Strategy

### Pseudocode / Workflow
```bash
# Example enforcement logic
<your script or logic here>
```

### Suggested Tooling:
- Git hooks (shell, Python, etc.)
- CI pipeline checks
- Regex-based matchers
- Static analysis/linting
- Custom CLI scripts

## 8. Exceptions & Edge Cases

- When can this rule be bypassed?  
- Who can bypass it?  
- Are there files, branches, or roles exempt?  
- Add any whitelisted scenarios explicitly.

## 9. Drawbacks

> List any trade-offs or potential downsides of enforcing this rule.  
> Could it cause friction or false positives?

---

## 🧷 10. Related Rules / RFCs

- <Add any related or dependent rule files>

---

## 🕘 11. Revision History

| Date       | Version | Author        | Notes                         |
|------------|---------|---------------|-------------------------------|
| YYYY-MM-DD | 1.0.0   | Your Name     | Initial draft                 |
| YYYY-MM-DD | 1.0.1   | Contributor X | Added CI enforcement snippet |

---

## 📖 12. Glossary

| Term | Definition |
|------|------------|
| CI   | Continuous Integration |
| Hook | Git hook script (e.g. `pre-commit`, `pre-push`) |
| …    | … |

---

## 🔗 13. References

- <any relevant link or resource>
---

<!-- 
💡 Usage Instructions:
Save each rule as a separate file under a `docs/rules/` directory:
- RULE_no-binary-commits.md
- RULE_branch-name-convention.md
- RULE_protect-main-branch.md
...etc.
-->