
#![no_std]
use gstd::{errors::Result, msg , prelude::*,ActorId};
use gmeta::{Metadata};
use hashbrown::HashMap;
use io::*;

#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));



static mut STATE:Option<HashMap<ActorId, String>> = None;



fn state_mut() -> &'static mut HashMap<ActorId,String> {

    let state = unsafe { STATE.as_mut()};

    unsafe { state.unwrap_unchecked() }


}


#[no_mangle]
extern "C" fn init () {

   unsafe { STATE = Some(HashMap::new())}


}

#[no_mangle]
extern "C" fn handle(){


    handle_state().expect("Execution Error")


}

    

fn handle_state() -> Result<()> {

        let payload = msg::load()?;

        if let TrafficLight::Green = payload {

            let currentstate = state_mut();
            currentstate.insert(msg::source(), "Green".to_string());
            msg::reply(TrafficLight::Green,0)?;

        }

        if let TrafficLight::Yellow = payload {

            let currentstate = state_mut();
            currentstate.insert(msg::source(), "Yellow".to_string());
            msg::reply(TrafficLight::Yellow,0)?;

        }

        if let TrafficLight::Red = payload {

            let currentstate = state_mut();
            currentstate.insert(msg::source(), "Red".to_string());
            msg::reply(TrafficLight::Red,0)?;

        }

    Ok(())
    }



    #[no_mangle]
    extern "C" fn state() {
        let state: <ContractMetadata as Metadata>::State =
            state_mut().iter().map(|(k, v)| (*k, v.clone())).collect();
    
        msg::reply(state, 0).expect("failed to encode or reply from `state()`");
    }
