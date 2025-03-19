PRAGMA foreign_keys = ON;

CREATE TABLE fields (
  field_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  field_name TEXT NOT NULL UNIQUE,
  route TEXT NOT NULL DEFAULT '[]',
  description TEXT NOT NULL DEFAULT '',
  created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE field_spots (
  field_spot_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  field_id INTEGER NOT NULL,
  spot_name TEXT NOT NULL,
  spot_type TEXT NOT NULL,
  description TEXT NOT NULL DEFAULT '',
  longitude REAL NOT NULL,
  latitude REAL NOT NULL,
  created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  UNIQUE (field_id, spot_name, longitude, latitude),
  FOREIGN KEY (field_id) references fields(field_id) ON DELETE CASCADE
);
