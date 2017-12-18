#!/bin/sh

until [ -e "/nodejs/flag/parcel.flag" ]
do
  echo "まだnpm install中です";
  sleep 5;
done

echo "parcelフラグファイルを削除します"
rm /nodejs/flag/parcel.flag

if [ $WWW_ENV = "local" ]; then
    echo "nodejs_parcel: 開発環境用ビルドを開始します"
    echo "CSS・JSファイルの作成を行います"
    npm run dev_build
elif [ $WWW_ENV = "prod" ]; then
    echo "nodejs_parcel: 本番環境ビルドです"
    echo "CSS・JSファイルの作成を行います"
    npm run prod_build
fi
