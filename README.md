# MinWeb2025-blogging-platform-backend

## セットアップ
データベースをやっている人はMongoDBのセットアップ方法の追記をお願いします
```
git clone git@github.com:kmc-jp/MinWeb2025-blogging-platform-backend.git
cd MinWeb2025-blogging-platform-backend
cargo run
```

## 提供されているバックエンドAPI

### 最大１００件の記事のデータをJSONで取得

エンドポイント`/api/articles`

HTTP GET通信で取得します。
このエンドポイントは、最新の記事から最大100件の記事データを取得します。

curlコマンドによる使用例
```
curl http://localhost:3000/api/articles
```

### 指定した範囲の記事のデータをJSONで取得

エンドポイント`/api/articles`

POST通信に用いるJSONの形式
```
{
    "from": 開始位置,
    "max": 取得する最大件数
}
```

curlコマンドによる使用例
```
curl -X POST http://localhost:3000/api/articles -H "Content-Type: application/json" -d '{"from": 0, "max": 10}'
```

### 特定の文字列をタイトルに含むすべての記事のデータをJSONで取得

エンドポイント`/api/articles/search`

POST通信に用いるJSONの形式
```
{
    "title_query": "検索したい文字列"
}
```

curlコマンドによる使用例
```
curl -X POST http://localhost:3000/api/articles/search -H "Content-Type: application/json" -d '{"title_query": "検索したい文字列"}'
```

### 新しい記事を作成

エンドポイント`/api/articles/{user}/new`

`user`は記事を作成するユーザーの名前です。

POST通信に用いるJSONの形式
```
{
    "title": "記事のタイトル",
    "content": "記事の内容"
}
```

curlコマンドによる使用例
```
curl -X POST http://localhost:3000/api/articles/hoge/new -H "Content-Type: application/json" -d '{"title": "記事のタイトル", "content": "記事の内容"}'
```

### 特定のユーザーが作成したすべての記事を取得

エンドポイント`/api/articles/{user}`

HTTP GET通信で取得します。

`user`は記事を作成したユーザーの名前です。

curlコマンドによる使用例
```
curl http://localhost:3000/api/articles/hoge
```

### 特定の記事を取得

エンドポイント`/api/articles/{user}/{id}`

HTTP GET通信で取得します。

`user`は記事を作成したユーザーの名前、`id`は記事のUUIDです。

curlコマンドによる使用例
```
curl http://localhost:3000/api/articles/hoge/{id}
```

上記の`{id}`は実際のUUIDに置き換えてください。

### 特定の記事を更新

エンドポイント`/api/articles/{user}/update/{id}`

`user`は記事を作成したユーザーの名前、`id`は記事のUUIDです。

POST通信に用いるJSONの形式
```
{
    "title": "記事のタイトル",
    "content": "記事の内容"
}
```

どちらかのフィールドを与えないことで、更新しないことも可能です。

curlコマンドによる使用例
```
curl -X POST http://localhost:3000/api/articles/hoge/update/{id} -H "Content-Type: application/json" -d '{"title": "新しい記事のタイトル"}'
```

上記の`{id}`は実際のUUIDに置き換えてください。








aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
