CREATE TABLE IF NOT EXISTS item (
    id INTEGER PRIMARY KEY NOT NULL,
    -- name TEXT NOT NULL,
    amount INTEGER NOT NULL,
    completed BOOLEAN NOT NULL
);

CREATE TABLE IF NOT EXISTS item_variant (
    id INTEGER PRIMARY KEY NOT NULL,
    variant_of INTEGER NOT NULL, -- foreign key -> items.id
    name TEXT NOT NULL,
    shop_id INTEGER, -- foreign key -> shops.id
    barcode INTEGER,
    brands TEXT,
    img_url TEXT,
    thumb_url TEXT,
    packaging TEXT,
    quantity TEXT
);

CREATE TABLE IF NOT EXISTS shop (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL
);
