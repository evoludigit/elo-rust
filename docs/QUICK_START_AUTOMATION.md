# Quick Start: Automated Publishing

Get started with automated releases in 3 simple steps.

## Step 1: Add Crates.io Token to GitHub (2 minutes)

1. Go to https://crates.io/me
2. Create or copy API token
3. Go to your repo: Settings → Secrets and variables → Actions
4. Click "New repository secret"
5. Name: `CARGO_REGISTRY_TOKEN`
6. Value: Your crates.io token
7. Save

**Done!** Publishing is now enabled.

## Step 2: Make Conventional Commits

Use these commit message formats:

```bash
# New feature
git commit -m "feat(component): what you added"

# Bug fix
git commit -m "fix(component): what you fixed"

# Documentation
git commit -m "docs: what you documented"
```

## Step 3: Push and Release

```bash
git push origin main
```

Automation will:
1. Run all tests
2. Analyze commits
3. Bump version automatically
4. Publish to crates.io
5. Create GitHub Release

## Version Bumping Rules

| Commit Type | Version Change | Example |
|------------|---|---|
| `feat(...)` | Minor bump | 0.1.0 → 0.2.0 |
| `fix(...)` | Patch bump | 0.1.0 → 0.1.1 |
| `BREAKING CHANGE:` | Major bump | 0.1.0 → 1.0.0 |

## That's It!

Your automation is ready. Just write code with conventional commits and watch it publish automatically. 🚀

---

For detailed info, see AUTOMATION_COMPLETE.md or RELEASE_GUIDE.md
