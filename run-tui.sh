#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
db_path="${QBZ_DB_PATH:-/tmp/qobuz-player.db}"

cd "$repo_root"

export DATABASE_URL="sqlite://$db_path"

if [[ ! -f "$db_path" ]]; then
    cargo sqlx db create
fi

cargo sqlx migrate run --source player-module/migrations
cargo run -p tui-module --bin qobuz-player -- "$@"
