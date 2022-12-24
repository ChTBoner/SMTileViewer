/*
 * Copyright (c) 2021 Sylvain "Skarsnik" Colinet
 *
 * This file is part of the usb2snes-cli project.
 * (see https://github.com/usb2snes/usb2snes-cli).
 *
 * usb2snes-cli is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * usb2snes-cli is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with QUsb2Snes.  If not, see <https://www.gnu.org/licenses/>.
 */


use websocket::{ClientBuilder, Message, WebSocketError};
use websocket::sync::stream::TcpStream;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

    #[derive(Display, Debug)]
    #[allow(dead_code)]
    pub enum Command {
        AppVersion,
        Name,
        DeviceList,
        Attach,
        Info,
        Boot,
        Reset,
        Menu,

        List,
        PutFile,
        GetFile,
        Rename,
        Remove,

        GetAddress
    }
    #[derive(Display, Debug)]
    #[allow(dead_code)]
    pub enum Space {
        None,
        SNES,
        CMD
    }

    pub struct Infos {
        pub version : String,
        pub dev_type : String,
        pub game : String,
        pub flags : Vec<String>
    }

    #[derive(Serialize)]
    #[allow(non_snake_case)]
    struct USB2SnesQuery {
        Opcode: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        Space: Option<String>,
        Flags: Vec<String>,
        Operands: Vec<String>
    }
    #[derive(Deserialize)]
    #[allow(non_snake_case)]
    struct USB2SnesResult {
        Results : Vec<String>
    }

    #[derive(PartialEq)]
    pub enum USB2SnesFileType {
        File = 0,
        Dir = 1
    }
    pub struct USB2SnesFileInfo {
        pub name : String,
        pub file_type : USB2SnesFileType
    }
    
    pub struct SyncClient {
        client : websocket::sync::Client<TcpStream>,
        devel : bool
    }
    impl SyncClient {
        pub fn connect() -> Result<SyncClient, WebSocketError>  {
            Ok(SyncClient {
                 client : ClientBuilder::new("ws://localhost:23074")
                .unwrap()
                .connect_insecure()?,
                devel : false
            })
        }
        pub fn connect_with_devel() -> Result<SyncClient, WebSocketError> {
            Ok(SyncClient {
                client : ClientBuilder::new("ws://localhost:23074")
                .unwrap()
                .connect_insecure()?,
                devel : true
            })
        }
        fn send_command(&mut self, command : Command, args : Vec<String>) -> Result<(), WebSocketError> {
            self.send_command_with_space(command, None, args)?;
            Ok(())
        }
        fn send_command_with_space(&mut self, command : Command, space : Option<Space>,  args : Vec<String>) -> Result<(), WebSocketError> {
            if self.devel {
                println!("Send command : {:?}", command);
            }
            let nspace : Option<String> = match space {
                None => None,
                Some(sp) => Some(sp.to_string())
            };
            let query = USB2SnesQuery {
                Opcode : command.to_string(),
                Space : nspace,
                Flags : vec![],
                Operands : args
            };
            let json = serde_json::to_string_pretty(&query).unwrap();
            if self.devel {
                println!("{}", json);
            }
            let message = Message::text(json);
            self.client.send_message(&message)?;
            Ok(())
        }

        fn get_reply(&mut self) -> Result<USB2SnesResult, WebSocketError> {
            let reply = self.client.recv_message()?;
            let mut textreply : String = String::from("");
            match reply
            {
                websocket::OwnedMessage::Text(value) => {textreply = value;}
                _ => {println!("Error getting a reply");}
            };
            if self.devel {
                println!("Reply:");
                println!("{}", textreply);
            }
            return Ok(serde_json::from_str(&textreply).unwrap());
        }
        pub fn set_name(&mut self, name : String) -> Result<(), WebSocketError> {
            self.send_command(Command::Name, vec![name])?;
            Ok(())
        }
        pub fn app_version(&mut self) -> Result<String, WebSocketError> {
            self.send_command(Command::AppVersion, vec![])?;
            let usbreply = self.get_reply()?;
            return Ok(usbreply.Results[0].to_string());
        }
        pub fn list_device(&mut self) -> Result<Vec<String>, WebSocketError> {
            self.send_command(Command::DeviceList, vec![])?;
            let usbreply = self.get_reply()?;
            return Ok(usbreply.Results);
        }
        pub fn attach(&mut self, device : &String) -> Result<(), WebSocketError> {
            self.send_command(Command::Attach, vec![device.to_string()])?;
            Ok(())
        }

        pub fn info(&mut self) -> Result<Infos, WebSocketError> {
            self.send_command(Command::Info, vec![])?;
            let usbreply = self.get_reply()?;
            let info : Vec<String> =  usbreply.Results;
            Ok(Infos { version: info[0].clone(), dev_type: info[1].clone(), game: info[2].clone(), flags: (info[3..].to_vec()) })
        }
        pub fn reset(&mut self) -> Result<(), WebSocketError> {
            self.send_command(Command::Reset, vec![])?;
            Ok(())
        }
        pub fn menu(&mut self) -> Result<(), WebSocketError> {
            self.send_command(Command::Menu, vec![])?;
            Ok(())
        }
        pub fn boot(&mut self, toboot : &String) -> Result<(), WebSocketError> {
            self.send_command(Command::Boot, vec![toboot.clone()])?;
            Ok(())
        }

        pub fn ls(&mut self, path : &String) -> Result<Vec<USB2SnesFileInfo>, WebSocketError> {
            self.send_command(Command::List,vec![path.to_string()])?;
            let usbreply = self.get_reply()?;
            let vec_info = usbreply.Results;
            let mut toret:Vec<USB2SnesFileInfo> = vec![];
            let mut i =  0;
            while i < vec_info.len() {
                let info : USB2SnesFileInfo = USB2SnesFileInfo {
                    file_type : if vec_info[i] == "1" {USB2SnesFileType::File} else {USB2SnesFileType::Dir},
                    name : vec_info[i + 1].to_string()
                };
                toret.push(info);
                i += 2;
            }
            return Ok(toret);
        }
        pub fn send_file(&mut self, path : &String, data : Vec<u8>) -> Result<(), WebSocketError> {
            self.send_command(Command::PutFile, vec![path.to_string(), format!("{:x}", data.len())])?;
            let mut start = 0;
            let mut stop = 1024;
            while stop <= data.len() {
                self.client.send_message(&Message::binary(&data[start..stop]))?;
                start += 1024;
                stop += 1024;
                if stop > data.len() {
                    stop = data.len();
                }
            }
            Ok(())
        }
        pub fn get_file(&mut self, path : &String) -> Result<Vec<u8>, WebSocketError> {
            self.send_command(Command::GetFile, vec![path.clone()])?;
            let string_hex = self.get_reply()?.Results[0].to_string();
            let size = usize::from_str_radix(&string_hex.to_string(), 16).unwrap();
            let mut data : Vec<u8> = vec![];
            data.reserve(size);
            loop {
                let reply = self.client.recv_message().unwrap();
                match reply {
                    websocket::OwnedMessage::Binary(msgdata) => {data.extend(&msgdata);}
                    _ => {println!("Error getting a reply");}
                }
                if data.len() == size {
                    break;
                }
            }
            Ok(data)
        }
        pub fn get_address(&mut self, address : u32, size : usize) -> Result<Vec<u8>, WebSocketError> {
            self.send_command_with_space(Command::GetAddress, Some(Space::SNES), vec![format!("{:x}", address), format!("{:x}", size)])?;
            let mut data : Vec<u8> = vec![];
            data.reserve(size);
            loop {
                let reply = self.client.recv_message()?;
                match reply {
                    websocket::OwnedMessage::Binary(msgdata) => {data.extend(&msgdata);}
                    _ => {println!("Error getting a reply");}
                }
                if data.len() == size {
                    break;
                }
            }
            Ok(data)
        }
        pub fn get_multi_address(&mut self, addresses : Vec<u32>, sizes : Vec<u8>) -> Result<Vec<u8>, WebSocketError> {
            let mut v_arg : Vec<String> = vec![];
            v_arg.reserve(addresses.len() * 2);
            let mut cpt = 0;
            let mut total_size : u8 = 0;
            while cpt < addresses.len() {
                v_arg.push(format!("{:x}", addresses[cpt]));
                v_arg.push(format!("{:x}", sizes[cpt]));
                total_size += sizes[cpt];
                cpt += 1
            }
            self.send_command_with_space(Command::GetAddress, Some(Space::SNES), v_arg)?;
            let mut data : Vec<u8> = vec![];
            data.reserve(total_size as usize);
            loop {
                let reply = self.client.recv_message()?;
                match reply {
                    websocket::OwnedMessage::Binary(msgdata) => {data.extend(&msgdata);}
                    _ => {println!("Error getting a reply");}
                }
                if data.len() == total_size as usize {
                    break;
                }
            }
            Ok(data)
        }
    }
