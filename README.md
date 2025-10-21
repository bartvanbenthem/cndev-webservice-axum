# CNDEV Webservice Axum
Project template for creating RESTful web services in Rust and Axum, with a relational database backend and example content. HTML5 and JS are used for a client-side rendered graphical interface. 

![page_screen](page_screen.png)

## Run Local

### install and start Mail Server
```bash
# On Docker
docker run -d -p 1025:1025 -p 8025:8025 mailhog/mailhog

# On Linux
mkdir ~/mailhog/
cd ~/mailhog
go mod init mailhog
go get github.com/mailhog/MailHog
go install github.com/mailhog/MailHog
~/go/bin/MailHog
```
Now the mail server is running on port 1025, To view the UI, start a web browser and go to: http://localhost:8025/

### Download & Start CNDEV Axum webapp
```bash
source ../00-ENV/env.sh # get $CR_PAT env variable
git clone https://bartvanbenthem:$CR_PAT@github.com/bartvanbenthem/cndev-webservice-axum.git
cd cndev-webservice-axum

docker compose up --build
docker container ls
```

## Run in the cloud:
Run HAProxy container to forward all incoming http/https traffic to the webapp container

### Allow http/https on Firewall
```bash
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
sudo ufw reload
sudo ufw status
```

### Automated TLS certificates
```bash
apt update && apt upgrade -y
apt install -y certbot

certbot certonly --standalone -d cndev.nl -d www.cndev.nl

sudo sh -c 'cat /etc/letsencrypt/live/cndev.nl/fullchain.pem \
 /etc/letsencrypt/live/cndev.nl/privkey.pem \
 > /etc/letsencrypt/live/cndev.nl/haproxy.pem'

sudo chmod 600 /etc/letsencrypt/live/cndev.nl/haproxy.pem
ls -l /etc/letsencrypt/live/cndev.nl/haproxy.pem

```

### HAProxy config
```txt
global
    log stdout format raw local0

defaults
    log     global
    mode    http
    option  httplog
    option  dontlognull
    timeout connect 5s
    timeout client  50s
    timeout server  50s

# HTTP frontend
frontend http_front
    bind *:80
    mode http
    option httplog

    # Route Let's Encrypt validation requests
    acl letsencrypt-acl path_beg /.well-known/acme-challenge/
    use_backend letsencrypt-backend if letsencrypt-acl

    # Otherwise, redirect to HTTPS
    redirect scheme https code 301 if !letsencrypt-acl

# HTTPS frontend
frontend https_front
    bind *:443 ssl crt /etc/letsencrypt/live/cndev.nl/haproxy.pem
    mode http
    option httplog
    default_backend web_backend

# Backend for your app
backend web_backend
    server webapp 127.0.0.1:3002 check

# Backend for Let's Encrypt challenges
backend letsencrypt-backend
    server certbot 127.0.0.1:54321
```
### run HAProxy in docker container with config
```bash
docker run -d \
  --name haproxy \
  --network host \
  --user root \
  -v $(pwd)/haproxy.cfg:/usr/local/etc/haproxy/haproxy.cfg:ro \
  -v /etc/letsencrypt/live/cndev.nl:/etc/letsencrypt/live/cndev.nl:ro \
  haproxy:alpine
```