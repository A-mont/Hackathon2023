
#![no_std]

use io::*;
use gmeta::{ Metadata, metawasm};
use gstd::{ ActorId, prelude::*};


#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

#[metawasm]
pub mod metafns {
    pub type State = Escrow;

    pub fn seller(state: State) -> ActorId {
        state.seller
    }

    pub fn buyer(state: State) -> ActorId {
        state.buyer
    }

    pub fn escrow_state(state: State) -> EscrowState {
        state.state
    }
}