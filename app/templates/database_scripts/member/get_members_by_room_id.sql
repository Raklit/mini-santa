SELECT id, account_id, room_id, pool_id, wishlist 
FROM members
WHERE
    room_id = '{{room_id}}';