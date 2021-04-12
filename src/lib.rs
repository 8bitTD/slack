use urlencoding::encode;
pub fn send_slack(tkn:&str, cnl:&str, usr:&str, msg:&str) -> std::io::Result<()> {
    let mut cmd: String = "curl -XPOST -d token=".to_string();
    cmd = cmd + &tkn;
    cmd = cmd + " -d channel=";
    cmd = cmd + &cnl + " -d text=";
    cmd = cmd + &encode(&msg) + " -d username=" + &usr + " -d icon_url=icon_emoji ";
    cmd = cmd + "https://slack.com/api/chat.postMessage";
    let res = std::process::Command::new("cmd.exe").args(&["/C", &cmd]).output().expect("failed to execute process");
    Ok(())
}