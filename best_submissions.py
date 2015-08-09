import os
import json
import requests
import collections

auth = ('', os.environ['API_TOKEN'])
url = "https://davar.icfpcontest.org/teams/54/solutions"

r = requests.get(url, auth=auth)

data = r.json()


per_problem = collections.defaultdict(list)
for row in data:
    per_problem[(row['problemId'], row['seed'])].append(row)

for _, v in per_problem.items():
    v.sort(key=lambda r: -r['score'])

submissions = []
for k, v in per_problem.items():
    if k[0] != 0:
        continue
    tag = 'the best'
    v = v[0]
    submission = {
        'problemId': v['problemId'],
        'seed': v['seed'],
        'tag': tag,
        'solution': v['solution']
    }
    submissions.append(submission)


data = json.dumps(submissions)
print(data)
headers = {'Content-type': 'application/json'}
r = requests.post(url, auth=auth, data=data, headers=headers)
print(r.text)
