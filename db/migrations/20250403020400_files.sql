PRAGMA foreign_keys = ON;

CREATE TABLE files (
  file_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  user_id INTEGER NOT NULL,
  content_type TEXT NOT NULL,
  file_size INTEGER NOT NULL,
  gcs_path TEXT NOT NULL,
  created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  FOREIGN KEY (user_id) references users(user_id) ON DELETE CASCADE
);
