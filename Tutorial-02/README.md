# Hackathon 2023

## Contrato inteligente: Semaforo

## Inicio: Clonar el template para contratos inteligentes

**comando:**
```bash

git clone https://github.com/Vara-Lab/SmartContractTemplate_v1.git
```

## Directorio IO

### PASO 1 Definir las acciones.
**comando:**
```rust
#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum Action {
    
    // Add Actions
    Hello
}

```

### PASO 2 Definir los eventos de salida y el estado.
**comando:**
```rust
pub struct ContractMetadata;

impl Metadata for ContractMetadata{
     type Init = ();
     type Handle = InOut<Action,String>; // Acciones como entrada y  eventos como salida.
     type Others = ();
     type Reply=();
     type Signal = ();
     type State = Vec<(ActorId, u128)>; // Estado 

}
```


## Directorio src

### PASO 1 Definir en el interior de la función Handle y definimos Acción->Transición->Evento.
**comando:**
```rust
#[no_mangle]
extern "C" fn handle(){


    let input_message: Action = msg::load()
        .expect("Error in loading InputMessages");
   

    match input_message {
       
        Action::Hello => {

            msg::reply(String::from("Hello"), 0)
            .expect("Error in sending a reply message");

           
        }
    }

}
```

## Despliega el contrato en la plataforma IDEA e interactua con tu contrato.