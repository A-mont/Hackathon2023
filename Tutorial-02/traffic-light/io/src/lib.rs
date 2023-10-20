
#![no_std]
use gstd::{ prelude::*, ActorId };
use gmeta::{InOut,Metadata};


#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum ActionTrafficLight {
     Green,
     Yellow,
     Red
}

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum EventTrafficLight {
     Green,
     Yellow,
     Red
}

pub struct ContractMetadata;


impl Metadata for ContractMetadata{
     type Init = ();
     type Handle = InOut<ActionTrafficLight,EventTrafficLight>;
     type Others = ();
     type Reply=();
     type Signal = ();
     type State = Vec<(ActorId, String)>;

}