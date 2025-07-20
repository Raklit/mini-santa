SELECT id, account_id, room_id, wishlist 
FROM members
WHERE
    account_id = '{{account_id}}';