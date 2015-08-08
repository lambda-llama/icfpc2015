#!/usr/bin/env python

import requests
import re
import json


url = 'https://davar.icfpcontest.org/rankings.js'
pattern = r'\{[^{]+"lambda-llama"[^}]+\}'

r = requests.get(url)
m = re.search(pattern, r.text)
data = json.loads(m.group())

print("tags:", data['tags'])
print("score:", data['score'])
