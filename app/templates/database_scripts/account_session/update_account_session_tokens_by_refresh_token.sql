UPDATE account_sessions
SET 
    auth_token = '{{auth_token}}',
    refresh_token = '{{refresh_token}}',
    auth_token_creation_date = '{{now}}',
    refresh_token_creation_date = '{{now}}',
    last_usage_date = '{{now}}'
WHERE 
    refresh_token = '{{old_refresh_token}}';