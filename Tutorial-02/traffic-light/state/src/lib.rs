
#![no_std]

use io::*;
use gmeta::{ Metadata, metawasm};
use gstd::{ ActorId, prelude::*};


#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

#[metawasm]
pub mod metafns {
    pub type State = <ContractMetadata as Metadata>::State;

     pub fn finalstates(state: State, actor: ActorId) -> Vec<&Str> {
        state.into_iter().map(|(_, state)| state).collect()
    }
}