# image-shiki-web-app

### 初回起動時

hostsの変更

```
# vagrantを使用せずにdockerのIPから設定する場合
XXX.XXX.XXX.XXX local.syonet.work
```

```
cd image-shiki-web-app
# 環境設定
sh env.sh
# vagrantfileにコメント記載している箇所を修正
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
