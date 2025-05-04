UPDATE account_sessions
SET 
    refresh_token = '{{refresh_token}}',
    refresh_token_creation_date = '{{now}}',
    last_usage_date = '{{now}}'
WHERE 
    id = '{{id}}';