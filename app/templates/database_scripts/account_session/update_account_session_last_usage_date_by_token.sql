UPDATE account_sessions
SET
    last_usage_date = '{{now}}'
WHERE 
    access_token = '{{token}}' OR refresh_token = '{{token}}';