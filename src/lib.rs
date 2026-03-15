//! Библиотека для работы с единицами измерения
//! 
//! Предоставляет типы и константы для работы с единицами СИ
//! и внесистемными единицами измерения.

pub mod prefixes;
pub mod unit;
pub mod si;
pub mod non_si;

// Реэкспорт основных типов и констант
pub use unit::{Unit, UnitError, Number};
pub use si::*;
pub use non_si::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operations() {
        let distance = METER * 25.0;
        assert_eq!(distance, 25.0);
        
        let speed = 100.0 / SECOND;
        assert_eq!(speed, 100.0);
        
        let area = METER.pow(2);
        assert_eq!(area, 1.0);
    }

    #[test]
    fn test_unit_composition() {
        let force = KILOGRAM * METER / (SECOND * SECOND);
        assert_eq!(force, NEWTON.value);
        
        let pressure = force / (METER * METER);
        assert_eq!(pressure, PASCAL.value);
    }
}