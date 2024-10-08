#/bin/sh

python3 problems/build.py -i sanderland-tsumego/problems/ -n 100 -o problems/all_problems.sql \
    && cd backend \
    && sqlx database drop \
    && sqlx database create \
    && sqlx migrate run \
    && sqlite3 test.db < add_test_user.sql \
    && cd .. \
    && sqlite3 backend/test.db < problems/all_problems.sql
