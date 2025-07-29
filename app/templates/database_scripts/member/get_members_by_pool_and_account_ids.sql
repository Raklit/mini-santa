SELECT id, account_id, room_id, pool_id, wishlist 
FROM members
WHERE
    pool_id = '{{pool_id}}' AND
    account_id = '{{account_id}}';