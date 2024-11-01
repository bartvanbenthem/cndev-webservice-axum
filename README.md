# cndev webservice axum
Project template for creating RESTful web services in Rust and Axum, with a relational database backend and example content. HTML5 and JS are used for a client-side rendered graphical interface. 


### install and start MailHog
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