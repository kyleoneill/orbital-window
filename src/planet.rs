use std::num::ParseFloatError;

const SECONDS_IN_YEAR: u32 = 9203544;
const SECONDS_IN_DAY: u32 = 21600; // A Kerbin day is 6 hours long, not 24

pub struct OrbitalMechanics {
    pub planets: [Planet; 7],
    pub current_planet: String,
    pub selected_planet: String,
    current_year: f32,
    current_day: f32,
    total_time_passed: f32
}

impl OrbitalMechanics {
    pub fn new() -> Self {
        Self {
            planets: Planet::get_all_planets(),
            current_planet: "Kerbin".to_string(),
            selected_planet: "Duna".to_string(),
            current_year: 0f32,
            current_day: 0f32,
            total_time_passed: 0f32
        }
    }

    pub fn set_time(&mut self, current_year: &str, current_day: &str) -> Result<(), ParseFloatError> {
        let year = match current_year.parse::<f32>() {
            Ok(t) => t,
            Err(e) => return Err(e)
        };
        let day = match current_day.parse::<f32>() {
            Ok(t) => t,
            Err(e) => return Err(e)
        };
        let year_seconds = year * SECONDS_IN_YEAR as f32;
        let day_seconds = day * SECONDS_IN_DAY as f32;
        self.current_year = year;
        self.current_day = day;
        self.total_time_passed = year_seconds + day_seconds;
        Ok(())
    }

    pub fn get_synodic_orbital_period(&self, target_planet: &Planet, time_unit: TimeUnit) -> f32 {
        if self.current_planet == target_planet.name {
            return 0 as f32;
        }
        let mut starting_orbital_period: u32 = 0;
        let mut target_orbital_period: u32 = 0;
        for planet in &self.planets {
            if planet.name == self.current_planet {
                starting_orbital_period = planet.orbital_period;
            }
            if planet.name == target_planet.name {
                target_orbital_period = planet.orbital_period;
            }
        }
        // abs( 1 / ( (1/orbital_period_start) - (1/orbital_period_end) ) )
        let res: f32 = 1f32 / ( (1f32 / starting_orbital_period as f32) - (1f32 / target_orbital_period as f32) );
        let absolute = res.abs();
        Self::convert_time_from_seconds(absolute, time_unit)
    }

    pub fn get_next_synodic_period(&self, transfer_window: f32, time_unit: TimeUnit) -> f32 {
        let time_into_current_window = self.total_time_passed % transfer_window;
        Self::convert_time_from_seconds(transfer_window - time_into_current_window, time_unit)
    }

    pub fn convert_time_from_seconds(res: f32, time_unit: TimeUnit) -> f32 {
        match time_unit {
            TimeUnit::Seconds => res,
            TimeUnit::Days => res / SECONDS_IN_DAY as f32
        }
    }
}

pub struct Planet {
    pub name: String,
    pub orbital_period: u32
}

pub enum TimeUnit {
    Seconds,
    Days
}

impl Planet {
    fn new(name: String, orbital_period: u32) -> Self {
        Self {
            name,
            orbital_period
        }
    }

    pub fn get_all_planets() -> [Planet; 7] {
        [
            Self::new("Moho".to_string(), 2215754),
            Self::new("Eve".to_string(), 5657995),
            Self::new("Kerbin".to_string(), SECONDS_IN_YEAR),
            Self::new("Duna".to_string(), 17315400),
            Self::new("Dres".to_string(), 47893063),
            Self::new("Jool".to_string(), 104661432),
            Self::new("Eeloo".to_string(), 156992048),
        ]
    }
}
