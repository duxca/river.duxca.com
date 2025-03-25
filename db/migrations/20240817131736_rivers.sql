PRAGMA foreign_keys = ON;

CREATE TABLE rivers (
  river_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  river_name TEXT NOT NULL UNIQUE,
  -- 代表点の緯度経度
  -- ex. [35.6895, 139.6917]
  waypoint TEXT NOT NULL,
  created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE river_tracks (
  river_track_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  river_id INTEGER NOT NULL,
  -- 作者
  user_id INTEGER NOT NULL,
  -- ex. 多摩川上流
  track_name TEXT NOT NULL,
  description TEXT NOT NULL,
  -- route is a JSON array of tuple of latitude and longitude: Array<[緯度, 経度]>
  -- ex. [[35.6895, 139.6917], [35.6895, 139.6917], [35.6895, 139.6917]]
  track TEXT NOT NULL,
  created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  FOREIGN KEY (river_id) references rivers(river_id) ON DELETE CASCADE,
  FOREIGN KEY (user_id) references users(user_id) ON DELETE CASCADE
);

CREATE TABLE river_waypoints (
  river_waypoint_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  river_id INTEGER NOT NULL,
  user_id INTEGER NOT NULL,
  waypoint_name TEXT NOT NULL,
  description TEXT NOT NULL,
  -- 代表点の緯度経度
  -- ex. [35.6895, 139.6917]
  waypoint TEXT NOT NULL,
  created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  FOREIGN KEY (river_id) references rivers(river_id) ON DELETE CASCADE,
  FOREIGN KEY (user_id) references users(user_id) ON DELETE CASCADE
);
