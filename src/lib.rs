use serde::{Deserialize, Serialize};

use std::io::prelude::*;
use std::{fs, fs::File};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub device_name: String,
    serial_port_name: String,
    baud_rate: u32,
    pub ip_addr: String,
    pub mask: String,
    pub gateway: String,
    pub broker_host: String,
    pub broker_port: u16,
    topic: String,
}

impl Config {
    pub fn new(
        device_name: String,
        serial_port_name: String,
        baud_rate: u32,
        ip_addr: String,
        mask: String,
        gateway: String,
        broker_host: String,
        broker_port: u16,
        topic: String,
    ) -> Config {
        Config {
            device_name,
            serial_port_name,
            baud_rate,
            ip_addr,
            mask,
            gateway,
            broker_host,
            broker_port,
            topic,
        }
    }

    pub fn get_serial_port_name(&self) -> &String {
        &self.serial_port_name
    }

    pub fn get_baud_rate(&self) -> u32 {
        self.baud_rate
    }

    pub fn get_topic(&self) -> &String {
        &self.topic
    }

    pub fn init_config_val() -> Config {
        Config::new(
            "Smart-Power".to_string(),
            "ttyS0".to_string(),
            115200,
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "localhost".to_string(),
            1883,
            "smart-power-box/status".to_string(),
        )
    }
}

pub fn init_config() -> Result<Config, String> {
    match File::create("config.json") {
        Ok(mut file) => {
            let config = Config::init_config_val();
            match serde_json::to_string(&config) {
                Ok(content) => match file.write_all(&content.into_bytes()) {
                    Ok(()) => Ok(config),
                    Err(_) => Err("write config file failed".to_string()),
                },
                Err(_) => Err("serialize failed".to_string()),
            }
        }
        Err(_) => Err("create config file failed".to_string()),
    }
}

pub fn read_config(path: &str) -> Result<Config, String> {
    match fs::read_to_string(path) {
        Ok(content) => match serde_json::from_str(&content) {
            Ok(conf) => {
                let config: Config = conf;
                Ok(config)
            }
            Err(_) => Err("json parse failed".to_string()),
        },
        Err(_) => Err("read config file failed".to_string()),
    }
}

pub fn save_config(path: &str, config: Config) -> Result<(), String> {
    match File::create(path) {
        Ok(mut file) => match serde_json::to_string(&config) {
            Ok(content) => match file.write_all(&content.into_bytes()) {
                Ok(()) => Ok(()),
                Err(_) => Err("write config file failed".to_string()),
            },
            Err(_) => Err("serialize failed".to_string()),
        },
        Err(_) => Err("save config file failed".to_string()),
    }
}
