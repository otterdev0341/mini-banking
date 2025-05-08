-- Create AssetType table
/* done */
CREATE TABLE AssetType (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

/* done */
-- Create Asset table
CREATE TABLE Asset (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    asset_type_id INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (asset_type_id) REFERENCES AssetType(id)
);

-- Create ExpenseType table
/* done */
CREATE TABLE ExpenseType (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Create Expense table
/* done */
CREATE TABLE Expense (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    description TEXT NOT NULL,
    expense_type_id INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (expense_type_id) REFERENCES ExpenseType(id)
);

/* done */
-- Create ContactType table
CREATE TABLE ContactType (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Create Contact table
/* done */
CREATE TABLE Contact (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    business_name TEXT NOT NULL,
    phone TEXT NOT NULL,
    description TEXT,
    contact_type_id INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (contact_type_id) REFERENCES ContactType(id)
);

-- Create TransactionRecord table
/* done */
CREATE TABLE TransactionRecord (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    transaction_type TEXT NOT NULL CHECK (transaction_type IN ('Income', 'Payment', 'Transfer')),
    amount REAL NOT NULL,
    asset_id INTEGER NOT NULL,
    destination_asset_id INTEGER,
    expense_id INTEGER,
    contact_id INTEGER,
    note TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (asset_id) REFERENCES Asset(id),
    FOREIGN KEY (destination_asset_id) REFERENCES Asset(id),
    FOREIGN KEY (expense_id) REFERENCES Expense(id),
    FOREIGN KEY (contact_id) REFERENCES Contact(id)
);

-- Create CurrentSheet table
CREATE TABLE CurrentSheet (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    asset_id INTEGER NOT NULL,
    balance REAL NOT NULL,
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (asset_id) REFERENCES Asset(id)
);
