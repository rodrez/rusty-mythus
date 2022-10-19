-- Add migration script hereAAA
CREATE TABLE users(
    id uuid PRIMARY KEY,
    first_name TEXT NOT NULL,
    username TEXT NOT NULL,
    last_name TEXT NOT NULL, 
    email TEXT NOT NULL,
    phone TEXT NOT NULL,
    zipcode TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    salt TEXT NOT NULL,
    experience json,
    education json,
    skills TEXT []
);

