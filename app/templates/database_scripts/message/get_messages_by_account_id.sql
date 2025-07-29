SELECT 
    id, text_content, account_id, room_id, pool_id, creation_date
FROM messages
WHERE account_id = '{{account_id}}';