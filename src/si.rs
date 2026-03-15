//! Базовые и производные единицы СИ
use crate::unit::Unit;

// Базовые единицы СИ
pub const METER: Unit<f64> = Unit { alias: "m", value: 1.0 };
pub const KILOGRAM: Unit<f64> = Unit { alias: "kg", value: 1.0 };
pub const SECOND: Unit<f64> = Unit { alias: "s", value: 1.0 };
pub const AMPERE: Unit<f64> = Unit { alias: "A", value: 1.0 };
pub const KELVIN: Unit<f64> = Unit { alias: "K", value: 1.0 };
pub const MOLE: Unit<f64> = Unit { alias: "mol", value: 1.0 };
pub const CANDELA: Unit<f64> = Unit { alias: "cd", value: 1.0 };
pub const RADIAN: Unit<f64> = Unit { alias: "rad", value: 1.0 };

// Производные единицы СИ

// Механика
pub const NEWTON: Unit<f64> = Unit { 
    alias: "N", 
    value: KILOGRAM.value * METER.value / (SECOND.value * SECOND.value) 
};

pub const PASCAL: Unit<f64> = Unit { 
    alias: "Pa", 
    value: NEWTON.value / (METER.value * METER.value) 
};

pub const JOULE: Unit<f64> = Unit { 
    alias: "J", 
    value: NEWTON.value * METER.value 
};

pub const WATT: Unit<f64> = Unit { 
    alias: "W", 
    value: JOULE.value / SECOND.value 
};

// Электричество и магнетизм
pub const COULOMB: Unit<f64> = Unit { 
    alias: "C", 
    value: SECOND.value * AMPERE.value 
};

pub const VOLT: Unit<f64> = Unit { 
    alias: "V", 
    value: WATT.value / AMPERE.value 
};

pub const FARAD: Unit<f64> = Unit { 
    alias: "F", 
    value: COULOMB.value / VOLT.value 
};

pub const OHM: Unit<f64> = Unit { 
    alias: "Ω", 
    value: VOLT.value / AMPERE.value 
};

pub const SIEMENS: Unit<f64> = Unit { 
    alias: "S", 
    value: AMPERE.value / VOLT.value 
};

pub const WEBER: Unit<f64> = Unit { 
    alias: "Wb", 
    value: VOLT.value * SECOND.value 
};

pub const TESLA: Unit<f64> = Unit { 
    alias: "T", 
    value: WEBER.value / (METER.value * METER.value) 
};

pub const HENRY: Unit<f64> = Unit { 
    alias: "H", 
    value: WEBER.value / AMPERE.value 
};

// Оптика
pub const LUMEN: Unit<f64> = Unit { 
    alias: "lm", 
    value: CANDELA.value * RADIAN.value 
};

pub const LUX: Unit<f64> = Unit { 
    alias: "lx", 
    value: LUMEN.value / (METER.value * METER.value) 
};

// Радиоактивность
pub const BECQUEREL: Unit<f64> = Unit { 
    alias: "Bq", 
    value: 1.0 / SECOND.value 
};

pub const GRAY: Unit<f64> = Unit { 
    alias: "Gy", 
    value: (METER.value * METER.value) / (SECOND.value * SECOND.value) 
};

pub const SIEVERT: Unit<f64> = Unit { 
    alias: "Sv", 
    value: (METER.value * METER.value) / (SECOND.value * SECOND.value) 
};

// Химия
pub const KATAL: Unit<f64> = Unit { 
    alias: "kat", 
    value: MOLE.value / SECOND.value 
};

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_si_units() {
        assert_relative_eq!(METER.value, 1.0);
        assert_relative_eq!(NEWTON.value, 1.0);
        assert_relative_eq!(JOULE.value, 1.0);
        assert_relative_eq!(WATT.value, 1.0);
    }
}