
#![no_std]
use gstd::{errors::Result, msg , prelude::*,ActorId};
use gmeta::{Metadata};
use hashbrown::HashMap;
use io::*;

#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));



static mut STATE:Option<HashMap<ActorId, u128>> = None;



fn state_mut() -> &'static mut HashMap<ActorId,u128> {

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


        // We load the input message
        let payload = msg::load()?;

        
        // We receive an action from the user and update the state. Example:
        if let Action::ExampleAction = payload {

            let currentstate = state_mut();



            // Update your state Example: (ActorId,u128)
            currentstate.insert(msg::source(), 10);
            

            // Generate response message
            msg::reply(Event::ExampleEvent,0)?;

        }

       

    Ok(())
    }



    #[no_mangle]
    extern "C" fn state() {

        // We create a state variable.
        let state: <ContractMetadata as Metadata>::State =
            state_mut().iter().map(|(k, v)| (*k, *v)).collect();
         
        // Generate response message
        msg::reply(state, 0).expect("failed to encode or reply from `state()`");
    }
