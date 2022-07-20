-- Add migration script here
-- title: String,
-- company: String,
-- workplace_type: String,
-- address: String,
-- employment_type: String,
-- description: String,
-- skills: HashMap, // Verify this jsonb type
-- tools: HashMap, // Verify this jsonb type
-- min_salary: String,
-- max_salary: String,
-- salary_rate: String,
-- bonuses: String,
-- benefits: String,
-- is_active: Boolean,
CREATE TABLE jobs
(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    title TEXT NOT NULL,
    company TEXT NOT NULL,
    workplace_type TEXT NOT NULL,
    address TEXT NOT NULL,
    employment_type TEXT NOT NULL,
    description TEXT NOT NULL,
    skills TEXT NOT NULL,
    tools TEXT NOT NULL,
    min_salary TEXT NOT NULL,
    max_salary TEXT NOT NULL,
    salary_rate TEXT NOT NULL,
    bonuses TEXT NOT NULL,
    benefits TEXT NOT NULL,
    is_active BOOLEAN NOT NULL,
    created_at timestamptz NOT NULL,
    updated_at timestamptz NOT NULL
)