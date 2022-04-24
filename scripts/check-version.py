#!/usr/bin/env python3

import re
import sys

expected = re.search(r'name = "ea"\nversion = "(.+)"', open('Cargo.toml').read(), re.M).group(1)
versions = {}
versions['Cargo.lock'] = re.search(r'name = "ea"\nversion = "(.+)"', open('Cargo.lock').read(), re.M).group(1)
versions['CHANGELOG.md'] = re.search(r'# main\s+#\s*(.+)', open('CHANGELOG.md').read(), re.M).group(1)

for file in versions:
    if expected != versions[file]:
        print(f"version mismatch: expected {expected}; found {versions[file]} in {file}", file=sys.stderr)
        exit(1)
