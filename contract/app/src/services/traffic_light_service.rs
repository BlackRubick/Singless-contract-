// Imports necesarios
use sails_rs::{prelude::*, gstd::msg, ActorId};
use crate::states::traffic_light_state::{TrafficLightState, IoTrafficLightState};

// Definir Balance como alias de u128
type Balance = u128;

// Estructura del servicio
#[derive(Default)]
pub struct TrafficLightService;

impl TrafficLightService {
    // Inicializa el estado del servicio
    pub fn seed() {
        TrafficLightState::init_state();
    }
}

#[service]
impl TrafficLightService {
    // Constructor del servicio
    pub fn new() -> Self {
        Self
    }

    // Método para cambiar la luz a verde y agregar fondos a la wallet del usuario
    pub fn green(&mut self) -> TrafficLightEvent {
        let current_light = "Green".to_string();
        let actor_id: ActorId = msg::source().into();

        // Actualizar el estado
        TrafficLightState::state_mut().current_light = current_light.clone();
        TrafficLightState::state_mut()
            .all_users
            .insert(actor_id, current_light);
        TrafficLightState::state_mut().some_value += 100;

        // Agregar fondos a la wallet del usuario
        let amount_to_send: Balance = 10_000_000_000; // Ajusta este valor según tu token
        msg::send(
            actor_id,
            (),
            amount_to_send,
        )
        .expect("Failed to send funds");

        TrafficLightEvent::Green
    }

    // Método para cambiar la luz a amarillo y descontar 1 unidad de la wallet del usuario
    pub fn yellow(&mut self) -> TrafficLightEvent {
        let current_light = "Yellow".to_string();
        let actor_id: ActorId = msg::source().into();

        // Verificar que el usuario adjuntó 1 unidad de valor
        let expected_amount: Balance = 1_000_000_000; // Ajusta según tu token
        let attached_value = msg::value();

        if attached_value != expected_amount {
            panic!("Debes adjuntar exactamente 1 unidad de valor al llamar a este método.");
        }

        // Actualizar el estado
        TrafficLightState::state_mut().current_light = current_light.clone();
        TrafficLightState::state_mut()
            .all_users
            .insert(actor_id, current_light);

        TrafficLightEvent::Yellow
    }

    // Método para cambiar la luz a rojo y eliminar una publicación
    pub fn red(&mut self) -> TrafficLightEvent {
        let current_light = "Red".to_string();
        TrafficLightState::state_mut().current_light = current_light.clone();
        let actor_id: ActorId = msg::source().into();
        TrafficLightState::state_mut().all_users.remove(&actor_id);
        TrafficLightEvent::Red
    }

    // Método para cambiar la luz a naranja y crear una publicación
    pub fn orange(&mut self) -> TrafficLightEvent {
        let current_light = "Orange".to_string();
        TrafficLightState::state_mut().current_light = current_light.clone();
        TrafficLightState::state_mut()
            .all_users
            .insert(msg::source().into(), current_light);
        TrafficLightEvent::Orange
    }

    // Método para obtener el estado actual
    pub fn traffic_light(&self) -> IoTrafficLightState {
        TrafficLightState::state_ref().to_owned().into()
    }
}

// Enum para los eventos del semáforo
#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum TrafficLightEvent {
    Green,
    Yellow,
    Red,
    Orange,
}
