-- Add migration script here
BEGIN;
    -- 
    UPDATE jobs
        SET agreement = true
        WHERE agreement IS NULL;
    
    UPDATE jobs
        SET first_name = 'Jonh'
        WHERE first_name IS NULL;
    
    UPDATE jobs
        SET last_name = 'Doe'
        WHERE last_name IS NULL;
    
    UPDATE jobs
        SET company_size = 'Just Me'
        WHERE company_size IS NULL;
    
    UPDATE jobs
        SET phone_number = 'N/A'
        WHERE phone_number IS NULL;
COMMIT;