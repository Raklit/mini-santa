SELECT id, account_id, 
    access_token, refresh_token, 
    start_date,
    access_token_creation_date, refresh_token_creation_date,
    last_usage_date
FROM account_sessions WHERE refresh_token = '{{refresh_token}}';