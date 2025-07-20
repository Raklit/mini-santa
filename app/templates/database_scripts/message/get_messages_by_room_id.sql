SELECT 
    id, text_content, account_id, room_id, creation_date
FROM messages
WHERE room_id = '{{room_id}}';