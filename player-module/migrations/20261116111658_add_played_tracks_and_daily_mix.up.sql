CREATE TABLE IF NOT EXISTS played_tracks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    track_id INTEGER NOT NULL,
    artist_id INTEGER,
    genre_id INTEGER,
    label_id INTEGER,
    played_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS played_tracks_played_at ON played_tracks (played_at);
CREATE INDEX IF NOT EXISTS played_tracks_genre ON played_tracks (genre_id, played_at);

CREATE TABLE IF NOT EXISTS daily_mix (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    day TEXT NOT NULL,
    tracks TEXT NOT NULL
);
