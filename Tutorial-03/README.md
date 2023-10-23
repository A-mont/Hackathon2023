# Hackathon 2023

## Contrato inteligente: Scrow

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
use gmeta::{In, InOut,Metadata}; // Agregamos la dependencia In en gmeta
```


### PASO 1 Definir las acciones para el contrato: .
**comando:**
```rust
#[derive(Encode, Decode, TypeInfo)]
pub enum EscrowAction {
    Deposit,
    ConfirmDelivery,
}

```

### PASO 2 Definir las eventos para el contrato: .
**comando:**
```rust
#[derive(Encode, Decode, TypeInfo)]
pub enum EscrowEvent {
    FundsDeposited,
    DeliveryConfirmed,
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
pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = In<InitEscrow>; // Definimos el struct que queremos de inicio.
    type Handle = InOut<EscrowAction, EscrowEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Escrow;
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