CREATE TABLE IF NOT EXISTS items (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    amount INTEGER NOT NULL,
    barcode INTEGER,
    brands TEXT,
    img_url TEXT,
    thumb_url TEXT,
    quantity TEXT
);
