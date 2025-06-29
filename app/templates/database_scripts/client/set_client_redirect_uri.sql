UPDATE clients
SET 
    redirect_uri = '{{redirect_uri}}'
WHERE 
    id = '{{id}}';
