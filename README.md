# MinWeb2025-blogging-platform-backend

## セットアップ
データベースをやっている人はMongoDBのセットアップ方法の追記をお願いします
```bash
git clone git@github.com:kmc-jp/MinWeb2025-blogging-platform-backend.git
cd MinWeb2025-blogging-platform-backend
cargo run
```

## /api/articlesのAPI仕様

データベース上のArticleデータ
```json
{
    "_id": "ObjectId",
    "author": "記事を作成したユーザー名",
    "title": "記事のタイトル",
    "content": "記事の内容",
    "created_at": "記事が作成された日時",
    "updated_at": "記事が更新された日時"
}
```

### 記事のデータのリストをJSONで取得

`GET /api/articles?skip={skip}&limit={limit}`

`skip`は取得をスキップする記事の数、`limit`は取得する記事の最大数です。
デフォルトでは`skip=0`、`limit=100`となっています。

使用例
```bash
curl http://localhost:3000/api/articles?skip=3&limit=10
```
```http
GET http://localhost:3000/api/articles?limit=5
```

### 指定した記事のデータをJSONで取得

`GET /api/articles/{id}`

`id`は記事のObject ID です。

使用例
```bash
curl http://localhost:3000/api/articles/{id}
```
```http
GET http://localhost:3000/api/articles/{id}
```

上記の`{id}`は実際のObject IDに置き換えてください。

### 新しい記事を作成

`POST /api/articles`

POST通信に用いるJSONの形式
```json
{
    "author": "記事を作成したユーザーの名前",
    "title": "記事のタイトル",
    "content": "記事の内容"
}
```

使用例
```bash
curl -X POST http://localhost:3000/api/articles -H "Content-Type: application/json" -d '{"author": "hoge", "title": "新しい記事のタイトル", "content": "記事の内容"}'
```
```http
POST http://localhost:3000/api/articles
Content-Type: application/json

{
    "author": "hoge",
    "title": "新しい記事のタイトル",
    "content": "記事の内容"
}
```

### 指定した記事を部分的に更新する

`PATCH /api/articles/{id}`

`id`は記事のObject IDです。

PATCH通信に用いるJSONの形式
```json
{
    "title": "記事のタイトル",
    "content": "記事の内容"
}
```

任意のフィールドのみを更新できます。例えば、タイトルだけを更新したい場合は他のフィールドを省略しても問題ありません。

使用例
```bash
curl -X PATCH http://localhost:3000/api/articles/{id} -H "Content-Type: application/json" -d '{"title": "部分的に更新された記事のタイトル", "content": "部分的に更新された記事の内容"}'
```
```http
PATCH http://localhost:3000/api/articles/{id}
Content-Type: application/json

{
    "title": "本文はそのままでタイトルだけを更新"
}
```

### 指定した記事を削除する

`DELETE /api/articles/{id}`

`id`は記事のObject IDです。

使用例
```bash
curl -X DELETE http://localhost:3000/api/articles/{id}
```
```http
DELETE http://localhost:3000/api/articles/{id}
```

### 特定の文字列をタイトルに含むすべての記事のデータをJSONで取得

`GET /api/articles/search?title_q={title_query}`

`title_query`は検索したい文字列です。

使用例
```bash
curl http://localhost:3000/api/articles/search?title_q=Rust
```
```http
GET http://localhost:3000/api/articles/search?title_q=マイクラ
```


### 特定のユーザーが作成したすべての記事を取得

`GET /api/articles/search?author={user_name}`

`user_name`は記事を作成したユーザー名です。

使用例
```bash
curl http://localhost:3000/api/articles/search?author=furakuta
```
```http
GET http://localhost:3000/api/articles/search?author=akkey
```

## /api/usersのAPI仕様

データベース上のUserデータ
```json
{
    "_id": "ObjectId",
    "name": "ユーザー名",　// 一意であることが保証されており、記事のauthorフィールドに使用される　英数字のみからなる　後から変更はできない
    "display_name": "表示名", // ユーザーの表示名　絵文字なども使用可能
    "intro": "自己紹介", // ユーザーの自己紹介
    "email": "メールアドレス", // ユーザーのメールアドレス
    "show_email": true, // ユーザーのメールアドレスを公開するかどうか
    "pw_hash": "ハッシュ化されたパスワード", // ユーザーのパスワードはハッシュ化されて保存されます
    "created_at": "ユーザーが作成された日時"
}
```

### ユーザーのデータのリストをJSONで取得

`GET /api/users?skip={skip}&limit={limit}`

`skip`は取得をスキップするユーザーの数、`limit`は取得するユーザーの最大数です。
デフォルトでは`skip=0`、`limit=100`となっています。

使用例
```bash
curl http://localhost:3000/api/users?skip=3&limit=10
```
```http
GET http://localhost:3000/api/users?limit=5
```

### 指定したユーザーのデータをJSONで取得

`GET /api/users/{user_name}`

`user_name`はユーザー名です。
`email`フィールドは`show_email`が`true`の場合のみ返されます。
`password`フィールドは返されません。

使用例
```bash
curl http://localhost:3000/api/users/wuhu1sland
```
```http
GET http://localhost:3000/api/users/hoge
```

### 新しいユーザーを作成

`POST /api/users`

POST通信に用いるJSONの形式
```json
{
    "name": "ユーザー名", // 一意であることが保証されており、記事のauthorフィールドに使用される　英数字のみからなる　後から変更はできない
    "display_name": "表示名", // ユーザーの表示名　絵文字なども使用可能
    "intro": "自己紹介", // ユーザーの自己紹介
    "email": "メールアドレス", // ユーザーのメールアドレス
    "show_email": true, // ユーザーのメールアドレスを公開するかどうか
    "password": "パスワード" // ユーザーのパスワード
}
```

使用例
```bash
curl -X POST http://localhost:3000/api/users -H "Content-Type: application/json" -d '{"name": "hoge", "display_name": "Hoge User", "intro": "Hello, I am Hoge.", "email": "hoge@gmail.com", "show_email": true, "password": "password123"}'
```
```http
POST http://localhost:3000/api/users
Content-Type: application/json

{
    "name": "hoge",
    "display_name": "Hoge User",
    "intro": "Hello, I am Hoge.",
    "email": "hoge@gmail.com",
    "show_email": true,
    "password": "password123"
}
```

### 指定したユーザーを部分的に更新する

`PATCH /api/users/{user_name}`

`user_name`はユーザー名です。

PATCH通信に用いるJSONの形式
```json
{
    "display_name": "表示名", // ユーザーの表示名　絵文字なども使用可能
    "intro": "自己紹介", // ユーザーの自己紹介
    "email": "メールアドレス", // ユーザーのメールアドレス
    "show_email": true, // ユーザーのメールアドレスを公開するかどうか
    "password": "パスワード" // ユーザーのパスワード
}
```

任意のフィールドのみを更新できます。例えば、表示名だけを更新したい場合は他のフィールドを省略しても問題ありません。

使用例
```bash
curl -X PATCH http://localhost:3000/api/users/{user_name} -H "Content-Type: application/json" -d '{"display_name": "新しい表示名", "intro": "新しい自己紹介"}'
```
```http
PATCH http://localhost:3000/api/users/furakuta
Content-Type: application/json

{
    "display_name": "🫠",
    "intro": "グッバイ物理学実験"
}
```

### 指定したユーザーを削除する
`DELETE /api/users/{user_name}`

`user_name`はユーザー名です。

使用例
```bash
curl -X DELETE http://localhost:3000/api/users/hoge
```
```http
DELETE http://localhost:3000/api/users/hoge
```

