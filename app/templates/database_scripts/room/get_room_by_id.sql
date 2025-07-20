SELECT 
    id, pool_id, mailer_id, recipient_id, room_state
FROM rooms
WHERE
    id = '{{id}}';