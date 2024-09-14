use warp::Filter;
use warp::http::Method;
use serde::{Deserialize, Serialize};
use tfhe::{ConfigBuilder, generate_keys, set_server_key, FheUint8};
use tfhe::prelude::*;

#[derive(Deserialize)]
struct Numbers {
    num1: u8,
    num2: u8,
}

#[derive(Serialize)]
struct Result {
    result: u8,
}

#[tokio::main]
async fn main() {
    // 足し算処理のルートを定義
    let add = warp::post()
        .and(warp::path("add"))
        .and(warp::body::json())
        .map(|nums: Numbers| {
            let config = ConfigBuilder::default().build();

            let (client_key, server_key) = generate_keys(config);

            let clear_a = nums.num1;
            let clear_b = nums.num2;

            let a = FheUint8::encrypt(clear_a, &client_key);
            let b = FheUint8::encrypt(clear_b, &client_key);

            set_server_key(server_key);
            let result = a + b;

            let decrypted_result: u8 = result.decrypt(&client_key);

            warp::reply::json(&Result {
                result: decrypted_result,
            })
        });

    // CORS 設定を追加
    let cors = warp::cors()
        .allow_any_origin()  // すべてのオリジンを許可
        .allow_methods(&[Method::POST])  // POST メソッドのみ許可
        .allow_headers(vec!["Content-Type"]);  // Content-Type ヘッダーを許可

    // サーバーの実行
    warp::serve(add.with(cors))
        .run(([127, 0, 0, 1], 8000))
        .await;
}