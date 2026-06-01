use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Ship {
    pub ship_type: String,
    pub ship_display: String,
    pub ship_name: Option<String>,
    pub ship_id: Option<String>,
}

impl From<edjr::elite::ship::Ship> for Ship {
    fn from(value: edjr::elite::ship::Ship) -> Self {
        Ship {
            ship_type: value.ship,
            ship_display: value.ship_display,
            ship_name: value.ship_name,
            ship_id: value.ship_ident,
        }
    }
}

impl Display for Ship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ship_display = if !self.ship_display.is_empty() {
            self.ship_display.clone()
        } else {
            self.ship_type.clone()
        };

        if let Some(ship_name) = &self.ship_name {
            ship_display.push_str(&format!(" ({})", ship_name));
        }

        write!(f, "{}", ship_display)
    }
}
