# MinWeb2025-blogging-platform-backend

## セットアップ
データベースをやっている人はMongoDBのセットアップ方法の追記をお願いします
```
git clone git@github.com:kmc-jp/MinWeb2025-blogging-platform-backend.git
cd MinWeb2025-blogging-platform-backend
cargo run
```

## 提供されているバックエンドAPI

### すべての記事のデータをJSONで取得

エンドポイント`/`

curlコマンドによる使用例
```
curl http://localhost:3000/
```

### 特定の文字列をタイトルに含むすべての記事のデータをJSONで取得

エンドポイント`/`

POST通信に用いるJSONの形式
```
{
    "title_query": "検索したい文字列"
}
```

curlコマンドによる使用例
```
curl -X POST http://localhost:3000/ -H "Content-Type: application/json" -d '{"title_query": "検索したい文字列"}'
```

### 新しい記事を作成

エンドポイント`/{user}/create-article/`

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
curl -X POST http://localhost:3000/hoge/create-article/ -H "Content-Type: application/json" -d '{"title": "記事のタイトル", "content": "記事の内容"}'
```

### 特定のユーザーが作成したすべての記事を取得

エンドポイント`/{user}/articles/`

`user`は記事を作成したユーザーの名前です。

curlコマンドによる使用例
```
curl http://localhost:3000/hoge/articles/
```

### 特定の記事を取得

エンドポイント`/{user}/articles/{id}`

`user`は記事を作成したユーザーの名前、`id`は記事のUUIDです。

curlコマンドによる使用例
```
curl http://localhost:3000/hoge/articles/{id}
```

上記の`{id}`は実際のUUIDに置き換えてください。

### 特定の記事を更新

エンドポイント`/{user}/update-article/{id}`

`user`は記事を作成したユーザーの名前、`id`は記事のUUIDです。

POST通信に用いるJSONの形式
```
{
    "title": "記事のタイトル",
    "content": "記事の内容"
}
```

どちらかのフィールドを空文字列にすることで、更新しないことも可能です。

curlコマンドによる使用例
```
curl -X POST http://localhost:3000/hoge/update-article/{id} -H "Content-Type: application/json" -d '{"title": "新しい記事のタイトル", "content": "新しい記事の内容"}'
```

上記の`{id}`は実際のUUIDに置き換えてください。