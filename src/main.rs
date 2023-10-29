use reqwest::blocking::Client; //Synchronous request
use reqwest::Error; //To hanfle request error coming from reqwest

fn main() -> Result<(), Error> {
    let client = Client::new(); //Iniitlize client
    let user = "testuser".to_string();
    // let passwd: Option<String> = None; //Setting it to none
    let passwd: Option<String> = Some("my_actual_password".to_string()); // Setting it to Some
    let res = client
        .get("http://httpbin.org/get") //Maing request
        .basic_auth(user, passwd)
        .send();

    println!("Got response {:?}", res);
    Ok(())
}
