# Hackathon 2023

## Contrato inteligente: Semaforo

## Inicio: Clonar el template para contratos inteligentes

**comando:**
```bash
git clone https://github.com/Vara-Lab/SmartContractTemplate_v1.git
```

## Directorio IO

### PASO 1 Definir las acciones para el semaforo: .
**comando:**
```rust
#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum ActionTrafficLight {
    // Actions
     Green,
     Yellow,
     Red
}

```

### PASO 2 Definir las eventos para el semaforo: .
**comando:**
```rust
#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum EventTrafficLight {
    // Actions
     Green,
     Yellow,
     Red
}

```

### PASO 3 Definir las acciones, estado y eventos.
**comando:**
```rust
pub struct ContractMetadata;

impl Metadata for ContractMetadata{
     type Init = ();
     type Handle = InOut<ActionTrafficLight,EventTrafficLight>; // Acciones como entrada y  eventos como salida.
     type Others = ();
     type Reply=();
     type Signal = ();
     type State = Vec<(ActorId, String)>; // Estado 

}
```


## Directorio src

### PASO 1 Definir en el interior de la función Handle y definimos Acción->Transición->Evento.
**comando:**
```rust

#[no_mangle]
extern "C" fn handle(){


    handle_state().expect("Execution Error")


}

    

fn handle_state() -> Result<()> {

        let payload = msg::load()?;

        if let ActionTrafficLight::Green = payload {

            let currentstate = state_mut();
            currentstate.insert(msg::source(), "Green".to_string());
            msg::reply(EventTrafficLight::Green,0)?;

        }

        if let ActionTrafficLight::Yellow = payload {

            let currentstate = state_mut();
            currentstate.insert(msg::source(), "Yellow".to_string());
            msg::reply(EventTrafficLight::Yellow,0)?;

        }

        if let ActionTrafficLight::Red = payload {

            let currentstate = state_mut();
            currentstate.insert(msg::source(), "Red".to_string());
            msg::reply(EventTrafficLight::Red,0)?;

        }

    Ok(())
    }


```

## Despliega el contrato en la plataforma IDEA e interactua con tu contrato.