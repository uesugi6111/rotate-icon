# rotate-icon 

アイコンを回すぜ

### これはなに
 
Twitterのアイコンを毎時回すぜ  
[これ](https://github.com/uesugi6111/generate-rotated-img)で回転アイコンを生成したぜ  
需要？知らんな

### 使い方

1. .env を.env.sample の通りに生成(Twitter APIのキーが必要なので普通無理では)
2. build
```
docker run --rm -it -v  {ここに絶対パス}\rotate-icon:/home/rust/src ekidd/rust-musl-builder cargo build --release
```
3. デプロイイメージの作成
```
docker build ./ t rotate-icon
```
4. aws ecr にリポジトリを作成
5. 作成したリポジトリにpush
6. aws lambda でリポジトリから関数を作成
7. Amazon EventBridge で毎時起動設定
8. おわり
