use sails_rs::{
    prelude::*,
    gstd::{service, msg},
    collections::HashMap,
};

// Definición del estado del semáforo
#[derive(Clone, Default)]
pub struct TrafficLightState {
    pub current_light: String,           // Luz actual del semáforo
    pub balance: u64,                    // Balance del usuario
    pub publications: Vec<String>,       // Publicaciones del usuario
    pub all_users: HashMap<ActorId, String>, // Usuarios y su luz actual
}

// Estructura para enviar el estado al usuario
#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct IoTrafficLightState {
    pub current_light: String,
    pub all_users: Vec<(ActorId, String)>,
}

// Implementación del trait From para convertir TrafficLightState a IoTrafficLightState
impl From<TrafficLightState> for IoTrafficLightState {
    fn from(value: TrafficLightState) -> Self {
        let TrafficLightState {
            current_light,
            all_users,
            ..
        } = value;

        let all_users = all_users.iter().map(|(k, v)| (*k, v.clone())).collect();

        Self {
            current_light,
            all_users,
        }
    }
}

// Definición del servicio del semáforo
pub struct TrafficLightService<'a> {
    pub state: RefMut<'a, TrafficLightState>,
}

// Implementación del servicio usando #[service]
#[service]
impl<'a> TrafficLightService<'a> {
    // Constructor del servicio
    pub fn new(state: RefMut<'a, TrafficLightState>) -> Self {
        Self { state }
    }

    // Añade dinero al balance
    pub fn green(&mut self, user_coded_name: String) -> TrafficLightEvent {
        self.process_event(user_coded_name, "Green", || {
            self.state.balance += 100; // Añade 100 al balance
        })
    }

    // Descuenta un 10% del balance
    pub fn yellow(&mut self, user_coded_name: String) -> TrafficLightEvent {
        self.process_event(user_coded_name, "Yellow", || {
            self.state.balance = (self.state.balance as f32 * 0.9) as u64; // Descuenta 10%
        })
    }

    // Elimina una publicacion especfica por índice
    pub fn red(&mut self, user_coded_name: String, index: usize) -> TrafficLightEvent {
        self.process_event(user_coded_name, "Red", || {
            if index < self.state.publications.len() {
                self.state.publications.remove(index); // Elimina la publicación específica
            }
        })
    }

    // Crea una nueva publicación
    pub fn orange(&mut self, user_coded_name: String, content: String) -> TrafficLightEvent {
        self.process_event(user_coded_name, "Orange", || {
            self.state.publications.push(content); // Añade una nueva publicación
        })
    }

    // Funcin auxiliar para manejar eventos comunes
    fn process_event<F>(
        &mut self,
        user_coded_name: String,
        light_color: &str,
        action: F,
    ) -> TrafficLightEvent
    where
        F: FnOnce(),
    {
        let keyring_address = msg::source();

        self.state
            .all_users
            .insert(keyring_address, light_color.to_string());

        self.state.current_light = light_color.to_string();
        action();

        match light_color {
            "Green" => TrafficLightEvent::Green,
            "Yellow" => TrafficLightEvent::Yellow,
            "Red" => TrafficLightEvent::Red,
            "Orange" => TrafficLightEvent::Orange,
            _ => TrafficLightEvent::Error(KeyringError::Unknown),
        }
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
    Error(KeyringError),
}

// Definicin del error para la Keyring
#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum KeyringError {
    Unknown,
}