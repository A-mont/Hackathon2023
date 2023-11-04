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
use gmeta::{InOut,Metadata};

use primitive_types::U256;

pub type TransactionId = u64;

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

### PASO 2 Definir los eventos para el contrato de subasta.
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

#[derive(Debug, Clone, Default, Encode, Decode, TypeInfo)]
pub struct Transaction<T: Clone> {
    pub id: TransactionId,
    pub action: T,
}

```

### PASO OPCIONAL(ERRORES). Definir errores personalizados para tener un mejor control de los eventos del contrato.
```rust
/// An enum that contains a error of processed [`Action`].
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum Error {
    PreviousTxMustBeCompleted,
    SendingError,
    NftValidateFailed,
    NftTransferFailed,
    NftOwnerFailed,
    NftNotApproved,
    NotRewarded,
    WrongReply,
    RewardSendFailed,
    NotOwner,
    AlreadyRunning,
    StartPriceLessThatMinimal,
    AlreadyStopped,
    InsufficientMoney,
    Expired,
    WrongState,
    IncorrectRewarder,
}

```

### PASO 4 Definir las acciones, estado y eventos.
**comando:**
```rust
pub struct ContractMetadata;

impl Metadata for ContractMetadata {
    type Init = ();
    type Handle = InOut<Action, Result<Event, Error>>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = AuctionInfo;
}
```


## Directorio src


### PASO 0 Importamos dependencias necesarias en el Src para el contrato de subasta:
```rust
use core::cmp::min;
use gmeta::Metadata;
use gstd::ActorId;
use gstd::{errors::Result as GstdResult, exec, msg, prelude::*, MessageId};
use primitive_types::U256;
use nft_io::{NFTAction, NFTEvent};
use io::*;
```

### PASO 1 Definimos el estado de la subasta.
**comando:**
```rust
static mut AUCTION: Option<Auction> = None;

```

### PASO 1.1 Definimos la mutabilidad del estado definido anteriormente
**comando:**
```rust
fn common_state() -> <ContractMetadata as Metadata>::State {
    static_mut_state().info()
}

fn static_mut_state() -> &'static mut Auction {
    unsafe { AUCTION.get_or_insert(Default::default()) }
}
```

### PASO 2 Creamos las estructuras necesarias para NFT y la subasta.
**comando:**
```rust
#[derive(Debug, Clone, Default)]
pub struct Nft {
    pub token_id: U256,
    pub owner: ActorId,
    pub contract_id: ActorId,
}

#[derive(Debug, Clone, Default)]
pub struct Auction {
    pub owner: ActorId,
    pub nft: Nft,
    pub starting_price: u128,
    pub discount_rate: u128,
    pub status: Status,
    pub started_at: u64,
    pub expires_at: u64,

    pub transactions: BTreeMap<ActorId, Transaction<Action>>,
    pub current_tid: TransactionId,
}
```

### PASO 3 Como el estado es una estructura llamada Auction/Subasta podemos hacerle implementaciones.
**comando:**
```rust
impl Auction {
    pub async fn buy(&mut self, transaction_id: TransactionId) -> Result<(Event, u128), Error> {
        if !matches!(self.status, Status::IsRunning) {
            return Err(Error::AlreadyStopped);
        }

        if exec::block_timestamp() >= self.expires_at {
            return Err(Error::Expired);
        }

        let price = self.token_price();
        let value = msg::value();
        if value < price {
            return Err(Error::InsufficientMoney);
        }

        self.status = Status::Purchased { price };

        let refund = value - price;
        let refund = if refund < 500 { 0 } else { refund };

        let reply = match msg::send_for_reply(
            self.nft.contract_id,
            NFTAction::Transfer {
                to: msg::source(),
                token_id: self.nft.token_id,
                transaction_id,
            },
            0,
            0,
        ) {
            Ok(reply) => reply,
            Err(_e) => {
                return Err(Error::NftTransferFailed);
            }
        };

        match reply.await {
            Ok(_reply) => {}
            Err(_e) => {
                return Err(Error::NftTransferFailed);
            }
        }

        Ok((Event::Bought { price }, refund))
    }

    pub fn token_price(&self) -> u128 {
        // time_elapsed is in seconds
        let time_elapsed = exec::block_timestamp().saturating_sub(self.started_at) / 1000;
        let discount = min(
            self.discount_rate * (time_elapsed as u128),
            self.starting_price,
        );

        self.starting_price - discount
    }

    pub async fn renew_contract(
        &mut self,
        transaction_id: TransactionId,
        config: &CreateConfig,
    ) -> Result<Event, Error> {
        if matches!(self.status, Status::IsRunning) {
            return Err(Error::AlreadyRunning);
        }

        let minutes_count = config.duration.hours * 60 + config.duration.minutes;
        let duration_in_seconds = minutes_count * 60 + config.duration.seconds;

        if config.starting_price < config.discount_rate * (duration_in_seconds as u128) {
            return Err(Error::StartPriceLessThatMinimal);
        }
        self.validate_nft_approve(config.nft_contract_actor_id, config.token_id)
            .await?;
        self.status = Status::IsRunning;
        self.started_at = exec::block_timestamp();
        self.expires_at = self.started_at + duration_in_seconds * 1000;
        self.nft.token_id = config.token_id;
        self.nft.contract_id = config.nft_contract_actor_id;
        self.nft.owner =
            Self::get_token_owner(config.nft_contract_actor_id, config.token_id).await?;

        self.discount_rate = config.discount_rate;
        self.starting_price = config.starting_price;

        msg::send_for_reply(
            self.nft.contract_id,
            NFTAction::Transfer {
                transaction_id,
                to: exec::program_id(),
                token_id: self.nft.token_id,
            },
            0,
            0,
        )
        .expect("Send NFTAction::Transfer at renew contract")
        .await
        .map_err(|_e| Error::NftTransferFailed)?;
        Ok(Event::AuctionStarted {
            token_owner: self.owner,
            price: self.starting_price,
            token_id: self.nft.token_id,
        })
    }

    pub async fn reward(&mut self) -> Result<Event, Error> {
        let price = match self.status {
            Status::Purchased { price } => price,
            _ => return Err(Error::WrongState),
        };
        if msg::source().ne(&self.nft.owner) {
            return Err(Error::IncorrectRewarder);
        }

        if let Err(_e) = msg::send(self.nft.owner, "REWARD", price) {
            return Err(Error::RewardSendFailed);
        }
        self.status = Status::Rewarded { price };
        Ok(Event::Rewarded { price })
    }

    pub async fn get_token_owner(contract_id: ActorId, token_id: U256) -> Result<ActorId, Error> {
        let reply: NFTEvent =
            msg::send_for_reply_as(contract_id, NFTAction::Owner { token_id }, 0, 0)
                .map_err(|_e| Error::SendingError)?
                .await
                .map_err(|_e| Error::NftOwnerFailed)?;

        if let NFTEvent::Owner { owner, .. } = reply {
            Ok(owner)
        } else {
            Err(Error::WrongReply)
        }
    }

    pub async fn validate_nft_approve(
        &self,
        contract_id: ActorId,
        token_id: U256,
    ) -> Result<(), Error> {
        let to = exec::program_id();
        let reply: NFTEvent =
            msg::send_for_reply_as(contract_id, NFTAction::IsApproved { token_id, to }, 0, 0)
                .map_err(|_e| Error::SendingError)?
                .await
                .map_err(|_e| Error::NftNotApproved)?;

        if let NFTEvent::IsApproved { approved, .. } = reply {
            if !approved {
                return Err(Error::NftNotApproved);
            }
        } else {
            return Err(Error::WrongReply);
        }
        Ok(())
    }

    pub fn stop_if_time_is_over(&mut self) {
        if matches!(self.status, Status::IsRunning) && exec::block_timestamp() >= self.expires_at {
            self.status = Status::Expired;
        }
    }

    pub async fn force_stop(&mut self, transaction_id: TransactionId) -> Result<Event, Error> {
        if msg::source() != self.owner {
            return Err(Error::NotOwner);
        }
        if let Status::Purchased { price: _ } = self.status {
            return Err(Error::NotRewarded);
        }

        let stopped = Event::AuctionStopped {
            token_owner: self.owner,
            token_id: self.nft.token_id,
        };
        if let Status::Rewarded { price: _ } = self.status {
            return Ok(stopped);
        }
        if let Err(_e) = msg::send_for_reply(
            self.nft.contract_id,
            NFTAction::Transfer {
                transaction_id,
                to: self.nft.owner,
                token_id: self.nft.token_id,
            },
            0,
            0,
        )
        .expect("Can't send NFTAction::Transfer at force stop")
        .await
        {
            return Err(Error::NftTransferFailed);
        }

        self.status = Status::Stopped;

        Ok(stopped)
    }

    pub fn info(&mut self) -> AuctionInfo {
        self.stop_if_time_is_over();
        AuctionInfo {
            nft_contract_actor_id: self.nft.contract_id,
            token_id: self.nft.token_id,
            token_owner: self.nft.owner,
            auction_owner: self.owner,
            starting_price: self.starting_price,
            current_price: self.token_price(),
            discount_rate: self.discount_rate,
            time_left: self.expires_at.saturating_sub(exec::block_timestamp()),
            expires_at: self.expires_at,
            status: self.status.clone(),
            transactions: self.transactions.clone(),
            current_tid: self.current_tid,
        }
    }
}
```

### PASO 4 Definimos la funcion Init()
**comando:**
```rust
#[no_mangle]
extern fn init() {
    let auction = Auction {
        owner: msg::source(),
        ..Default::default()
    };

    unsafe { AUCTION = Some(auction) };
}

```


### PASO 5 Definimos la funcion Handle() o principal(main) como asincrona usando el macro gstd::async_main
**comando:**
```rust
#[gstd::async_main]
async fn main() {
    let action: Action = msg::load().expect("Could not load Action");
    let auction: &mut Auction = unsafe { AUCTION.get_or_insert(Auction::default()) };

    auction.stop_if_time_is_over();

    let msg_source = msg::source();

    let r: Result<Action, Error> = Err(Error::PreviousTxMustBeCompleted);
    let transaction_id = if let Some(Transaction {
        id: tid,
        action: pend_action,
    }) = auction.transactions.get(&msg_source)
    {
        if action != *pend_action {
            reply(r, 0).expect("Failed to encode or reply with `Result<Action, Error>`");
            return;
        }
        *tid
    } else {
        let transaction_id = auction.current_tid;
        auction.transactions.insert(
            msg_source,
            Transaction {
                id: transaction_id,
                action: action.clone(),
            },
        );
        auction.current_tid = auction.current_tid.wrapping_add(1);
        transaction_id
    };

    let (result, value) = match &action {
        Action::Buy => {
            let reply = auction.buy(transaction_id).await;
            let result = match reply {
                Ok((event, refund)) => (Ok(event), refund),
                Err(_e) => (Err(_e), 0),
            };
            auction.transactions.remove(&msg_source);
            result
        }
        Action::Create(config) => {
            let result = (auction.renew_contract(transaction_id, config).await, 0);
            auction.transactions.remove(&msg_source);
            result
        }
        Action::ForceStop => {
            let result = (auction.force_stop(transaction_id).await, 0);
            auction.transactions.remove(&msg_source);
            result
        }
        Action::Reward => {
            let result = (auction.reward().await, 0);
            auction.transactions.remove(&msg_source);
            result
        }
    };
    reply(result, value).expect("Failed to encode or reply with `Result<Event, Error>`");
}
```

### PASO 5 Definimos la funcion State()
**comando:**
```rust
#[no_mangle]
extern "C" fn state() {
    reply(common_state(), 0).expect(
        "Failed to encode or reply with `<AuctionMetadata as Metadata>::State` from `state()`",
    );
}
fn reply(payload: impl Encode, value: u128) -> GstdResult<MessageId> {
    msg::reply(payload, value)
}
```


## Directorio state


### PASO 1 Definimos el estado y la metadata asociada en el directorio state
**comando:**
```rust
#[gmeta::metawasm]
pub mod metafns {
    pub type State = AuctionInfo;

    pub fn info(mut state: State) -> AuctionInfo {
        if matches!(state.status, Status::IsRunning) && exec::block_timestamp() >= state.expires_at
        {
            state.status = Status::Expired
        }
        state
    }
}
```

### PASO FINAL: COMPILAMOS EL CONTRATO USANDO EL COMANDO
**comando:**
```rust
cargo build --release
```

## Despliega el contrato en la plataforma IDEA e interactua con tu contrato.