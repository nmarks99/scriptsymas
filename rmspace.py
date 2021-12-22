#!/usr/bin/env python
import os 

# Get directory contents
contents = os.listdir('.')

# Replace any " " with "_" in file or folder names
for f in contents:
    if " " in f:
        new = f.replace(" ", "_")
        os.rename(f,new)
