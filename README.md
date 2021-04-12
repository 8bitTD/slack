# slack
Send a message to slack
Works in windows10 environment
```
//Cargo.toml
[dependencies]
slack = { git = "https://github.com/8bitTD/slack" }
```
```
//main.rs
use slack;
fn main() {
    let tkn: &str = "xxxx-xxxxxxxxxxxx-xxxxxxxxxxxx-xxxxxxxxxxxx-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";
    let cnl: &str = "channel_name";
    let usr: &str = "user_name";
    let msg: &str = "test_send_message!";
    slack::send_slack(tkn, cnl, usr, msg).unwrap();
}
```
