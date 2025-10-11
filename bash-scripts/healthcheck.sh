#! bin/bash
curl -sS -f --max-time 5 http://localhost:8080/api/hello || exit 1 