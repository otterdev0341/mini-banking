-- This file should undo anything in `up.sql`

-- Create temporary table for CurrentSheet
CREATE TABLE CurrentSheet_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    asset_id INTEGER NOT NULL,
    balance FLOAT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Copy data from old table
INSERT INTO CurrentSheet_new SELECT * FROM CurrentSheet;

-- Drop old table
DROP TABLE CurrentSheet;

-- Rename new table
ALTER TABLE CurrentSheet_new RENAME TO CurrentSheet;

-- Create temporary table for TransactionRecord
CREATE TABLE TransactionRecord_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    transaction_type TEXT NOT NULL,
    amount FLOAT NOT NULL,
    asset_id INTEGER NOT NULL,
    destination_asset_id INTEGER,
    expense_id INTEGER,
    contact_id INTEGER,
    note TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Copy data from old table
INSERT INTO TransactionRecord_new SELECT * FROM TransactionRecord;

-- Drop old table
DROP TABLE TransactionRecord;

-- Rename new table
ALTER TABLE TransactionRecord_new RENAME TO TransactionRecord;
