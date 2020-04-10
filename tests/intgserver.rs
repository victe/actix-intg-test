use std::process::Command;

use actix_web::client::Client;
use std::thread::sleep;

#[actix_rt::test]
async fn main() {
    let program = "./target/debug/actix-intg-test";
    // let mut axs = Command::new(program);

    let s1 = "127.0.0.1:8080";
    let mut axs_1 = Command::new(program).arg(s1).spawn().expect("failed to execute process");
    sleep(std::time::Duration::new(1,0));
    let s2 = "127.0.0.1:8081";
    let mut axs_2 = Command::new(program).arg(s2).spawn().expect("failed to execute process");
    sleep(std::time::Duration::new(1,0));

    let mut client = Client::default();

    // Create request builder and send request
    let pro = String::from("http://");
    let url_1 = pro.clone() + s1 + "/1/a";
    let response1 = client.get(url_1)
        .header("User-Agent", "Actix-web")
        .send().await;                      // <- Send http request
    let url_2 = pro.clone() + s2 + "/2/b";
    let response2 = client.get(url_2)
        .header("User-Agent", "Actix-web")
        .send().await;                      // <- Send http request

    // println!("Response1: {:?}", response1.unwrap().body().await);
    // println!("Response2: {:?}", response2.unwrap().body().await);
    assert_eq!("Hello a! id:1", response1.unwrap().body().await.expect("Cannot get body of response 1"));
    assert_eq!("Hello b! id:2", response2.unwrap().body().await.expect("Cannot get body of response 1"));
    axs_1.kill().expect("Cannot kill axs 1");
    axs_2.kill().expect("Cannot kill axs 2");
}
