SELECT 
    id, pool_id, mailer_id, recipient_id, room_state
FROM rooms
WHERE
    pool_id = '{{pool_id}}';