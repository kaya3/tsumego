#!/usr/bin/python3

import argparse
from glob import glob
import json
from natsort import natsorted
import os

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

def write_json_output(path: str, problems):
    output = json.dumps({
        'problems': [p.to_output_format() for p in problems],
    })
    
    with open(path, 'w') as output_file:
        output_file.write(output)

def write_sqlite_output(path: str, problems):
    with open(path, 'w') as output_file:
        prefix = 'INSERT OR REPLACE INTO tsumego (name, board, tree) VALUES'
        for problem in problems:
            name = problem.name
            board = problem.to_board_string()
            tree = json.dumps(problem.tree)
            
            output_file.write(f"{prefix}\n    ('{name}', '{board}', '{tree}')")
            prefix = ','

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('-i', '--input_path')
    parser.add_argument('-o', '--output_file')
    parser.add_argument('-n', '--max_problems', type=int, default=None)
    args = parser.parse_args()
    
    output_file = args.output_file
    if output_file.endswith('.json'):
        write = write_json_output
    elif output_file.endswith('.sql'):
        write = write_sqlite_output
    else:
        print('Output must be a .json or .sql file')
        os.exit(1)
    
    paths = find_json_paths(args.input_path, args.max_problems)
    
    problems = []
    num_failed = 0
    
    for path in paths:
        try:
            problem = load_problem(path, args.input_path)
            
            # Ensure that every problem is "black to play"
            if problem.next_player != 'b':
                problem = problem.swap_colours()
            
            problems.append(problem)
        except Exception as e:
            # Some of the problems in this library have no solution, in which case
            # an error is raised; just skip these
            print(f'Failed to load {path}: {e}')
            num_failed += 1
    
    write(output_file, problems)
    
    print(f'Successfully wrote {len(problems)} problem(s) to {output_file}')
    if num_failed > 0:
        print(f'Failed to convert {num_failed} problem(s)');

if __name__ == '__main__':
    main()
