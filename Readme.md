# Garlicoin Broadcaster

1. Configure provided `docker-compose.yml` with port and volume information.
1. Configure vars.env with a user and password. I think these do not need not be secure as the node is not publicly accessible but it never hurts.
1. `docker-compose up -d --build --force-recreate` to start the service.
