SELECT 
    id, pool_id, mailer_id, recipient_id, room_state
FROM rooms
WHERE
    mailer_id = '{{account_id}}' OR 
    recipient_id = '{{recipient_id}}';