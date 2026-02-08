# Security Workflows Configuration

This document describes all GitHub Actions workflows and their security configuration.

## Workflow Overview

### 1. CI Workflow (`.github/workflows/ci.yml`)

**Trigger**: Every push to main/develop, every PR

**Jobs**:
- ✅ Test Suite (Linux, macOS, Windows × stable, beta)
- ✅ Clippy (code quality)
- ✅ Rustfmt (formatting)
- ✅ Documentation (doc building)
- ✅ Build (debug + release)
- ✅ Code Coverage (tarpaulin)
- ✅ Security Audit (cargo audit)
- ✅ SBOM Generation (dependencies)

**Permissions**:
- Read access to repository
- Write access to artifacts (coverage, SBOM)
- No external service access

**Secrets Used**: None

**Status Badge**:
```markdown
[![CI](https://github.com/evoludigit/elo-rust/workflows/CI/badge.svg)](https://github.com/evoludigit/elo-rust/actions/workflows/ci.yml)
```

### 2. Security Workflow (`.github/workflows/security.yml`)

**Trigger**:
- Push to main/develop
- Every day at 00:00 UTC
- PRs to main/develop

**Jobs**:
- ✅ Cargo Audit - CVE detection
- ✅ Cargo Deny - supply chain security
- ✅ Trivy - filesystem scanning
- ✅ TruffleHog - secret detection
- ✅ Supply Chain - no git dependencies
- ✅ Secret Scanning - credential leaks

**Permissions**:
- Read access to repository
- Write access to security events (SARIF upload)

**Secrets Used**: None

**GitHub Integration**:
- Trivy results → GitHub Security tab (SARIF)
- Secret scanning → GitHub Security tab
- All results visible in: https://github.com/evoludigit/elo-rust/security/code-scanning

### 3. Release Workflow (`.github/workflows/release.yml`)

**Trigger**: Workflow completion of CI on main/develop

**Jobs**:
- ✅ Semantic Release - auto-version & changelog

**Permissions**:
- Read: Repository
- Write: Contents (tags, releases), Issues, PRs, Discussions

**Secrets Used**:
- `GITHUB_TOKEN` (automatic)

**Environment Variables**:
- `GITHUB_TOKEN` → Used by semantic-release

**Output**:
- Creates tag (v0.1.1, v0.2.0, etc.)
- Updates CHANGELOG.md
- Creates GitHub Release

### 4. Publish Workflow (`.github/workflows/publish.yml`)

**Trigger**: Tag push matching `v[0-9]+.[0-9]+.[0-9]+*`

**Jobs**:
- ✅ Verify (run all tests before publishing)
- ✅ Publish to crates.io
- ✅ Create GitHub Release
- ✅ Update documentation

**Permissions**:
- Read: Repository, Contents
- Write: Contents (releases), Packages

**Secrets Used**:
- `CARGO_REGISTRY_TOKEN` - **REQUIRED** ⚠️
  - Used: `cargo publish --token ${{ secrets.CARGO_REGISTRY_TOKEN }}`
  - Scope: Only publish workflow
  - Rotation: Every 90 days

**Output**:
- Published to crates.io
- GitHub Release created with artifacts
- Docs updated on docs.rs (automatic)

## Permission Levels

### Automatic (GitHub Always Provides)
- `GITHUB_TOKEN` - Scoped to current repo, expires after workflow
- Pull request context (PR numbers, branch info, etc.)

### Manual Setup Required
- `CARGO_REGISTRY_TOKEN` - Must be added by repository maintainer

## Security Event Outputs

### GitHub Security Tab Integration

All security findings appear in:
**Settings → Security → Code scanning**

Or direct link:
```
https://github.com/evoludigit/elo-rust/security/code-scanning
```

**Scanners integrated**:
- ✅ Trivy vulnerability scanner (SARIF format)
- ✅ Secret scanning (native GitHub)
- ✅ Dependabot alerts (native GitHub)
- ✅ Code scanning (when enabled)

## Artifact Retention

### Build Artifacts

Artifacts are retained for **90 days** (GitHub default):

- **SBOM** (dependencies.txt, all-dependencies.txt)
  - Location: CI workflow artifacts
  - Use: Audit trails, license compliance
  - Retention: 90 days

- **Code Coverage** (cobertura.xml)
  - Location: codecov.io
  - Use: Track coverage trends
  - Retention: Codecov default

## Monitoring & Alerts

### Check Workflow Status

```bash
# List all workflows
gh run list --repo evoludigit/elo-rust

# Watch CI status
gh run watch --repo evoludigit/elo-rust

# Get specific run details
gh run view <RUN_ID> --repo evoludigit/elo-rust
```

### Set Up Notifications

**In GitHub**:
1. Go to: https://github.com/settings/notifications
2. Check: "Automatically watch all repositories"
3. Check: "Include your own updates"
4. Set email for: "Workflow runs notifications"

**Via Email**:
- Watch failures: Settings → Notifications → Email
- Filter: Enabled

**Via Slack** (optional):
- Add `SLACK_WEBHOOK` secret
- Update security.yml to send alerts

## Troubleshooting

### Workflow Didn't Run

**Causes**:
1. Path filter didn't match changes
2. Branch is not main/develop
3. Tag format wrong (should be v*.*.*)
4. Workflow syntax error

**Fix**:
```bash
# Check syntax
gh workflow view ci --repo evoludigit/elo-rust

# Run workflow manually
gh workflow run ci.yml --repo evoludigit/elo-rust
```

### Secret Not Available

**Check**:
1. Secret exists: `gh secret list --repo evoludigit/elo-rust`
2. Secret name matches exactly (case-sensitive)
3. Workflow has permission to access secret
4. Using correct syntax: `${{ secrets.SECRET_NAME }}`

### Publish Failed - "Token Invalid"

**Causes**:
1. CARGO_REGISTRY_TOKEN not set
2. Token expired or revoked
3. Token doesn't have publish permission

**Fix**:
1. Generate new token on crates.io
2. Update secret: `gh secret set CARGO_REGISTRY_TOKEN --body "new-token"`
3. Re-run publish workflow

## Best Practices

### For Maintainers

1. **Review Secrets Monthly**
   ```bash
   gh secret list --repo evoludigit/elo-rust
   ```

2. **Rotate Tokens Every 90 Days**
   - Set calendar reminders
   - Revoke old tokens
   - Update GitHub secret

3. **Monitor Audit Log**
   ```
   https://github.com/evoludigit/elo-rust/settings/audit-log
   ```

4. **Review Security Alerts**
   ```
   https://github.com/evoludigit/elo-rust/security/
   ```

### For Contributors

1. **Don't commit secrets**
   - Use `.gitignore` for local tokens
   - Never echo secrets in code

2. **Use GitHub Secrets**
   - Always use `${{ secrets.NAME }}`
   - For local testing, use environment variables

3. **Report security issues**
   - Use confidential issue form
   - Don't post in public discussions

## Workflow Diagram

```
Push to main/develop
    ↓
[CI Workflow] (tests, lint, coverage)
    ↓
Success?
    ├─ Yes → [Security Workflow] (audit, scan, SBOM)
    └─ No → Fail & notify
    ↓
[Semantic Release Workflow] (tag & changelog)
    ↓
Tag created (v*.*.*)
    ↓
[Publish Workflow] (verify → publish → release)
    ↓
Package on crates.io ✅
Release on GitHub ✅
Docs on docs.rs ✅
```

## CI Status Badges

Add to README:

```markdown
[![CI][ci-badge]][ci-link]
[![Security][security-badge]][security-link]

[ci-badge]: https://github.com/evoludigit/elo-rust/workflows/CI/badge.svg
[ci-link]: https://github.com/evoludigit/elo-rust/actions/workflows/ci.yml
[security-badge]: https://github.com/evoludigit/elo-rust/workflows/Security/badge.svg
[security-link]: https://github.com/evoludigit/elo-rust/actions/workflows/security.yml
```

---

**Last Updated**: February 8, 2026
**Status**: All workflows configured and ready
