-- Your SQL goes here
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
