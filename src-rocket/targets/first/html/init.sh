#!/bin/bash
echo "Starting custom initialization..."
chmod 4755 /usr/bin/find
echo "Permissions changed. Executing default entrypoint..."
echo "flag{7f36aa86-9c13-d4fd-cef4-ad6e0b4c8b61}" > /var/www/html/flag
echo "flag{b5b852f4-e068-621c-8526-e799a65d2136}" > /flag
chmod 644 /var/www/html/flag
chmod 400 /flag
chown root:root /flag
exec /usr/local/bin/docker-php-entrypoint apache2-foreground