#[macro_use] extern crate magic_crypt;
extern crate base64;
extern crate regex;
use magic_crypt::MagicCryptTrait;
use regex::Regex;

fn encode(name:&str, password:&str, data:&str) -> String {

    let datatoencode = format!("{},{}", name, data);
    let mcrypt = new_magic_crypt!(password, 256);
    let encrypted_string = mcrypt.encrypt_str_to_base64(datatoencode);
    return encrypted_string;
}

fn decode(password:&str, data:String) -> (bool, String, String){

    let mcrypt = new_magic_crypt!(password, 256);
    let decrypted_string = mcrypt.decrypt_base64_to_string(&data);
    match decrypted_string {
        Ok(data) => {
            let re = Regex::new("^([^,]*),(.*)").unwrap();
            let name = re.captures(&data).unwrap();
            let datatosend = (true, name[1].to_string(), name[2].to_string());
            return datatosend;
        },
        Err(msg) => {
            return(false, "".to_string(), "".to_string())
        }
    }
}

fn main() {

    let encodeddata:String = encode("test","test","test");
    decode("test", encodeddata);
}