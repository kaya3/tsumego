#!/usr/bin/python3

INPUT_PATH = 'sanderland-tsumego/problems/'
OUTPUT_FILENAME = 'all_problems.json'
MAX_PROBLEMS = 100

from glob import glob
import json
from natsort import natsorted

from problem import Problem

def find_json_paths(path_prefix: str, max_paths=None):
    paths = glob(f'{path_prefix}**/*.json', recursive=True)
    paths = natsorted(paths)
    if max_paths is not None:
        paths = paths[:max_paths]
    return paths

def load_problem(path: str, path_prefix: str):
    with open(path) as json_file:
        json_str = json_file.read()
    
    name = path.removeprefix(path_prefix).removesuffix('.json')
    return Problem.from_input_format(name, json.loads(json_str))

def write_output(path: str, problems):
    with open(path, 'w') as output_file:
        output_file.write(json.dumps({
            'problems': problems,
        }))

def main():
    paths = find_json_paths(INPUT_PATH, MAX_PROBLEMS)
    
    problems = []
    num_failed = 0
    
    for path in paths:
        try:
            problem = load_problem(path, INPUT_PATH)
            
            # Ensure that every problem is "black to play"
            if problem.next_player != 'b':
                problem = problem.swap_colours()
            
            problems.append(problem.to_output_format())
        except Exception as e:
            # Some of the problems in this library have no solution, in which case
            # an error is raised; just skip these
            print(f'Failed to load {path}: {e}')
            num_failed += 1
    
    write_output(OUTPUT_FILENAME, problems)
    
    print(f'Successfully wrote {len(problems)} problem(s) to {OUTPUT_FILENAME}')
    if num_failed > 0:
        print(f'Failed to convert {num_failed} problem(s)');

if __name__ == '__main__':
    main()
