WITH ranked AS (
  SELECT
    id,
    room_id,
    creation_date,
    ROW_NUMBER() OVER (
      PARTITION BY room_id
      ORDER BY creation_date DESC, id DESC
    ) AS rn
  FROM messages
)
DELETE FROM messages
WHERE id IN (
  SELECT id FROM ranked WHERE rn > '{{limit}}'
) OR DATETIME(creation_date, '+{{lifetime}} seconds') < '{{now}}';