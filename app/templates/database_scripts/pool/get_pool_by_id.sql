SELECT id, name, description, account_id,
min_price, max_price, 
is_creator_involved, lifetime, creation_date, pool_state
FROM pools
WHERE
    id = '{{id}}';