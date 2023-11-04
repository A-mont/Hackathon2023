# Hackathon 2023

## Contrato inteligente: Subasta

## Inicio: Clonar el template para contratos inteligentes

**comando:**
```bash
git clone https://github.com/Vara-Lab/SmartContractTemplate_v1.git
```

## Directorio IO

## Librerias y dependencias necesarias
```rust
#![no_std]
use gstd::{ prelude::*, ActorId };
use gmeta::{In, InOut,Metadata}; 
```


### PASO 1 Definir las acciones para el contrato de subasta.
**comando:**
```rust
/// After a successful processing of this enum, the program replies with [`Event`].
#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Action {
    /// Creates auction
    Create(CreateConfig),
    /// Buy current NFT
    Buy,
    /// Stop Auction
    ForceStop,
    /// Reward gas to NFT seller
    Reward,
}

```

### PASO 1.1 Definir una estructura para la accion Create.
**comando:**
```rust
#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct CreateConfig {
    /// Address of NFT contract
    pub nft_contract_actor_id: ActorId,
    /// NFT token id
    pub token_id: U256,
    /// Starting price
    pub starting_price: u128,
    /// Price step by which the NFT price decreases
    pub discount_rate: u128,
    /// Auction duration
    pub duration: Duration,
}

// Definimos la estructura Duracion para CreateConfig
#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Duration {
    pub hours: u64,
    pub minutes: u64,
    pub seconds: u64,
}


```

### PASO 2 Definir las eventos para el contrato de subasta.
**comando:**
```rust
#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Event {
    AuctionStarted {
        /// Owner of auction NFT
        token_owner: ActorId,
        /// Started price of NFT
        price: u128,
        /// NFT token id
        token_id: U256,
    },
    Bought {
        /// Price for which the NFT were bought
        price: u128,
    },
    AuctionStopped {
        token_owner: ActorId,
        token_id: U256,
    },
    Rewarded {
        /// Reward that owner received
        price: u128,
    },
}


```


### PASO 3 Estructura personalizada para la subasta

```rust
#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct AuctionInfo {
    /// NFT contract address
    pub nft_contract_actor_id: ActorId,
    /// NFT token id
    pub token_id: U256,
    /// NFT owner
    pub token_owner: ActorId,
    /// Auction owner
    pub auction_owner: ActorId,
    /// Starting price of NFT at auction
    pub starting_price: u128,
    /// Current price of NFT
    pub current_price: u128,
    /// Price step by which the NFT price decreases
    pub discount_rate: u128,
    /// Time left until the end of the auction
    pub time_left: u64,
    /// Time when the auction expires
    pub expires_at: u64,
    /// Current auction status
    pub status: Status,

    /// Transactions that cached on contract
    pub transactions: BTreeMap<ActorId, Transaction<Action>>,
    /// Current transaction id
    pub current_tid: u64,
}

#[derive(Debug, Decode, Default, Encode, TypeInfo, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Status {
    #[default]
    None,
    /// Auction is running right now
    IsRunning,
    /// Someone purchased NFT, but previous NFT owner not rewarded
    Purchased { price: u128 },
    /// Someone purchased NFT and previous NFT owner rewarded
    Rewarded { price: u128 },
    /// Time for the auction has expired and no one has made a purchase.
    Expired,
    /// Auction stopped by auction owner
    Stopped,
}

```



### PASO 3 Definimos un Struct llamado Scrow y un enum para su estado
**comando:**
```rust
#[derive(Default, Encode, Decode, TypeInfo)]
pub struct Escrow {
    pub seller: ActorId,
    pub buyer: ActorId,
    pub price: u128,
    pub state: EscrowState,
}

#[derive(Default, Debug, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum EscrowState {
    #[default]
    AwaitingPayment,
    AwaitingDelivery,
    Closed,
}

```

### PASO 4 Definimos un Struct para iniciar el programa
**comando:**
```rust
#[derive(Decode,Encode, TypeInfo)]
pub struct InitEscrow {
    pub seller: ActorId,
    pub buyer: ActorId,
    pub price: u128,
}

```





### PASO 5 Definir las acciones, estado y eventos.
**comando:**
```rust
use crate::auction::{Action, AuctionInfo, Error, Event};

pub struct AuctionMetadata;

impl Metadata for AuctionMetadata {
    type Init = ();
    type Handle = InOut<Action, Result<Event, Error>>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = Out<AuctionInfo>;
}
```


## Directorio src


### PASO 1 Definimos el estado SCROW.
**comando:**
```rust
static mut ESCROW: Option<Escrow> =  None;
```


### PASO 2 Creamos la función para volver mutable el estado.
**comando:**
```rust

fn scrow_state_mut() -> &'static mut Escrow {

    let state = unsafe {  ESCROW.as_mut()};

    unsafe { state.unwrap_unchecked() }


}
```

### PASO 3 Como el estado es un struct podemos hacerle implementaciones.
**comando:**
```rust


#[derive(Default, Encode, Decode, TypeInfo)]
pub struct Escrow {
    pub seller: ActorId,
    pub buyer: ActorId,
    pub price: u128,
    pub state: EscrowState,
}

impl Escrow {
    fn deposit(&mut self) {}
    fn confirm_delivery(&mut self) {}
}

```

### PASO 4 Definimos la funcion Init()
**comando:**
```rust
#[no_mangle]
extern "C" fn init() {
    let init_config: InitEscrow = msg::load()
        .expect("Error in decoding `InitEscrow`");
    let escrow = Escrow {
        seller: init_config.seller,
        buyer: init_config.buyer,
        price: init_config.price,
        state: EscrowState::AwaitingPayment,
    };
    unsafe { ESCROW = Some(escrow) };

}
```


### PASO 5 Definimos la funcion Handle()
**comando:**
```rust
#[no_mangle]
extern "C" fn handle() {

    let action: EscrowAction = msg::load()
    .expect("Unable to decode `EscrowAction`");
let escrow: &mut Escrow = unsafe {
    ESCROW
        .as_mut()
        .expect("The contract is not initialized")
};
match action {
    EscrowAction::Deposit => escrow.deposit(),
    EscrowAction::ConfirmDelivery => escrow.confirm_delivery(),
}


}
```

### PASO 5 Definimos la funcion State()
**comando:**
```rust
#[no_mangle]
extern "C" fn state() {
    let escrow = unsafe {
        ESCROW.get_or_insert(Default::default())
    };
    msg::reply(escrow, 0).expect("Failed to share state");
}
```


## Directorio state


### PASO 1 Definimos el estado y la metadata asociada en el directorio state
**comando:**
```rust
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
```

## Despliega el contrato en la plataforma IDEA e interactua con tu contrato.