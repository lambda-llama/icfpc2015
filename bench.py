#!/usr/bin/env python3

import subprocess
import os

def main():
    subprocess.call(['cargo', 'build', '--release'])
    for i in os.listdir("bench_problems/"):
        if i.endswith(".json"):
            print("run: {}".format(i))
            problem_path = "problems/{}".format(i)
            subprocess.call(['cargo', 'run', '--release',
                             '--', '-s', '-f', problem_path])

if __name__ == "__main__":
    main()
