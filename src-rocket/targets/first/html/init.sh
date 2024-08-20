#!/bin/bash
echo "Starting custom initialization..."
chmod 4755 /usr/bin/find
echo "Permissions changed. Executing default entrypoint..."
exec /usr/local/bin/docker-php-entrypoint apache2-foreground