UPDATE accounts
SET 
    password_hash = '{{password_hash}}',
    password_salt = '{{password_salt}}'
WHERE 
    id = '{{id}}';
