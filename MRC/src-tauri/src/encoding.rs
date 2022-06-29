extern crate magic_crypt;
extern crate base64;
extern crate regex;
use magic_crypt::MagicCryptTrait;
use regex::Regex;

pub fn group_encode(groupname:String, encrypted_string:String) -> String {

    let encodedstring = format!("{},{}", groupname, encrypted_string);
    return encodedstring;
}

pub fn get_group(encodedstring:String) -> (String, String) {

    let re = Regex::new("^([^,]*),(.*)").unwrap();
    let name = re.captures(&encodedstring).unwrap();
    let groupdata = (name[1].to_string(), name[2].to_string());
    // println!("{:#?}", groupdata);
    return groupdata;
}
// encode data with name as a string seperated by a comma.
// maybe add an extra value for header for different types of information.
pub fn encode(name:&str, password:&str, data:&str) -> String {

    let datatoencode = format!("{},{}", name, data);
    let mcrypt = magic_crypt::new_magic_crypt!(password, 256);
    let encrypted_string = mcrypt.encrypt_str_to_base64(datatoencode);
    return encrypted_string;
}

// decodes the data with a given password.
#[allow(dead_code)]
pub fn decode(password:&str, data:String) -> (bool, String, String){
    let mcrypt = magic_crypt::new_magic_crypt!(password, 256);
    let decrypted_string = mcrypt.decrypt_base64_to_string(&data);
    match decrypted_string {
        Ok(data) => {
            let re = Regex::new("^([^,]*),(.*)").unwrap();
            let name = re.captures(&data).unwrap();
            let datatosend = (true, name[1].to_string(), name[2].to_string());
            return datatosend;
        },
        Err(_msg) => {
            return(false, "".to_string(), "".to_string())
        }
    }
}

// fn main() {

//     let encodeddata:String = encode("test","test","test");
//     // decode("test", encodeddata);
//     let groupencoded:String = group_encode(encodeddata, "group1".to_string());
//     find_group(groupencoded);
// }
