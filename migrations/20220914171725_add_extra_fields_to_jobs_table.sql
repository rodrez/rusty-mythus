-- Add migration script here
ALTER TABLE jobs 
ADD COLUMN agreement BOOLEAN NOT NULL,
ADD first_name TEXT NOT NULL,
ADD last_name TEXT NOT NULL,
ADD company_size TEXT NOT NULL,
ADD phone_number TEXT NOT NULL;