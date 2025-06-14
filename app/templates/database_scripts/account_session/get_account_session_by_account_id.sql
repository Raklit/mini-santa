SELECT id, account_id, 
    access_token, refresh_token, 
    is_active, is_ended,
    start_date,
    access_token_creation_date, refresh_token_creation_date,
    last_usage_date
FROM account_sessions WHERE account_id = '{{account_id}}';