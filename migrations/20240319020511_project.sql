PRAGMA foreign_keys = ON ;

CREATE TABLE IF NOT EXISTS settings
(
    settings_id             INTEGER PRIMARY KEY NOT NULL,
    description             TEXT                NOT NULL,
    created_on              DATETIME DEFAULT (datetime('now','localtime')),
    updated_on              DATETIME DEFAULT (datetime('now','localtime')),
    done                    BOOLEAN             NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS project
(
    project_id                   INTEGER PRIMARY KEY AUTOINCREMENT,
    product_name                 TEXT ,
    created_on                   DATETIME DEFAULT (datetime('now','localtime')),
    updated_on                   DATETIME DEFAULT (datetime('now','localtime')),
    img_directory                TEXT NOT NULL,
    out_directory                TEXT NOT NULL,
    status                       TEXT NOT NULL,
    settings_id                  INTEGER  NOT NULL DEFAULT 1,
    FOREIGN KEY (settings_id)    REFERENCES settings (settings_id) ON UPDATE SET NULL ON DELETE SET NULL
);-- Add migration script here
