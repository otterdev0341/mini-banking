-- Enable foreign key constraints
PRAGMA foreign_keys = ON;

-- Table: AssetType
CREATE TABLE AssetType (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);

-- Table: Asset
CREATE TABLE Asset (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    asset_type_id INTEGER NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,
    FOREIGN KEY (asset_type_id) REFERENCES AssetType(id)
);

-- Table: ExpenseType
CREATE TABLE ExpenseType (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);

-- Table: Expense
CREATE TABLE Expense (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    description TEXT NOT NULL,
    expense_type_id INTEGER NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,
    FOREIGN KEY (expense_type_id) REFERENCES ExpenseType(id)
);

-- Table: CustomerType
CREATE TABLE CustomerType (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL CHECK(name IN ('Customer', 'Vendor')),
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);

-- Table: Contact
CREATE TABLE Contact (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    business_name TEXT,
    phone TEXT,
    description TEXT,
    customer_type_id INTEGER NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,
    FOREIGN KEY (customer_type_id) REFERENCES CustomerType(id)
);

-- Table: Transaction
CREATE TABLE Transaction (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    transaction_type TEXT NOT NULL CHECK(transaction_type IN ('Income', 'Payment', 'Transfer')),
    amount DECIMAL(10,2) NOT NULL,
    asset_id INTEGER NOT NULL,
    destination_asset_id INTEGER,
    expense_id INTEGER,
    contact_id INTEGER,
    note TEXT,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,
    FOREIGN KEY (asset_id) REFERENCES Asset(id),
    FOREIGN KEY (destination_asset_id) REFERENCES Asset(id),
    FOREIGN KEY (expense_id) REFERENCES Expense(id),
    FOREIGN KEY (contact_id) REFERENCES Contact(id)
);

-- Table: CurrentSheet
CREATE TABLE CurrentSheet (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    asset_id INTEGER NOT NULL,
    balance DECIMAL(10,2) NOT NULL,
    updated_at DATETIME NOT NULL,
    FOREIGN KEY (asset_id) REFERENCES Asset(id)
);
