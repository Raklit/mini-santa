SECRET SANTA PROJECT ON RUST. ALMOST READY TO PRODUCTION

Statup:
- create .env folder
- create ".basic_info" file in it
- create also ".admin_info" file


.basic_info content example
DOMAIN_NAME: ""
DOMAIN_EMAIL: ""
DONATION_LINK: ""
CONTACT_EMAIL: ""

Fill data like your want or left it empty (doesn't use, but need for future updates)

.admin_info content example

SERVER_SECRET: "123456"
ADMIN_LOGIN: "admin"
ADMIN_PASSWORD: "qwerty123456"
ADMIN_NICKNAME: "BigBoss"
ADMIN_EMAIL: "admin@test.ru"

Change value like your want. Only latin and cyrillic letters allowed in nicknames.
Password minimum length is 12 chars.
Email available in data model, but verify doesn't implemented yet.
Phone is available in data model too, but doesn't implemented yet.


PLAN:
- end oauth2.0
- split santa stuff from core folder
- develop api for oauth 2.0
- develop api for secret santa
- develop front end
- ???
- PROFIT

Status:
- weak oauth2.0 server
- santa stuff is worked
- frontend is almost ready