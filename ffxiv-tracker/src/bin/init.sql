CREATE TABLE IF NOT EXISTS profile_snapshots (
    snapshot_id INTEGER PRIMARY KEY NOT NULL,
    user_id INTEGER NOT NULL,
    free_company TEXT,
    name TEXT NOT NULL,
    nameday TEXT NOT NULL,
    guardian TEXT NOT NULL,
    city_state TEXT NOT NULL,
    server TEXT NOT NULL,
    race_clan_gender TEXT NOT NULL,
    hp INTEGER NOT NULL,
    mp INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS experience_snapshots (
    id INTEGER PRIMARY KEY NOT NULL,
    snapshot_id INTEGER NOT NULL,
    class_name TEXT NOT NULL,
    xp_level INTEGER NOT NULL,
    current_xp INTEGER,
    max_xp INTEGER,
    FOREIGN KEY (snapshot_id) REFERENCES profile_snapshots (snapshot_id)
);
