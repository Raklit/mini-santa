INSERT INTO account_sessions (
    id, account_id, 
    auth_token, refresh_token,
    start_date,
    auth_token_creation_date, refresh_token_creation_date,
    last_usage_date
) VALUES (
    '{{id}}', '{{account_id}}', 
    '{{auth_token}}', '{{refresh_token}}',
    '{{start_date}}', 
    '{{auth_token_creation_date}}', '{{refresh_token_creation_date}}', 
    '{{last_usage_date}}'
);