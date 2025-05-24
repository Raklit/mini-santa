UPDATE account_sessions
SET
    last_usage_date = '{{now}}'
WHERE 
    id = '{{id}}';