#[cfg(test)]
mod tests {
    use units::*;

    #[test]
    fn test_basic_operations() {
        // Тестирование основных операций
        assert_eq!(METER * 5.0, 5.0);
        assert_eq!(5.0 * METER, 5.0);
        assert_eq!(METER / 2.0, 0.5);
        assert_eq!(10.0 / METER, 10.0);
    }

    #[test]
    fn test_unit_conversion() {
        // Тестирование преобразования единиц
        let inches_per_meter = METER / INCH;
        assert!((inches_per_meter - 39.3701).abs() < 0.0001);

        let feet_per_meter = METER / FOOT;
        assert!((feet_per_meter - 3.28084).abs() < 0.0001);
    }

    #[test]
    fn test_composite_units() {
        // Тестирование составных единиц
        let speed = 100.0 * METER / SECOND;
        assert_eq!(speed, 100.0);

        let kinetic_energy = 0.5 * KILOGRAM * (10.0 * METER / SECOND).powi(2);
        assert_eq!(kinetic_energy, 50.0);
    }
}