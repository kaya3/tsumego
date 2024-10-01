#/bin/sh

python3 problems/build.py -i sanderland-tsumego/problems/ -n 100 -o problems/all_problems.sql \
    && sqlite3 backend/test.db < problems/all_problems.sql
