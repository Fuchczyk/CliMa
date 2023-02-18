CREATE TABLE client (
    id INTEGER PRIMARY KEY,
    email TEXT NOT NULL,
    name TEXT NOT NULL,
    postcode TEXT NOT NULL,
    city TEXT NOT NULL,
    phone TEXT NOT NULL
);

CREATE TABLE call (
    id INTEGER PRIMARY KEY,
    date DATE NOT NULL,
    description TEXT NOT NULL,
    client INTEGER,
    FOREIGN KEY(client) REFERENCES client(id)
);

CREATE TABLE scheduled_call (
    id INTEGER PRIMARY KEY,
    date DATE NOT NULL,
    description TEXT,
    client INTEGER,
    FOREIGN KEY(client) REFERENCES client(id)
);

CREATE TABLE postcode_mapping (
  postcode TEXT PRIMARY KEY,
  province TEXT NOT NULL
);

CREATE TABLE assortment (
  id INTEGER PRIMARY KEY,
  name TEXT UNIQUE
);

CREATE TABLE client_assortment (
  client_id INTEGER NOT NULL,
  assortment_id INTEGER NOT NULL,
  FOREIGN KEY client_id REFERENCES client(id),
  FOREIGN KEY assortment_id REFERENCES assortment(id)
);

