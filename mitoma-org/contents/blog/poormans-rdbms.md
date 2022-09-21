# 貧者の RDBMS

今はただのメモ帳

## やりたいこと

Cloud Run で PostgreSQL を立ててお安い開発用 RDBMS を準備する。

そのためには Cloud Run の以下の制約をクリアする必要がある

- http port しか外部公開できない

## 事前調査

### httptunnel で Proxy をするテスト

hts --forward-port localhost:5432 1234 -w
htc --forward-port 15432 localhost:8888 1234 -w

### http-tunnel で Proxy をするテスト

http-tunnel は以下の実装を利用
https://github.com/xnuter/http-tunnel

http-tunnel --bind localhost:8080 http

送信側は proxytunnel で以下のようにアクセスする。

proxytunnel --standalone=15432 -p localhost:8080 -d localhost:5432

