UPDATE account_sessions
SET
    last_usage_date = '{{now}}'
WHERE 
    auth_token = '{{token}}' OR refresh_token = '{{token}}';