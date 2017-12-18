#!/bin/sh

echo "フラグファイルを削除します"
rm -rf flag

echo "npm installを開始します"
npm install
npm rebuild node-sass

echo "フラグファイルを作成します"
mkdir flag
touch flag/parcel.flag
