const SECONDS_IN_YEAR: u32 = 9203544;
const SECONDS_IN_DAY: u32 = 21600;

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

    pub fn get_orbital_period(planets: &[Planet; 7], starting_planet: &str, ending_planet: &str, time_unit: TimeUnit) -> f32 {
        if starting_planet == ending_planet {
            return 0 as f32;
        }
        let mut starting_orbital_period: u32 = 0;
        let mut target_orbital_period: u32 = 0;
        for planet in planets {
            if planet.name == starting_planet {
                starting_orbital_period = planet.orbital_period;
            }
            if planet.name == ending_planet {
                target_orbital_period = planet.orbital_period;
            }
        }
        // abs( 1 / ( (1/orbital_period_start) - (1/orbital_period_end) ) )
        let res: f32 = 1f32 / ( (1f32 / starting_orbital_period as f32) - (1f32 / target_orbital_period as f32) );
        let absolute = res.abs();
        Self::convert_time_from_seconds(absolute, time_unit)
    }

    pub fn get_next_transfer_window(current_year: &str, current_day: &str, transfer_window: f32, time_unit: TimeUnit) -> f32 {
        let year = match current_year.parse::<f32>() {
            Ok(t) => t * SECONDS_IN_YEAR as f32, // Convert years to seconds. Kerbins orbital period is the length of a KSP year
            Err(_e) => return -1f32
        };
        let day = match current_day.parse::<f32>() {
            Ok(t) => t * SECONDS_IN_DAY as f32, // Convert days to seconds
            Err(_e) => return -1f32
        };
        let total_time_passed = year + day;
        let time_into_current_window = total_time_passed % transfer_window;
        Self::convert_time_from_seconds(transfer_window - time_into_current_window, time_unit)
    }

    pub fn convert_time_from_seconds(res: f32, time_unit: TimeUnit) -> f32 {
        match time_unit {
            TimeUnit::Seconds => res,
            TimeUnit::Days => res / SECONDS_IN_DAY as f32
        }
    }
}
