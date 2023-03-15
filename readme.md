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
