mkdir -p frontend/out
mkdir -p backend/static

cp -a frontend/html/. backend/static/

tsc --project frontend/tsconfig.json \
    && cp frontend/out/app.js backend/static/

tsc --project frontend/test/tsconfig.json
