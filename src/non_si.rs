//! Внесистемные единицы измерения
use crate::si::*;
use crate::unit::Unit;

// Время
pub const MINUTE: Unit<f64> = Unit { 
    alias: "min", 
    value: 60.0 * SECOND.value 
};

pub const HOUR: Unit<f64> = Unit { 
    alias: "h", 
    value: 60.0 * MINUTE.value 
};

pub const DAY: Unit<f64> = Unit { 
    alias: "d", 
    value: 24.0 * HOUR.value 
};

// Объём
pub const LITER: Unit<f64> = Unit { 
    alias: "L", 
    value: 0.001 * METER.value * METER.value * METER.value 
};

// Масса
pub const GRAM: Unit<f64> = Unit { 
    alias: "g", 
    value: 0.001 * KILOGRAM.value 
};

pub const TONNE: Unit<f64> = Unit { 
    alias: "t", 
    value: 1000.0 * KILOGRAM.value 
};

pub const DALTON: Unit<f64> = Unit { 
    alias: "Da", 
    value: 1.66053906660e-27 * KILOGRAM.value 
};

pub const ATOMIC_MASS_UNIT: Unit<f64> = Unit { 
    alias: "u", 
    value: DALTON.value 
};

// Давление
pub const BAR: Unit<f64> = Unit { 
    alias: "bar", 
    value: 100000.0 * PASCAL.value 
};

pub const ATMOSPHERE: Unit<f64> = Unit { 
    alias: "atm", 
    value: 101325.0 * PASCAL.value 
};

pub const MMHG: Unit<f64> = Unit { 
    alias: "mmHg", 
    value: 133.322 * PASCAL.value 
};

pub const TORR: Unit<f64> = Unit { 
    alias: "Torr", 
    value: MMHG.value 
};

// Энергия
pub const ELECTRON_VOLT: Unit<f64> = Unit { 
    alias: "eV", 
    value: 1.602176634e-19 * JOULE.value 
};

// Мощность
pub const HORSEPOWER: Unit<f64> = Unit { 
    alias: "hp", 
    value: 745.69987158227022 * WATT.value 
};

// Площадь
pub const HECTARE: Unit<f64> = Unit { 
    alias: "ha", 
    value: 10000.0 * METER.value * METER.value 
};

pub const ARE: Unit<f64> = Unit { 
    alias: "a", 
    value: 100.0 * METER.value * METER.value 
};

// Длина
pub const ANGSTROM: Unit<f64> = Unit { 
    alias: "Å", 
    value: 1e-10 * METER.value 
};

pub const MILE: Unit<f64> = Unit { 
    alias: "mi", 
    value: 1609.344 * METER.value 
};

pub const YARD: Unit<f64> = Unit { 
    alias: "yd", 
    value: 0.9144 * METER.value 
};

pub const FOOT: Unit<f64> = Unit { 
    alias: "ft", 
    value: 0.3048 * METER.value 
};

pub const INCH: Unit<f64> = Unit { 
    alias: "in", 
    value: 0.0254 * METER.value 
};

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_non_si_units() {
        assert_relative_eq!(MINUTE.value, 60.0);
        assert_relative_eq!(HOUR.value, 3600.0);
        assert_relative_eq!(GRAM.value, 0.001);
        assert_relative_eq!(BAR.value, 100000.0);
    }
}