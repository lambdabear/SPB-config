use spb_config::*;

fn main() {
    match read_config("./config.json") {
        Ok(mut config) => {
            println!("{:?}", config);
            config.ip_addr = "10.0.0.111".to_string();
            config.mask = "255.255.255.0".to_string();
            config.gateway = "10.0.0.1".to_string();
            config.device_name = "农行人民街支行".to_string();
            save_config("./config.json", config).unwrap();
        }
        Err(_) => match init_config() {
            Ok(()) => println!("init config file success"),
            Err(e) => println!("{}", e),
        },
    }
}
