SELECT id, account_id, room_id, pool_id, wishlist 
FROM members
WHERE
    id = '{{id}}';