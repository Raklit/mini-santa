SELECT id, account_id, 
    auth_token, refresh_token,
    start_date,
    auth_token_creation_date, refresh_token_creation_date,
    last_usage_date
FROM account_sessions WHERE id = '{{id}}';