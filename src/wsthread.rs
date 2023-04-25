use std::sync::{Mutex, Arc};
use tungstenite::error::Error;
use std::time::Duration;
use crate::data::{self, SharedData};
use crate::data::Usb2SnesError;
use rusb2snes;
use rusb2snes::SyncClient;


pub fn wsthread(data : Arc<Mutex<data::SharedData>>) {
    'main : loop {
        let mut usb2snes : SyncClient;
        let co_return = try_to_connect();
        match co_return {
            Ok(plop) => usb2snes = plop,
            Err(_err) => {
                std::thread::sleep(Duration::new(2, 0));
                let mut mutex = data.lock().unwrap();
                (*mutex).usb2snes_error = Usb2SnesError::CantConnect;
                continue;
            }
        }
        '_attach : loop {
            fn list_device(usb2snes : &mut SyncClient) -> Result<i32, Error> {
                let devices = usb2snes.list_device()?;
                if devices.len() != 0 {
                    usb2snes.attach(&devices[0])?;
                } else {
                    return Ok(0);
                }
                Ok(1)
            }
            match list_device(&mut usb2snes) {
                Err(_err) => {
                    let mut mutex = data.lock().unwrap();
                    (*mutex).usb2snes_error = Usb2SnesError::CantAttach;
                    continue 'main
                },
                Ok(nb_device) => {
                    if nb_device == 1 {
                        break;
                    } else {
                        let mut mutex = data.lock().unwrap();
                        (*mutex).usb2snes_error = Usb2SnesError::NoDevice;
                        std::thread::sleep_ms(1000);
                    }
                }
            }
        }
        // Let do stuff finally, just kidding, we need a game
        loop {
            let infos = usb2snes.info();
            match infos {
                Err(_err) => {continue 'main}
                Ok(info) => {
                    if info.game == "/boot/menu.bin" || info.game == "/boot/m3nu.bin" {
                        let mut mutex = data.lock().unwrap();
                        (*mutex).usb2snes_error = Usb2SnesError::NoGame;
                        std::thread::sleep_ms(2000);
                        continue;
                    } else {
                        break;
                    }
                }
            }
        }
        {
            let mut mutex = data.lock().unwrap();
            (*mutex).usb2snes_ready = true;
        }
        loop {
            match actually_getting_data(&mut usb2snes, &data) {
                Err(_err) => {continue 'main},
                Ok(()) => {}
            }
        }
    }
}

// Thing to read each frame
const A_GAME_STATE : u32 = 0x7E0998;
const A_MAP_ID : u32 = 0x7E079B;
const A_SAMUS_X : u32 = 0x7E0AF6;
const A_SAMUS_Y : u32 = 0x7E0AFA;
const A_RADIUS_X : u32 = 0x7E0AFE;
const A_RADIUS_Y : u32 = 0x7E0B00;
const A_WIDTH : u32 = 0x7E07A5;
const A_DOOR_STUFF : u32 = 0x7E07B5;
// var BTS = 0x7F0000 + ((0x6402 + a) % 0x10000)
// var BTSvalue = memory.readUnsignedByte(BTS);
// var Clip = 0x7F0000 + ((0x0002 + a * 2) % 0x10000)
// var ClipValue = memory.readUnsignedWord(Clip)


fn actually_getting_data(usb2snes : &mut SyncClient, data : &Mutex<SharedData>) -> Result<(), Error> {
    static mut old_map_id : u16 = 0;
    let bytes = get_base_wram_value(usb2snes)?;
    let samus = sdl2::rect::Point::new(get_uword(bytes[1], bytes[2]).into(),
                                              get_uword(bytes[3], bytes[4]).into());
    let mut camera = sdl2::rect::Point::new((samus.x - 256) & 0xFFFF, (samus.y - 224) & 0xFFFF);
    if camera.x >= 10000 {
        camera.x = camera.x - 65535
    }
    if camera.y >= 10000 {
        camera.y = camera.y - 65535
    }
    let map_id = bytes[0];
    let game_state = bytes[11];
    let width = get_uword(bytes[9], bytes[10]);
    /*let a : i32 = ((camera.x + x * 16) & 0xFFFF) / 16 + ((((camera.y + y * 16) & 0xFFF) / 16) * (width as i32) & 0xFFFF);
    let bts : usize = (0x6402 as usize + a as usize) % 0x10000;
    let plop = usb2snes.get_address(bts as u32, 1)?;*/
    /*if (map_id == 0) {
        usb2snes.get_address(0x8F0000, size)
    }*/
    unsafe {
    if map_id as u16 != old_map_id {
        if (game_state == 0x08) {
            let mapinfos = usb2snes.get_address(0xF60000, 0x10000)?;
            let mut mutex =  data.lock().unwrap();
            (*mutex).map_data = mapinfos;
            old_map_id = map_id as u16;
        } else {
            return Ok(())
        }
    }
    }    //println!("Before lock");
    let mut mutex =  data.lock().unwrap();
    //println!("{}, {}", samus.x, samus.y);
    (*mutex).door_stuff = get_uword(bytes[12], bytes[13]);
    (*mutex).samus_pos = samus;
    (*mutex).camera = camera;
    (*mutex).width = width;
    (*mutex).radius = sdl2::rect::Point::new(get_uword(bytes[5], bytes[6]).into(), get_uword(bytes[7], bytes[8]).into());
    //(*mutex).bts_byte = plop[0];
    Ok(())
}

fn get_base_wram_value(usb2snes : &mut SyncClient) -> Result<Vec<u8>, Error> {
    let mut address : Vec<u32> = vec![0; 8];
    let mut sizes : Vec<usize> = vec![2;8];
    address[0] = A_MAP_ID - 0x7E0000 + 0xF50000;
    address[1] = A_SAMUS_X - 0x7E0000 + 0xF50000;
    address[2] = A_SAMUS_Y - 0x7E0000 + 0xF50000;
    address[3] = A_RADIUS_X - 0x7E0000 + 0xF50000;
    address[4] = A_RADIUS_Y - 0x7E0000 + 0xF50000;
    address[5] = A_WIDTH - 0x7E0000 + 0xF50000;
    address[6] = A_GAME_STATE - 0x7E0000 + 0xF50000;
    address[7] = A_DOOR_STUFF - 0x7E0000 + 0xF50000;
    sizes[0] = 1;
    sizes[1] = 2;
    sizes[2] = 2;
    sizes[3] = 2;
    sizes[4] = 2;
    sizes[5] = 2;
    sizes[6] = 1;
    sizes[7] = 2;
    usb2snes.get_multi_address_as_u8(address, sizes)
}

// This is dumb, but << give me overflow error
fn get_uword(byte1 : u8, byte2 : u8) -> u16 {
    //println!("get_uword: {:x} {:x}", byte1, byte2);
    (u16::from(byte2) & 0x00FF) * 256 + u16::from(byte1)
}

fn _get_sword(byte1 : u8, byte2 : u8) -> i16 {
    (byte2 as i16) << 8 + (byte1 as i16)
}

fn try_to_connect() -> Result<SyncClient, Error> {
    let mut usb2snes = SyncClient::connect()?;
    usb2snes.set_name(String::from("SM TileViewer"))?;
    Ok(usb2snes)
}
