UPDATE account_sessions
SET 
    access_token = '{{access_token}}',
    refresh_token = '{{refresh_token}}',
    access_token_creation_date = '{{now}}',
    refresh_token_creation_date = '{{now}}',
    last_usage_date = '{{now}}'
WHERE 
    refresh_token = '{{old_refresh_token}}';