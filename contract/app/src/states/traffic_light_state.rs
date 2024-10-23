// Imports necesarios
use sails_rs::{prelude::*, collections::HashMap, ActorId};

// Estado estático mutable
pub static mut TRAFFIC_LIGHT_STATE: Option<TrafficLightState> = None;

// Estructura para el estado
#[derive(Clone, Default)]
pub struct TrafficLightState {
    pub current_light: String,
    pub all_users: HashMap<ActorId, String>,
    pub some_value: u32, // Campo adicional para almacenar valores
}

impl TrafficLightState {
    // Inicializa una nueva instancia del estado
    pub fn new() -> Self {
        Self {
            current_light: "".to_string(),
            all_users: HashMap::new(),
            some_value: 0,
        }
    }

    // Inicializa el estado una vez
    pub fn init_state() {
        unsafe {
            TRAFFIC_LIGHT_STATE = Some(Self::new());
        };
    }

    // Devuelve el estado como mutable
    pub fn state_mut() -> &'static mut TrafficLightState {
        let state = unsafe { TRAFFIC_LIGHT_STATE.as_mut() };
        debug_assert!(state.is_some(), "El estado no está inicializado");
        unsafe { state.unwrap_unchecked() }
    }

    // Devuelve el estado como referencia inmutable
    pub fn state_ref() -> &'static TrafficLightState {
        let state = unsafe { TRAFFIC_LIGHT_STATE.as_ref() };
        debug_assert!(state.is_some(), "El estado no está inicializado");
        unsafe { state.unwrap_unchecked() }
    }
}

// Estructura para enviar el estado al usuario
#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct IoTrafficLightState {
    pub current_light: String,
    pub all_users: Vec<(ActorId, String)>,
    pub some_value: u32,
}

impl From<TrafficLightState> for IoTrafficLightState {
    fn from(value: TrafficLightState) -> Self {
        let TrafficLightState {
            current_light,
            all_users,
            some_value,
        } = value;

        let all_users = all_users.iter().map(|(k, v)| (*k, v.clone())).collect();

        Self {
            current_light,
            all_users,
            some_value,
        }
    }
}
