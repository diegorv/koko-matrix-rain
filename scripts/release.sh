#!/usr/bin/env bash
set -euo pipefail

# ─── Helpers ──────────────────────────────────────────────────────────
usage() {
  echo "Usage: $0 [patch|minor|major]"
  echo ""
  echo "Bump types:"
  echo "  patch  (default)  Bug fixes and small tweaks"
  echo "                    0.1.0 → 0.1.1 → 0.1.2 ..."
  echo ""
  echo "  minor             New features (resets patch to 0)"
  echo "                    0.1.2 → 0.2.0 → 0.3.0 ..."
  echo ""
  echo "  major             Breaking changes (resets minor and patch to 0)"
  echo "                    0.3.0 → 1.0.0 → 2.0.0 ..."
  echo ""
  echo "Creates an annotated tag with a changelog, bumps Cargo.toml,"
  echo "and pushes to origin. The GitHub Action then publishes to crates.io."
  echo ""
  echo "Examples:"
  echo "  $0              # 0.1.0 → 0.1.1"
  echo "  $0 minor        # 0.1.1 → 0.2.0"
  echo "  $0 major        # 0.2.0 → 1.0.0"
  exit 1
}

# ─── Parse bump type ─────────────────────────────────────────────────
BUMP="${1:-patch}"
case "$BUMP" in
  patch|minor|major) ;;
  -h|--help) usage ;;
  *) echo "Error: unknown bump type '$BUMP'"; usage ;;
esac

# ─── Get latest tag ──────────────────────────────────────────────────
LATEST_TAG=$(git tag --sort=-v:refname | head -1)

if [ -z "$LATEST_TAG" ]; then
  echo "No tags found. Starting from v0.1.0"
  LATEST_TAG="v0.0.0"
fi

echo "Latest tag: $LATEST_TAG"

# ─── Strip prefix and split version ──────────────────────────────────
VERSION=$(echo "$LATEST_TAG" | grep -oE '[0-9]+\.[0-9]+\.[0-9]+')
IFS='.' read -r MAJOR MINOR PATCH <<< "$VERSION"

# ─── Bump version ────────────────────────────────────────────────────
case "$BUMP" in
  patch) PATCH=$((PATCH + 1)) ;;
  minor) MINOR=$((MINOR + 1)); PATCH=0 ;;
  major) MAJOR=$((MAJOR + 1)); MINOR=0; PATCH=0 ;;
esac

NEW_VERSION="${MAJOR}.${MINOR}.${PATCH}"
NEW_TAG="v${NEW_VERSION}"

# ─── Build changelog from commits since last tag ─────────────────────
echo ""
echo "──────────────────────────────────────"
echo "  $LATEST_TAG → $NEW_TAG ($BUMP)"
echo "──────────────────────────────────────"
echo ""

if [ "$LATEST_TAG" = "v0.0.0" ]; then
  COMMITS=$(git log --oneline --no-decorate)
else
  COMMITS=$(git log "${LATEST_TAG}..HEAD" --oneline --no-decorate)
fi

if [ -z "$COMMITS" ]; then
  echo "No new commits since $LATEST_TAG. Aborting."
  exit 1
fi

# ─── Format changelog ────────────────────────────────────────────────
CHANGELOG=$(echo "$COMMITS" | while IFS= read -r line; do
  MSG="${line#* }"
  echo "- $MSG"
done)

TAG_BODY="Release ${NEW_TAG}

Changes since ${LATEST_TAG}:

${CHANGELOG}
"

echo "$TAG_BODY"
echo "──────────────────────────────────────"
echo ""

# ─── Confirm ─────────────────────────────────────────────────────────
read -rp "Create tag $NEW_TAG? [y/N] " CONFIRM
if [[ ! "$CONFIRM" =~ ^[Yy]$ ]]; then
  echo "Aborted."
  exit 0
fi

# ─── Bump version in Cargo.toml ─────────────────────────────────────
echo "Updating version in Cargo.toml..."

sed -i '' "s/^version = \"[^\"]*\"/version = \"${NEW_VERSION}\"/" Cargo.toml
cargo generate-lockfile --quiet

git add Cargo.toml Cargo.lock
git commit -m "bump version to ${NEW_VERSION}"

echo "Version bumped and committed."
echo ""

# ─── Create annotated tag ────────────────────────────────────────────
git tag -a "$NEW_TAG" -m "$TAG_BODY"

echo ""
echo "Tag $NEW_TAG created. Pushing to origin..."
echo ""

git push origin main
git push origin "$NEW_TAG"

echo ""
echo "Done! Tag $NEW_TAG pushed. GitHub Action will publish to crates.io."
