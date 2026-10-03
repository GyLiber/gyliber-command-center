#!/usr/bin/env bash
# Uses disposable local containers only; never supply a production database URL.
set -euo pipefail

image="${1:-gyliber-command-center:ci}"
expected_version="$(sed -n 's/^version = "\([^"]*\)"$/\1/p' Cargo.toml)"
[[ -n "$expected_version" ]] || { echo 'Cannot read the expected release from Cargo.toml' >&2; exit 1; }
for dependency in docker curl jq openssl; do
  command -v "$dependency" >/dev/null || { echo "Missing dependency: $dependency" >&2; exit 1; }
done

work_dir="$(mktemp -d)"
run_id="gyliber-smoke-$(openssl rand -hex 6)"
network="$run_id"
database="$run_id-db"
application="$run_id-app"
missing_database="$run_id-no-db"

cleanup() {
  local result=$?
  trap - EXIT
  if (( result != 0 )); then
    # All runtime configuration is generated test data, never production secrets.
    docker logs "$application" >&2 || true
    docker logs "$database" >&2 || true
  fi
  docker rm -f -v "$application" "$database" "$missing_database" >/dev/null 2>&1 || true
  docker network rm "$network" >/dev/null 2>&1 || true
  rm -rf "$work_dir"
  exit "$result"
}
trap cleanup EXIT

session_key="$(openssl rand -hex 64)"
database_password="$(openssl rand -hex 32)"
cat > "$work_dir/application.env" <<EOF
APP_ENV=production
COOKIE_SECURE=true
SESSION_MASTER_KEY=$session_key
GITHUB_CLIENT_ID=container-smoke-test
GITHUB_CLIENT_SECRET=container-smoke-test
GITHUB_REDIRECT_URL=https://example.invalid/auth/github/callback
GYLIBER_ALLOWED_GITHUB_LOGINS=container-smoke-test
EOF
printf 'POSTGRES_PASSWORD=%s\nPOSTGRES_DB=gyliber_smoke\n' "$database_password" > "$work_dir/database.env"

# Production must not silently start with MemoryStore when DATABASE_URL is absent.
docker run --detach --name "$missing_database" --network none \
  --env-file "$work_dir/application.env" "$image" >/dev/null
for (( attempt=0; attempt<30; attempt++ )); do
  [[ "$(docker inspect --format '{{.State.Running}}' "$missing_database")" == false ]] && break
  sleep 1
done
[[ "$(docker inspect --format '{{.State.Running}}' "$missing_database")" == false ]]
[[ "$(docker inspect --format '{{.State.ExitCode}}' "$missing_database")" != 0 ]]
docker logs "$missing_database" > "$work_dir/no-database.log" 2>&1
grep -Fq 'DATABASE_URL is required in production' "$work_dir/no-database.log"

docker network create "$network" >/dev/null
docker run --detach --name "$database" --network "$network" \
  --env-file "$work_dir/database.env" postgres:18 >/dev/null
database_ready=false
for (( attempt=0; attempt<60; attempt++ )); do
  if docker exec "$database" pg_isready -h 127.0.0.1 -U postgres -d gyliber_smoke >/dev/null 2>&1; then
    database_ready=true
    break
  fi
  sleep 1
done
[[ "$database_ready" == true ]]

query_database() {
  docker exec "$database" psql -U postgres -d gyliber_smoke -At -v ON_ERROR_STOP=1 -c "$1"
}
[[ "$(query_database "SELECT to_regclass('public.gyliber_sessions') IS NULL")" == t ]]
printf 'DATABASE_URL=postgresql://postgres:%s@%s:5432/gyliber_smoke\n' \
  "$database_password" "$database" >> "$work_dir/application.env"

docker run --detach --name "$application" --network "$network" \
  --publish 127.0.0.1::3000 --env-file "$work_dir/application.env" "$image" >/dev/null
[[ "$(docker inspect --format '{{.Config.User}}' "$application")" == appuser ]]
address="http://$(docker port "$application" 3000/tcp)"

wait_for_health() {
  for (( attempt=0; attempt<60; attempt++ )); do
    if curl --silent --show-error --fail --max-time 2 \
      "$address/api/health" > "$work_dir/health.json" 2>/dev/null; then
      jq -e --arg version "$expected_version" \
        '.service == "gyliber-command-center" and .status == "ok" and .version == $version' \
        "$work_dir/health.json" >/dev/null
      return
    fi
    [[ "$(docker inspect --format '{{.State.Running}}' "$application")" == true ]] || return 1
    sleep 1
  done
  return 1
}

expect_status() {
  local actual
  actual="$(curl --silent --show-error --max-time 5 --output /dev/null \
    --write-out '%{http_code}' "$address$1")"
  [[ "$actual" == "$2" ]] || { echo "$1 returned $actual; expected $2" >&2; return 1; }
}

wait_for_health
[[ "$(query_database "SELECT to_regclass('public.gyliber_sessions') IS NOT NULL")" == t ]]
[[ "$(query_database 'SELECT count(*) FROM gyliber_sessions')" == 0 ]]
expect_status / 200
expect_status /static/app.css 200
expect_status /command 303
expect_status /command/math-playground 303
expect_status /static/math-playground.mjs 200
[[ "$(query_database "SELECT to_regclass('public.gyliber_math_exhibits') IS NOT NULL")" == t ]]
for route in /api/state /api/modules /api/repository /api/resources /api/math-playground /api/math-playground/demos/giant-pi/engine.mjs /api/math-playground/curated /api/math-playground/authoring-template /api/math-playground/curated/metric-couriers/0.1.0/engine.mjs; do
  expect_status "$route" 401
done

# Starting OAuth creates a persisted state/PKCE session without calling GitHub.
# Do not follow the redirect or use real OAuth credentials.
expect_status /auth/github/start 303
[[ "$(query_database 'SELECT count(*) FROM gyliber_sessions')" == 1 ]]
query_database 'SELECT id FROM gyliber_sessions' > "$work_dir/session-before"

# A second startup must accept the existing schema and leave the record intact.
docker restart "$application" >/dev/null
# Docker can allocate a new host port for an ephemeral binding on restart.
address="http://$(docker port "$application" 3000/tcp)"
wait_for_health
query_database 'SELECT id FROM gyliber_sessions' > "$work_dir/session-after"
cmp "$work_dir/session-before" "$work_dir/session-after"
[[ "$(query_database 'SELECT count(*) FROM gyliber_sessions')" == 1 ]]
expect_status /api/state 401

echo 'Production container checks passed: fail-closed configuration, migration, health, assets, access boundary and restart.'
