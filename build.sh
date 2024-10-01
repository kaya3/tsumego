mkdir -p frontend/out

python3 problems/build.py -i sanderland-tsumego/problems/ -n 100 -o frontend/out/all_problems.json

tsc --project frontend/tsconfig.json
tsc --project frontend/test/tsconfig.json
