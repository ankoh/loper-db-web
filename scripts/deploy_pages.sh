#!/usr/bin/env bash

set -ex

PROJECT_ROOT="$(cd $(dirname "$BASH_SOURCE[0]") && cd .. && pwd)" &> /dev/null
PAGES_DIR=${PROJECT_ROOT}/worktrees/gh-pages

mkdir -p ${PROJECT_ROOT}/worktrees
if [ ! -d ${PAGES_DIR} ]; then
    echo "[ RUN ] Add worktree origin/gh-pages"
    git worktree add ${PAGES_DIR} origin/gh-pages
fi

DEFAULT_BRANCH="main"
CURRENT_BRANCH=${1:-main}

cd ${PAGES_DIR}
git fetch origin gh-pages
git reset --hard origin/gh-pages

if [ "${CURRENT_BRANCH}" = "${DEFAULT_BRANCH}" ]; then
    echo "[ RUN ] Install @ankoh/loper-db-web-app to ${PAGES_DIR}/"
    find ${PAGES_DIR} \
        -mindepth 1 \
        -maxdepth 1 \
        -type d \
        -not -name CNAME \
        -exec rm -rf '{}' \;
    cp -r ${PROJECT_ROOT}/packages/loper-db-web-app/build/pwa/prod/* ${PAGES_DIR}
else
    TARGET_DIR="${PAGES_DIR}/branches/${CURRENT_BRANCH}"
    echo "[ RUN ] Install @ankoh/loper-db-web-app to ${TARGET_DIR}/"
    rm -rf ${TARGET_DIR}
    mkdir -p ${PAGES_DIR}/branches
    cp -r ${PROJECT_ROOT}/packages/loper-web-app/build/pwa/prod ${TARGET_DIR}
fi

git config user.name 'github-actions[bot]'
git config user.email '41898282+github-actions[bot]@users.noreply.github.com'
git add -A .
git commit --amend --reset-author -m "Deploy GitHub Pages"
git push origin HEAD:gh-pages --force
