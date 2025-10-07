#! bin/bash
envsubst '$SERVER_SECRET $ADMIN_LOGIN $ADMIN_PASSWORD $ADMIN_NICKNAME $ADMIN_EMAIL' < ./Config.toml.template > ./Config.toml
/executables/mini-santa