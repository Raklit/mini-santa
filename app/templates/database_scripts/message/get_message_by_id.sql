SELECT 
    id, text_content, account_id, room_id, creation_date
FROM messages
WHERE id = '{{id}}';