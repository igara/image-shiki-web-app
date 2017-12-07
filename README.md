# image-shiki-web-app

### 初回起動時

```
cd image-shiki-web-app
# 環境設定
sh env.sh
openssl genrsa -aes128 2048 > server.key
openssl req -new -key server.key > server.csr
openssl x509 -in server.csr -days 365000 -req -signkey server.key > server.crt
vagrant up --provision
```

vagrantのOSにSSHする

```
cd image-shiki-web-app
vagrant ssh
(cd /vagrant && docker-sync start)
(cd /vagrant && docker-compose up --build -d)

# 起動確認はこちら
(cd /vagrant && docker-compose logs -f)
# nginx、wwwがupになって入ればOK
(cd /vagrant && docker-compose ps)
# コンテナの中に入る
(cd /vagrant/ && docker-compose exec nginx sh)
```
