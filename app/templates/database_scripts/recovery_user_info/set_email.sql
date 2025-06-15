UPDATE recovery_user_infos
    SET email = '{{email}}'
WHERE
    account_id = '{{account_id}}';