UPDATE account_sessions
SET 
    auth_token = '{{auth_token}}',
    auth_token_creation_date = '{{now}}',
    last_usage_date = '{{now}}'
WHERE 
    id = '{{id}}';