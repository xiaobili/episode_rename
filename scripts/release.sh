#!/bin/bash
set -e

# release.sh - Automate tag creation and pushing for openlist-tui
#
# Usage: ./scripts/release.sh
#
# This script:
# 1. Extracts the version from Cargo.toml
# 2. Validates the version was found
# 3. Checks if the tag already exists
# 4. Creates a git tag in format v{version}
# 5. Pushes the tag to origin to trigger the release workflow

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Get the project root directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
CARGO_TOML="${PROJECT_ROOT}/Cargo.toml"

echo "=== openlist-tui Release Script ==="
echo ""

# Check if Cargo.toml exists
if [ ! -f "${CARGO_TOML}" ]; then
    echo -e "${RED}Error: Cargo.toml not found at ${CARGO_TOML}${NC}"
    exit 1
fi

# Extract version from Cargo.toml
echo "Extracting version from Cargo.toml..."
VERSION=$(grep -E '^version\s*=\s*"[^"]+"' "${CARGO_TOML}" | head -1 | cut -d'"' -f2)

if [ -z "${VERSION}" ]; then
    echo -e "${RED}Error: Could not extract version from Cargo.toml${NC}"
    exit 1
fi

echo -e "Found version: ${GREEN}${VERSION}${NC}"

# Check if we're in a git repository
if [ ! -d "${PROJECT_ROOT}/.git" ]; then
    echo -e "${RED}Error: Not a git repository${NC}"
    exit 1
fi

cd "${PROJECT_ROOT}"

# Check if tag already exists
TAG="v${VERSION}"
echo "Checking if tag ${TAG} exists..."

if git rev-parse "${TAG}" >/dev/null 2>&1; then
    echo -e "${YELLOW}Warning: Tag ${TAG} already exists${NC}"
    read -p "Do you want to force delete and recreate it? (y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo "Deleting existing tag ${TAG}..."
        git tag -d "${TAG}"
        echo -e "${GREEN}Deleted local tag ${TAG}${NC}"
    else
        echo "Aborting release."
        exit 0
    fi
fi

# Create the tag
echo "Creating git tag ${TAG}..."
git tag "${TAG}"
echo -e "${GREEN}Created tag ${TAG}${NC}"

# Push the tag to origin
echo "Pushing tag ${TAG} to origin..."
git push origin "${TAG}"
echo -e "${GREEN}Pushed tag ${TAG} to origin${NC}"

echo ""
echo -e "${GREEN}=== Release tag ${TAG} created and pushed successfully! ===${NC}"
echo ""
echo "The GitHub Actions release workflow should now be triggered."
echo "Check the Actions tab: https://github.com/$(git remote get-url origin | sed 's/.*github.com[:/]//;s/\.git$//')/actions"
