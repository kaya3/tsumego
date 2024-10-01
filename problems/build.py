#!/usr/bin/python3

OUTPUT_FILENAME = 'all_problems.json'

from glob import glob
import json
from natsort import natsorted

from problem import Problem

paths = glob('sanderland-tsumego/problems/**/*.json', recursive=True)
paths = natsorted(paths)

problems = []

for path in paths:
    try:
        with open(path) as json_file:
            json_str = json_file.read()
        
        name = path.replace('sanderland-tsumego/problems/', '')
        problem = Problem.from_input_format(name, json.loads(json_str))
        
        if problem.next_player != 'b':
            problem = problem.swap_colours()
        
        problems.append(problem.to_output_format())
    except Exception as e:
        print(f'Failed to load {path}: {e}')

with open(OUTPUT_FILENAME, 'w') as output_file:
    output_file.write(json.dumps({
        'problems': problems,
    }))
