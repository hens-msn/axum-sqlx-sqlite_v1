-- Add migration script here

CREATE TABLE products (
    id BLOB PRIMARY KEY,
    name TEXT NOT NULL,
    price REAL NOT NULL,
    stock INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Trigger utk update timestamp (SQLite version)
CREATE TRIGGER IF NOT EXISTS update_products_updated_at
AFTER UPDATE ON products
BEGIN
    UPDATE products 
    SET updated_at = datetime('now') 
    WHERE id = OLD.id;
END;