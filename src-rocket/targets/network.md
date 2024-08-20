```bash
docker network create --subnet=10.10.0.0/16 my_network
docker network inspect my_network

version: '3.8'

services:
  web:
    image: php:7.4-apache
    container_name: php_apache
    ports:
      - "80:80"
    volumes:
      - ./html:/var/www/html
    networks:
      my_network:
        ipv4_address: 10.10.0.10

networks:
  my_network:
    external: true


docker-compose down
docker-compose up -d

docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' php_apache
```
