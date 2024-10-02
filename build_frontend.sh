mkdir -p frontend/out
mkdir -p backend/static

cp frontend/index.html backend/static/

tsc --project frontend/tsconfig.json \
    && cp frontend/out/tsumego.js backend/static/

tsc --project frontend/test/tsconfig.json
