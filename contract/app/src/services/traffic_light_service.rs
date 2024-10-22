use sails_rs::{
    prelude::*,
    gstd::{service, msg},
    cell::{RefMut, Ref},
};

// importar los estados definidos en otros mdulos
use crate::states::{
    traffic_light_state::TrafficLightState,
    keyring_state::{KeyringAccounts, KeyringError},
};

// definicion del servicio del semáforo
pub struct TrafficLightService<'a> {
    pub state: RefMut<'a, TrafficLightState>,
    pub keyring_state: Ref<'a, KeyringAccounts>,
}

// implementacion del servicio usando el macro #[service]
#[service]
impl<'a> TrafficLightService<'a> {
    // constructor del servicio
    pub fn new(
        state: RefMut<'a, TrafficLightState>,
        keyring_state: Ref<'a, KeyringAccounts>,
    ) -> Self {
        Self { state, keyring_state }
    }

    // añade dinero al balance
    pub fn green(&mut self, user_coded_name: String) -> TrafficLightEvent {
        self.process_event(user_coded_name, "Green", || {
            self.state.balance += 100; // añade 100 al balance
        })
    }

    // descuenta un 10% del balance
    pub fn yellow(&mut self, user_coded_name: String) -> TrafficLightEvent {
        self.process_event(user_coded_name, "Yellow", || {
            self.state.balance = (self.state.balance as f32 * 0.9) as u64; // Descuenta 10%
        })
    }

    // elimina una publicacion específica por indice
    pub fn red(
        &mut self, 
        user_coded_name: String, 
        index: usize
    ) -> TrafficLightEvent {
        self.process_event(user_coded_name, "Red", || {
            if index < self.state.publications.len() {
                self.state.publications.remove(index); // Elimina la publicación específica
            }
        })
    }

    // crea una nueva publicacion
    pub fn orange(&mut self, user_coded_name: String, content: String) -> TrafficLightEvent {
        self.process_event(user_coded_name, "Orange", || {
            self.state.publications.push(content); // Añade una nueva publicación
        })
    }

    // funcin auxiliar para manejar eventos comunes
    fn process_event<F>(
        &mut self,
        user_coded_name: String,
        light_color: &str,
        action: F,
    ) -> TrafficLightEvent
    where
        F: FnOnce(), // el cierre se ejecuta una vez
    {
        let keyring_address = msg::source(); // direccin del usuario

        // verificar si la dirección esta asociada al nombre codificado
        let temp = self.keyring_state.check_keyring_address_by_user_coded_name(
            keyring_address,
            user_coded_name,
        );

        if let Err(error) = temp {
            return TrafficLightEvent::Error(error); // retorna error si ocurre
        }

        // Actualiza el estado con el color actual
        self.state.current_light = light_color.to_string();
        self.state
            .all_users
            .insert(keyring_address, light_color.to_string());

        // ejecuta la acción especifica para el evento
        action();

        // devuelve el evento correspondiente
        match light_color {
            "Green" => TrafficLightEvent::Green,
            "Yellow" => TrafficLightEvent::Yellow,
            "Red" => TrafficLightEvent::Red,
            "Orange" => TrafficLightEvent::Orange,
            _ => TrafficLightEvent::Error(KeyringError::Unknown),
        }
    }
}

// num para los eventos del trfico
#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum TrafficLightEvent {
    Green,
    Yellow,
    Red,
    Orange,
    Error(KeyringError),
}