#!/bin/bash

# Usage: bash changelog.sh "2025-01-01" "2025-04-01"
START_DATE=$1
END_DATE=$2

echo ""
echo "# Features"
git log --since="$START_DATE" --until="$END_DATE" --pretty=format:"%s" \
  | grep -E "^feat:" \
  | sort
echo ""

echo "# Fixes"
git log --since="$START_DATE" --until="$END_DATE" --pretty=format:"%s" \
  | grep -E "^fix:" \
  | sort
echo ""

echo "# Refactor"
git log --since="$START_DATE" --until="$END_DATE" --pretty=format:"%s" \
  | grep -E "^refactor:" \
  | sort
echo ""

echo "# Chores"
git log --since="$START_DATE" --until="$END_DATE" --pretty=format:"%s" \
  | grep -E "^chore:" \
  | sort
