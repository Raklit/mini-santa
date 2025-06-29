SELECT 
    id, account_id, code, creation_date
FROM auth_codes
WHERE
    code = '{{code}}';
