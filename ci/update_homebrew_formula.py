#!/usr/bin/env python3

import os
import re
import sys

hash = os.environ.get("HASH")
target = os.environ.get("TARGET")
formula_path = os.environ.get("FORMULA_PATH")
tag = os.environ.get("TAG")

if hash is None:
    print("HASH is not set")
    print("Exiting")
    sys.exit(1)

if target is None:
    print("TARGET is not set")
    print("Exiting")
    sys.exit(1)

if formula_path is None:
    print("FORMULA_PATH is not set")
    print("Exiting")
    sys.exit(1)

if tag is None:
    print("TAG is not set")
    print("Exiting")
    sys.exit(1)

# update the hash
with open(formula_path, 'r+') as f:
    contents = f.read()

    r = re.compile('sha256 \"(.*)\"')
    match_list = list(re.finditer(r, contents))
    
    if len(match_list) != 2:
        print("Number of matches is not 2")
        print("Exiting")
        sys.exit(1)

    if target == 'x86_64-unknown-linux-gnu':
        current_hash = match_list[1].group(1)
        contents = contents.replace(current_hash, hash)
    elif target == 'x86_64-apple-darwin':
        current_hash = match_list[0].group(1)
        contents = contents.replace(current_hash, hash)
    else:
        print(f"TARGET {target} is not a valid target")
        print("Exiting")
        sys.exit(1)

    f.seek(0)
    f.write(contents)
    f.truncate()

# update the tag
with open(formula_path, 'r+') as f:
    contents = f.read()

    contents = re.sub('v[0-9]+\.[0-9]+\.[0-9]+', tag, contents)

    f.seek(0)
    f.write(contents)
    f.truncate()
    
