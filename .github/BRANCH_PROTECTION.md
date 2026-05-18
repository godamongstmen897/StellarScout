# Branch Protection Rules Configuration
# Apply these settings to the main branch via GitHub UI

# Main Branch Protection Settings:
# 
# 1. Require a pull request before merging
#    - Dismiss stale pull request approvals when new commits are pushed
#    - Require approval of the most recent reviewers
#
# 2. Require status checks to pass before merging
#    - Require branches to be up to date before merging
#    - Status checks required:
#      - Tests (stable)
#      - Tests (nightly)
#      - Rustfmt
#      - Clippy
#      - Code Coverage
#      - Security Audit
#      - Dependency Check
#      - Secret Scanning
#      - Code Review
#
# 3. Enforce all the above rules for administrators
#
# 4. Restrict who can push to matching branches
#    - Include administrators in restrictions (optional)

# To apply via GitHub API:
# BRANCH_PROTECTION_RULES=$(cat << 'EOF'
# {
#   "required_status_checks": {
#     "strict": true,
#     "contexts": [
#       "Tests (stable)",
#       "Tests (nightly)",
#       "Rustfmt",
#       "Clippy",
#       "Code Coverage",
#       "Security Audit",
#       "Dependency Check",
#       "Secret Scanning",
#       "Code Review"
#     ]
#   },
#   "required_pull_request_reviews": {
#     "dismiss_stale_reviews": true,
#     "require_code_owner_reviews": false,
#     "required_approving_review_count": 1
#   },
#   "enforce_admins": true,
#   "restrictions": null
# }
# EOF
# )
