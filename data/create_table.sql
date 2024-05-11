CREATE TABLE IF NOT EXISTS item (
    id INTEGER PRIMARY KEY NOT NULL,
    -- name TEXT NOT NULL,
    amount INTEGER NOT NULL,
    completed BOOLEAN NOT NULL
);

CREATE TABLE IF NOT EXISTS item_variant (
    id INTEGER PRIMARY KEY NOT NULL,
    variant_of INTEGER NOT NULL, -- foreign key -> items.id
    name TEXT NOT NULL CHECK(name <> ''),
    -- shop_id INTEGER, -- foreign key -> shops.id
    shop TEXT NOT NULL DEFAULT '',
    barcode INTEGER,
    brands TEXT NOT NULL DEFAULT '',
    img_url TEXT DEFAULT '',
    thumb_url TEXT DEFAULT '',
    packaging TEXT NOT NULL DEFAULT '',
    quantity TEXT NOT NULL DEFAULT ''
);

-- CREATE TABLE IF NOT EXISTS shop (
--     id INTEGER PRIMARY KEY NOT NULL,
--     name TEXT NOT NULL
-- );
