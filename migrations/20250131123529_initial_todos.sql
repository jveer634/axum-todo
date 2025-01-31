-- Add migration script here

CREATE TABLE todo (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  title TEXT NOT NULL,
  status TEXT CHECK (status IN ('Created', 'InProgress', 'Completed')) NOT NULL DEFAULT 'Created',
  created_on TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_on TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);