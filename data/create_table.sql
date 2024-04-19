CREATE TABLE IF NOT EXISTS items (
    id INTEGER PRIMARY KEY NOT NULL,
    -- name TEXT NOT NULL,
    amount INTEGER NOT NULL,
    completed BOOLEAN NOT NULL,
);

CREATE TABLE IF NOT EXISTS item_variants (
    id INTEGER PRIMARY KEY NOT NULL,
    variant_of INTEGER NOT NULL, -- foreign key -> items.id
    name TEXT NOT NULL,
    amount INTEGER NOT NULL,
    shop_id INTEGER, -- foreign key -> shops.id
    barcode INTEGER,
    brands TEXT,
    img_url TEXT,
    thumb_url TEXT,
    quantity TEXT
);

CREATE TABLE IF NOT EXISTS shops (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
);
