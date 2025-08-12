# MinWeb2025-blogging-platform-backend

## /api/articlesのAPI仕様

データベース上のArticleデータ
```json
{
    "_id": "ObjectId",
    "author": "記事を作成したユーザー名",
    "title": "記事のタイトル",
    "content": "記事の内容",
    "created_at": "記事が作成された日時",
    "updated_at": "記事が更新された日時",
    "tags": ["タグ1", "タグ2"] // 記事に関連するタグのIDのリスト
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
    "content": "記事の内容",
    "tags": ["タグ1", "タグ2"] // 記事に関連するタグのリスト
}
```

使用例
```bash
curl -X POST http://localhost:3000/api/articles -H "Content-Type: application/json" -d '{"author": "hoge", "title": "新しい記事のタイトル", "content": "記事の内容", "tags": ["tag1", "tag2"]}'
```
```http
POST http://localhost:3000/api/articles
Content-Type: application/json

{
    "author": "hoge",
    "title": "新しい記事のタイトル",
    "content": "記事の内容",
    "tags": ["tag1", "tag2"]
}
```

### 指定した記事を部分的に更新する

`PATCH /api/articles/{id}`

`id`は記事のObject IDです。

PATCH通信に用いるJSONの形式
```json
{
    "title": "記事のタイトル",
    "content": "記事の内容",
    "tags": ["タグ1", "タグ2"] // 記事に関連するタグのリスト
}
```

任意のフィールドのみを更新できます。例えば、タイトルだけを更新したい場合は他のフィールドを省略しても問題ありません。

使用例
```bash
curl -X PATCH http://localhost:3000/api/articles/{id} -H "Content-Type: application/json" -d '{"title": "部分的に更新された記事のタイトル", "content": "部分的に更新された記事の内容", "tags": ["tag1", "tag2"]}'
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

### 特定のタグの組み合わせを含むすべての記事のデータをJSONで取得
`GET /api/articles/search?tags={tag1},{tag2}`
`tag1`と`tag2`は検索したいタグの名前です。複数のタグをカンマで区切って指定します。
使用例
```bash
curl http://localhost:3000/api/articles/search?tags=競プロ,Rust
```
```
```http
GET http://localhost:3000/api/articles/search?tags=ML
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

### 記事本文に特定の文字列を含むすべての記事を取得
`GET /api/articles/search?content_q={content_query}`
`content_query`は検索したい文字列です。
使用例
```bash
curl http://localhost:3000/api/articles/search?content_q=Rust
```
```http
GET http://localhost:3000/api/articles/search?content_q=プログラミング
```

### 複数の条件を組み合わせて検索
`GET /api/articles/search?author={user_name}&title_q={title_query}&content_q={content_query}&tags={tag1},{tag2}`
上記の`/api/articles/search`は、記事のタイトル、本文、タグ、著者名に対して検索を行います。複数のクエリパラメータを組み合わせて使用することも可能です。すべての条件に一致する記事が返されます。
使用例
```bash
curl http://localhost:3000/api/articles/search?author=furakuta&tags=rust
```
```http
GET http://localhost:3000/api/articles/search?author=akkey&tags=構文解析
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

## /api/tagsのAPI仕様
データベース上のTagデータ
```json
{
    "_id": "ObjectId",
    "name": "タグ名", // タグの名前
    "created_at": "タグが作成された日時",
    "num_articles": 123 // タグが付けられた記事の数
}
```

### タグのデータのリストをJSONで取得
`GET /api/tags?skip={skip}&limit={limit}`
`skip`は取得をスキップするタグの数、`limit`は取得するタグの最大数です。
デフォルトでは`skip=0`、`limit=100`となっています。
使用例
```bash
curl http://localhost:3000/api/tags?skip=3&limit=10
```
```http
GET http://localhost:3000/api/tags?limit=5
```

### 指定したタグのデータをJSONで取得
`GET /api/tags/{tag_name}`
`tag_name`はタグの名前です。

使用例
```bash
curl http://localhost:3000/api/tags/rust
```
```http
GET http://localhost:3000/api/tags/競プロ
```

### 新しいタグを作成
`POST /api/tags`
POST通信に用いるJSONの形式
```json
{
    "name": "タグ名" // タグの名前
}
```
使用例
```bash
curl -X POST http://localhost:3000/api/tags -H "Content-Type: application/json" -d '{"name": "新しいタグ"}'
```
```http
POST http://localhost:3000/api/tags
Content-Type: application/json

{
    "name": "新しいタグ"
}
```
