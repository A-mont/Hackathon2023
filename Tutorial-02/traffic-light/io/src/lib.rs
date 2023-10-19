
#![no_std]
use gstd::{ prelude::*, ActorId };
use gmeta::{InOut,Metadata};


#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum TrafficLight {
     Green,
     Yellow,
     Red
}

pub struct ContractMetadata;


impl Metadata for ContractMetadata{
     type Init = ();
     type Handle = InOut<TrafficLight,TrafficLight>;
     type Others = ();
     type Reply=();
     type Signal = ();
     type State = Vec<(ActorId, String)>;

}