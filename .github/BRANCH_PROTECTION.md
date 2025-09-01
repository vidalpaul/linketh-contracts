# Branch Protection Configuration

This document outlines the required branch protection settings for the Linketh repository to ensure code quality and security.

## Required Branch Protection Rules

### For `main` branch:

**Status Checks (Required)**:
- ✅ `CI Success` (from ci.yml workflow)
- ✅ `Stylus Compatibility Check` (from ci.yml workflow) 
- ✅ `Build and Test` (from ci.yml workflow)
- ✅ `Code Validation` (from ci.yml workflow)
- ✅ `Docker Build Test` (from ci.yml workflow)

**Protection Settings**:
- ✅ Require status checks to pass before merging
- ✅ Require branches to be up to date before merging
- ✅ Require pull request reviews before merging (minimum 1 reviewer)
- ✅ Dismiss stale pull request approvals when new commits are pushed
- ✅ Require review from code owners (if CODEOWNERS file exists)
- ✅ Restrict pushes that create files larger than 100MB
- ✅ Require signed commits (recommended)
- ✅ Include administrators in these restrictions
- ✅ Allow force pushes: **DISABLED**
- ✅ Allow deletions: **DISABLED**

### For `develop` branch:

**Status Checks (Required)**:
- ✅ `CI Success` (from ci.yml workflow)
- ✅ `Stylus Compatibility Check` (from ci.yml workflow)
- ✅ `Build and Test` (from ci.yml workflow) 
- ✅ `Code Validation` (from ci.yml workflow)

**Protection Settings**:
- ✅ Require status checks to pass before merging
- ✅ Require branches to be up to date before merging
- ✅ Require pull request reviews before merging (minimum 1 reviewer)
- ✅ Restrict pushes that create files larger than 100MB
- ✅ Include administrators in these restrictions
- ✅ Allow force pushes: **DISABLED**
- ✅ Allow deletions: **DISABLED**

## Critical Status Checks

### 🛡️ Stylus Compatibility Check
**Why Critical**: This check ensures the contract compiles correctly for Arbitrum Stylus and passes all WASM validation requirements. **Failures here mean the contract cannot be deployed.**

**What it checks**:
- WASM compilation for `wasm32-unknown-unknown` target
- Stylus runtime compatibility via `cargo stylus check`
- WASM binary size limits (must be under 128KB)
- Contract activation requirements

### 🧪 Build and Test
**Why Critical**: Ensures all unit tests pass and the code compiles without errors.

**What it checks**:
- Rust compilation for both debug and release builds
- All 20 unit tests must pass
- WASM target compilation
- Test coverage reporting

### ✨ Code Validation  
**Why Critical**: Maintains code quality and security standards.

**What it checks**:
- Code formatting via `cargo fmt`
- Linting via `cargo clippy` (zero warnings policy)
- Security vulnerability scanning via `cargo audit`

## Setting Up Branch Protection

### Via GitHub Web Interface:

1. Navigate to **Settings** → **Branches**
2. Click **Add rule** or edit existing rule
3. Set **Branch name pattern**: `main` (or `develop`)
4. Configure settings as specified above
5. In **Status checks**, add the required checks:
   - Search for and select each required status check
   - Enable "Require branches to be up to date"

### Via GitHub CLI:

```bash
# For main branch
gh api repos/:owner/:repo/branches/main/protection \
  --method PUT \
  --field required_status_checks='{"strict":true,"checks":[{"context":"CI Success"},{"context":"Stylus Compatibility Check"},{"context":"Build and Test"},{"context":"Code Validation"},{"context":"Docker Build Test"}]}' \
  --field enforce_admins=true \
  --field required_pull_request_reviews='{"required_approving_review_count":1,"dismiss_stale_reviews":true}' \
  --field restrictions=null

# For develop branch  
gh api repos/:owner/:repo/branches/develop/protection \
  --method PUT \
  --field required_status_checks='{"strict":true,"checks":[{"context":"CI Success"},{"context":"Stylus Compatibility Check"},{"context":"Build and Test"},{"context":"Code Validation"}]}' \
  --field enforce_admins=true \
  --field required_pull_request_reviews='{"required_approving_review_count":1}' \
  --field restrictions=null
```

## Workflow Dependencies

The `CI Success` job depends on all critical jobs and will only pass if:

1. ✅ **Code Validation** completes successfully
2. ✅ **Build and Test** completes successfully  
3. ✅ **Stylus Compatibility Check** completes successfully
4. ✅ **Docker Build Test** completes successfully
5. ✅ **Documentation Check** completes successfully

## Emergency Procedures

### Bypassing Protection (Admins Only)

In rare emergency situations, administrators can temporarily disable branch protection:

```bash
# Disable protection
gh api repos/:owner/:repo/branches/main/protection --method DELETE

# Re-enable after emergency fix
# (Use the setup commands above)
```

**⚠️ Important**: Always re-enable protection immediately after emergency fixes.

### Failing Status Checks

If legitimate code changes cause status check failures:

1. **For Stylus Check Failures**: 
   - Review WASM compilation errors
   - Check contract size limits
   - Verify Stylus SDK compatibility

2. **For Test Failures**:
   - Fix failing unit tests
   - Ensure new code has test coverage
   - Verify no breaking changes to existing APIs

3. **For Validation Failures**:
   - Run `cargo fmt` to fix formatting
   - Address `clippy` warnings
   - Update dependencies to fix security issues

## Monitoring

### Check Status via GitHub API:
```bash
# Check protection status
gh api repos/:owner/:repo/branches/main/protection

# Check recent status checks
gh api repos/:owner/:repo/commits/main/status
```

### Weekly Review Checklist:
- [ ] Verify all required status checks are still configured
- [ ] Review security audit results
- [ ] Check for outdated dependencies
- [ ] Validate Docker build performance
- [ ] Review pull request merge patterns

## Contact

For questions about branch protection or to request changes:
- Open an issue with label `ci/cd`
- Contact repository administrators
- Review GitHub Actions logs for specific failure details