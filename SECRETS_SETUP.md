# GitHub Secrets Setup Guide

This guide documents all required and optional GitHub repository secrets for the elo-rust project.

## Overview

GitHub Secrets are encrypted environment variables used in CI/CD workflows. They are:
- Encrypted at rest in GitHub
- Never shown in logs
- Available only to authorized workflows
- Revokable at any time

## Required Secrets

### 1. CARGO_REGISTRY_TOKEN (Required for Publishing)

**Purpose**: Publishing crate to crates.io

**How to obtain**:
1. Go to https://crates.io/me
2. Click "API Tokens"
3. Create a new token with:
   - Name: `GitHub Actions - elo-rust`
   - Permissions: `Publish new versions`
4. Copy the token (only shown once!)

**How to set**:

#### Option A: Using GitHub Web UI (Recommended for first-time)
1. Go to: `https://github.com/evoludigit/elo-rust/settings/secrets/actions`
2. Click "New repository secret"
3. Name: `CARGO_REGISTRY_TOKEN`
4. Value: Paste the token from crates.io
5. Click "Add secret"

#### Option B: Using GitHub CLI
```bash
gh secret set CARGO_REGISTRY_TOKEN --body "your-crates-io-token" \
  --repo evoludigit/elo-rust
```

**Verification**:
- Check: `https://github.com/evoludigit/elo-rust/settings/secrets/actions`
- Should see `CARGO_REGISTRY_TOKEN` in the list
- Run: `cargo publish --token ${{ secrets.CARGO_REGISTRY_TOKEN }}`

**Rotation**: crates.io tokens should be rotated every 90 days (set a calendar reminder)

### 2. GITHUB_TOKEN (Automatically Provided)

**Purpose**: GitHub Actions CI/CD operations

**Status**: ✅ **Automatically created by GitHub**
- No manual setup needed
- Available in all workflows via `${{ secrets.GITHUB_TOKEN }}`
- Scoped to current repository only
- Expires at end of workflow

**Permissions**:
- Create releases
- Upload artifacts
- Update commit status
- Comment on PRs

**Note**: Already configured in `.github/workflows/publish.yml`

## Optional Secrets (For Enhanced Features)

### 3. GIT_AUTHOR_NAME (Optional - for Semantic Release)

**Purpose**: Author name for automated commits

**Default**: `github-actions[bot]`

**How to set**:
```bash
gh secret set GIT_AUTHOR_NAME --body "Your Name" \
  --repo evoludigit/elo-rust
```

**Example**: `Claude Assistant`

### 4. GIT_AUTHOR_EMAIL (Optional - for Semantic Release)

**Purpose**: Author email for automated commits

**Default**: `github-actions[bot]@users.noreply.github.com`

**How to set**:
```bash
gh secret set GIT_AUTHOR_EMAIL --body "email@example.com" \
  --repo evoludigit/elo-rust
```

**Example**: `claude@anthropic.com`

### 5. SLACK_WEBHOOK (Optional - for Notifications)

**Purpose**: Send security alerts to Slack channel

**How to obtain**:
1. Go to your Slack workspace
2. Create incoming webhook: https://api.slack.com/apps
3. Create new app → From scratch
4. Name: `elo-rust-security`
5. Select workspace
6. Go to "Incoming Webhooks"
7. Create New Webhook
8. Select channel (e.g., `#security`)
9. Copy the Webhook URL

**How to set**:
```bash
gh secret set SLACK_WEBHOOK --body "https://hooks.slack.com/..." \
  --repo evoludigit/elo-rust
```

**Usage**: Can be added to workflows for alerts on failures

## Setup Checklist

### Critical (Required for Publishing)
- [ ] **CARGO_REGISTRY_TOKEN** - Set up at crates.io, added to GitHub

### Automatic (No Action Needed)
- [x] **GITHUB_TOKEN** - Automatically available

### Optional (Recommended)
- [ ] GIT_AUTHOR_NAME - For branded commits
- [ ] GIT_AUTHOR_EMAIL - For branded commits
- [ ] SLACK_WEBHOOK - For security notifications

## Verification Steps

### Verify Secrets Are Set

```bash
# List all secrets (names only, not values)
gh secret list --repo evoludigit/elo-rust
```

Expected output:
```
CARGO_REGISTRY_TOKEN  Updated Dec 8, 2024
GITHUB_TOKEN          (automatic)
```

### Test Secret Access in Workflow

Add this to any workflow job:

```yaml
- name: Verify secrets are available
  run: |
    if [ -z "${{ secrets.CARGO_REGISTRY_TOKEN }}" ]; then
      echo "❌ CARGO_REGISTRY_TOKEN not set"
      exit 1
    else
      echo "✅ CARGO_REGISTRY_TOKEN is available"
    fi
```

## Security Best Practices

### For Repository Maintainers

1. **Minimal Access**
   - Only add secrets that are actively used
   - Remove unused secrets quarterly
   - Rotate tokens every 90 days

2. **Monitoring**
   - Check GitHub audit log: `https://github.com/evoludigit/elo-rust/settings/audit-log`
   - Look for secret access patterns
   - Alert on unusual activity

3. **Rotation Schedule**
   - Every 90 days: Rotate CARGO_REGISTRY_TOKEN
   - Quarterly: Review all secrets
   - Immediately: If leaked or compromised

4. **Secret Scope**
   - CARGO_REGISTRY_TOKEN: Only available to `publish.yml` workflow
   - GITHUB_TOKEN: Available to all workflows (auto-scoped)

### For Developers

1. **Never Log Secrets**
   - Don't echo secrets in logs
   - Use `--no-verbose` flags where possible
   - Be careful with `set-output` commands

2. **Never Hardcode Secrets**
   - Always use `${{ secrets.SECRET_NAME }}`
   - Never commit tokens or keys
   - Use `.gitignore` for local secrets

3. **Report Breaches**
   - If a secret is exposed:
     1. Immediately revoke it
     2. Generate a new one
     3. Update it in GitHub
     4. Check audit logs for unauthorized access

## Troubleshooting

### Error: "Could not resolve to a Repository"

**Cause**: Wrong repository path or no authentication

**Fix**:
```bash
# Set default repo
gh repo set-default evoludigit/elo-rust

# Or use full path
gh secret set CARGO_REGISTRY_TOKEN \
  --repo evoludigit/elo-rust \
  --body "your-token"
```

### Error: "Failed to authenticate"

**Cause**: GitHub CLI not authenticated

**Fix**:
```bash
# Authenticate
gh auth login

# Choose: GitHub.com
# Choose: HTTPS
# Authenticate: Yes
# Default: Yes
```

### Secret Not Available in Workflow

**Possible causes**:
1. Secret name misspelled
2. Secret uses underscore, workflow uses dash (or vice versa)
3. Workflow doesn't have permissions
4. Secret only available to main branch

**Fix**:
```yaml
# ✅ Correct
- run: cargo publish --token ${{ secrets.CARGO_REGISTRY_TOKEN }}

# ❌ Wrong (CARGO-REGISTRY-TOKEN with dashes)
- run: cargo publish --token ${{ secrets.CARGO-REGISTRY-TOKEN }}

# ❌ Wrong (CRATES_IO_TOKEN instead)
- run: cargo publish --token ${{ secrets.CRATES_IO_TOKEN }}
```

## Rotating CARGO_REGISTRY_TOKEN

Every 90 days, rotate your crates.io token:

1. **Generate new token on crates.io**:
   - https://crates.io/me
   - Click "API Tokens"
   - Create new token
   - Name: `GitHub Actions - elo-rust (rotated YYYY-MM-DD)`

2. **Update GitHub secret**:
   ```bash
   gh secret set CARGO_REGISTRY_TOKEN --body "new-token-here" \
     --repo evoludigit/elo-rust
   ```

3. **Revoke old token on crates.io**:
   - https://crates.io/me
   - Click the "Revoke" button next to old token

4. **Set calendar reminder** for next rotation (90 days)

## Reference

- [GitHub Secrets Documentation](https://docs.github.com/en/actions/security-guides/using-secrets-in-github-actions)
- [Crates.io API Tokens](https://doc.rust-lang.org/cargo/registries/crates-io-token.html)
- [GitHub CLI Secret Management](https://cli.github.com/manual/gh_secret)

---

**Last Updated**: February 8, 2026
**Status**: Setup Guide Complete
**Next**: Run through checklist above
