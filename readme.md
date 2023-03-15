# essential command

`cargo watch -x check -x test -x fmt -x clippy  -x run`
`curl -v`

`curl -v http://127.0.0.1:8000/health_check`
`curl -i -X POST -d 'email=thomas_mann@hotmail.com&name=Tom' \
http://127.0.0.1:8000/subscriptions`

`TEST_LOG=true cargo test health_check_works|bunyan`

`docker build --tag zero2prod --file Dockerfile .`

`docker run -p 8000:8000 zero2prod`

`docker system prune`
`docker system prune --volumes`
`sudo docker exec -it 775c7c9ee1e1 /bin/bash  `


# Set ssl for postgres in docker
```
su postgres
cd /var/lib/postgresql/data/
openssl req -new -text -passout pass:abcd -subj /CN=localhost -out server.req -keyout privkey.pem
openssl rsa -in privkey.pem -passin pass:abcd -out server.key
openssl req -x509 -in server.req -text -key server.key -out server.crt
chmod 600 server.key
```

change conf:
/var/lib/postgresql/data/postgresql.conf
search `# - SSL -`

```  # - SSL -
- # ssl = off
+ ssl = on
- # ssl_cert_file = ''
+ ssl_cert_file = '/var/lib/postgresql/data/server.crt'
- # ssl_key_file = ''
+ ssl_key_file = '/var/lib/postgresql/data/server.key'```
```
# 3-14~3-15 summary
## Question:
I have a CI drone setting, how to make cd automatic?

## Tasks:
1. docker part
   1. how to clean old docker
      1. stop docker
      2. remove docker
      3. remove image
   2. how to generate new docker
      1. Docker file
      2. run docker
2. image part
   1. how to make new image

## solve method

1. write docker file
2. rewrite db_generation_script
3. learn how to make new image based on docker
   1. for postgres(for ssl)
4. write self dockerfile to reduce CI time spend(in rust ci)