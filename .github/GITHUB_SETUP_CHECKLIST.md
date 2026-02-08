# GitHub Setup Checklist for elo-rust

Quick reference for setting up the elo-rust repository on GitHub.

## 1. Repository Secrets (⚠️ REQUIRED FOR PUBLISHING)

### CARGO_REGISTRY_TOKEN

**Steps:**
1. Get token from crates.io:
   - Go: https://crates.io/me
   - Click: "API Tokens"
   - Create new token
   - Copy the token (⚠️ only shown once!)

2. Add to GitHub:
   - Go: https://github.com/evoludigit/elo-rust/settings/secrets/actions
   - Click: "New repository secret"
   - Name: `CARGO_REGISTRY_TOKEN`
   - Value: [paste token from crates.io]
   - Click: "Add secret"

3. Verify:
   - Should appear in settings/secrets/actions
   - Should not be visible after creation

**Alternative (using GitHub CLI)**:
```bash
gh secret set CARGO_REGISTRY_TOKEN --repo evoludigit/elo-rust \
  --body "your-crates-io-token-here"
```

**Verify with CLI**:
```bash
gh secret list --repo evoludigit/elo-rust
```

## 2. GitHub Settings

### Branch Protection

1. Go: https://github.com/evoludigit/elo-rust/settings/branches
2. Add rule for `main`:
   - Require status checks to pass: ✅
   - Require code review before merge: ✅
   - Require CODEOWNERS review: (optional)
   - Dismiss stale reviews: ✅
   - Require branches up to date before merge: ✅
   - Require status checks (select):
     - CI / Test Suite
     - CI / Clippy
     - CI / Rustfmt
     - CI / Build
     - Security / Cargo Audit
     - Security / Cargo Deny

### Require Signed Commits (Optional)

1. Go: https://github.com/evoludigit/elo-rust/settings/branches
2. Edit `main` rule
3. Check: "Require signed commits"

### Code Scanning

1. Go: https://github.com/evoludigit/elo-rust/security/code-scanning
2. Status should show:
   - ✅ Trivy (from security.yml)
   - ✅ Secret scanning (GitHub native)
   - ✅ Dependabot alerts (GitHub native)

### Dependabot

1. Go: https://github.com/evoludigit/elo-rust/settings/security_analysis
2. Enable:
   - ✅ Dependabot alerts
   - ✅ Dependabot security updates
   - ✅ Secret scanning

## 3. Labels & Milestones

### Create Labels

```bash
# Security labels
gh label create security \
  --description "Security related issue" \
  --color FF0000 \
  --repo evoludigit/elo-rust

gh label create security/critical \
  --description "Critical security issue" \
  --color DD0000 \
  --repo evoludigit/elo-rust

# Other useful labels
gh label create bug --color FF6600 --repo evoludigit/elo-rust
gh label create enhancement --color 0366D6 --repo evoludigit/elo-rust
gh label create documentation --color 0075CA --repo evoludigit/elo-rust
gh label create testing --color FBCA04 --repo evoludigit/elo-rust
```

## 4. Automation Features

### Enable GitHub Discussions (Optional)

1. Go: https://github.com/evoludigit/elo-rust/settings
2. Check: "Discussions"
3. Customize categories for:
   - Announcements
   - General discussion
   - Q&A
   - Security advisories

### Auto-close Issues

Create `.github/workflows/auto-close.yml`:
```yaml
name: Auto-close

on:
  issues:
    types: [opened]

jobs:
  auto-respond:
    if: contains(github.event.issue.body, 'BUG_TEMPLATE')
    runs-on: ubuntu-latest
    steps:
      - uses: actions/github-script@v6
        with:
          script: |
            github.rest.issues.createComment({
              issue_number: context.issue.number,
              owner: context.repo.owner,
              repo: context.repo.repo,
              body: 'Please follow the bug report template.'
            })
```

## 5. Notifications

### Watching Repository

1. Go: https://github.com/evoludigit/elo-rust
2. Click: "Watch"
3. Select: "Custom"
4. Check:
   - ✅ Issues
   - ✅ Pull requests
   - ✅ Releases
   - ✅ Security alerts
   - ✅ Discussions (if enabled)

### Email Notifications

1. Go: https://github.com/settings/notifications
2. Set:
   - Default notifications: Email
   - Security alerts: Email
   - Workflow run failures: Email

## 6. Verification

### Verify Secrets
```bash
gh secret list --repo evoludigit/elo-rust
```

Expected output:
```
CARGO_REGISTRY_TOKEN  Updated Feb  8, 2026
```

### Verify Workflows
```bash
gh workflow list --repo evoludigit/elo-rust
```

Expected workflows:
```
CI             active
Security       active
Release        active
Publish        active
```

### Run Workflow (Optional Test)
```bash
# Dry-run CI workflow
gh workflow run ci.yml --repo evoludigit/elo-rust --ref main
```

### Check Status
```bash
gh run list --repo evoludigit/elo-rust
```

## 7. First Release Walkthrough

When ready to make first release (v0.1.1):

1. **Ensure CI passes**
   ```bash
   gh run list --repo evoludigit/elo-rust --limit 1
   ```

2. **Create tag**
   ```bash
   git tag v0.1.1
   git push origin v0.1.1
   ```

3. **Publish workflow runs automatically**
   - Verify on: https://github.com/evoludigit/elo-rust/actions
   - Check: "Publish" workflow status

4. **Verify on crates.io**
   - Visit: https://crates.io/crates/elo-rust
   - Should show: v0.1.1

5. **Verify on docs.rs**
   - Visit: https://docs.rs/elo-rust/0.1.1/
   - Should rebuild automatically

## 8. Troubleshooting

### "Token invalid" error in publish

1. Check token is set:
   ```bash
   gh secret list --repo evoludigit/elo-rust
   ```

2. Regenerate token:
   - https://crates.io/me
   - Create new token
   - Update secret

3. Retry publish:
   ```bash
   # Manual retry (requires tag)
   gh workflow run publish.yml --repo evoludigit/elo-rust
   ```

### Workflow didn't trigger

1. Check branch name: should be `main` or `develop`
2. Check tag format: should be `v*.*.*.`
3. Check GitHub Actions enabled:
   - https://github.com/evoludigit/elo-rust/settings/actions

### Secret not available in workflow

1. Verify secret exists:
   ```bash
   gh secret list --repo evoludigit/elo-rust
   ```

2. Check workflow syntax (must be exact case):
   ```yaml
   ${{ secrets.CARGO_REGISTRY_TOKEN }}  # ✅ Correct
   ${{ secrets.cargo_registry_token }}  # ❌ Wrong (case-sensitive)
   ```

## Done! ✅

- [ ] CARGO_REGISTRY_TOKEN added
- [ ] Branch protection enabled
- [ ] Workflows enabled
- [ ] Notifications configured
- [ ] Secrets verified

You're ready to publish! 🚀

---

**Reference**: See `SECRETS_SETUP.md` for detailed instructions
