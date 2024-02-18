use rand::Rng;

#[derive(Debug, PartialEq)]
enum IsEnabled {
    On,
    Off,
}

#[derive(Debug)]
pub struct Thermometr {
    temp: i32,
    is_enabled: IsEnabled
}

impl Default for Thermometr {
    fn default() -> Self {
        Self::new()
    }
}


impl Thermometr {
    pub fn new() -> Self {
        Self { temp: 0, is_enabled: IsEnabled::Off }
    }

    pub fn on_off(&mut self, click: &str) -> String {
        match Some(click.trim()) {
            Some("on") => {
                self.is_enabled = IsEnabled::On;
                "On"
            }

            Some("off") => {
                self.is_enabled = IsEnabled::Off;
                "Off"
            }
            _ => "Wrong command",
        };

        format!("{:#?}", self)
    }

    pub fn random_temp(&mut self) -> i32 {
        let mut rng = rand::thread_rng();
        let random_temp = rng.gen_range(-50..=50);

        self.temp = random_temp;
        self.temp
        
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_new_thermometer() {
        let _new_thermometr = Thermometr::new();
    }

    #[test]
    fn test_on_off_thermometr() {
        let mut new_thermometr = Thermometr::new();
        new_thermometr.on_off("on");
        assert_eq!(new_thermometr.is_enabled, IsEnabled::On);
        new_thermometr.on_off("off");
        assert_eq!(new_thermometr.is_enabled, IsEnabled::Off);
    }

    #[test]
    fn test_random_temp() {
        let mut new_thermometr = Thermometr::new();
        new_thermometr.on_off("on");
        let r_temp = new_thermometr.random_temp();
        assert_eq!(r_temp, new_thermometr.temp);
    }
}