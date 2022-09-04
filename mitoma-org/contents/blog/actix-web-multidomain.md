# actix-web で multidomain を実装する

[ブログづくり](create_blog) の中で Cloud Run + Rust + actix-web で mitoma.org を作ったのだけど、ふとこれだけ actix-web が軽ければ一つの Cloud Run で複数のサイトを作れるようにしてもいいなと思った。

どんなやり方ができるかいろいろ試してみたが、最終的には [actix_web::guard::Host][actix-guard-host] を使うだけでよかった。

試しに [https://hello.mitoma.org/](https://hello.mitoma.org/) を立ててみた。これは [https://mitoma.org/](https://mitoma.org/) と同じ Cloud Run インスタンスで動いている。

ドキュメントにも書かれてるけど、これは古き良き Virtual Host だね。

[actix-guard-host]: https://docs.rs/actix-web/latest/actix_web/guard/fn.Host.html#examples
