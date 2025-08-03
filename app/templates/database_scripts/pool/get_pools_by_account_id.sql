SELECT id, name, description, account_id,
min_price, max_price, 
lifetime, creation_date, pool_state
FROM pools
WHERE
    account_id = '{{account_id}}';