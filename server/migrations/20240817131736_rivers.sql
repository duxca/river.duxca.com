PRAGMA foreign_keys = ON;

CREATE TABLE rivers (
   river_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
   name TEXT NOT NULL UNIQUE,
   created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
   updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE river_waypoints (
  river_waypoint_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  river_id INTEGER NOT NULL,
  name TEXT NOT NULL,
  description TEXT NOT NULL DEFAULT '',
  longitude REAL NOT NULL,
  latitude REAL NOT NULL,
  created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  UNIQUE (river_id, name, description, longitude, latitude),
  UNIQUE (longitude, latitude),
  FOREIGN KEY (river_id) references rivers(river_id) ON DELETE CASCADE
);
