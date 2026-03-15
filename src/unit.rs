//! Основной модуль для работы с единицами измерения
use std::fmt;
use std::ops::{Add, Div, Mul, Sub, Neg};
use num_traits::Num;

/// Ошибка недопустимых операций с единицами измерения
#[derive(Debug, Clone, PartialEq)]
pub struct UnitError(pub String);

impl fmt::Display for UnitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UnitError: {}", self.0)
    }
}

impl std::error::Error for UnitError {}

/// Тип для числовых значений
pub trait Number: Num + Copy + fmt::Display + Into<f64> + From<f64> {}
impl<T> Number for T where T: Num + Copy + fmt::Display + Into<f64> + From<f64> {}

/// Единица измерения
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Unit<T = f64> {
    pub alias: &'static str,
    pub value: T,
}

impl<T: Number> Unit<T> {
    /// Создать новую единицу измерения
    pub fn new(alias: &'static str, value: T) -> Self {
        Unit { alias, value }
    }

    /// Получить значение в базовых единицах
    pub fn value(&self) -> T {
        self.value
    }

    /// Получить алиас единицы
    pub fn alias(&self) -> &'static str {
        self.alias
    }
}

impl<T: Number> fmt::Display for Unit<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.alias, self.value.into())
    }
}

// Арифметические операции с числами
impl<T: Number> Mul<T> for Unit<T> {
    type Output = T;

    fn mul(self, rhs: T) -> Self::Output {
        self.value * rhs
    }
}

impl<T: Number> Div<T> for Unit<T> {
    type Output = T;

    fn div(self, rhs: T) -> Self::Output {
        self.value / rhs
    }
}

impl<T: Number> Mul<Unit<T>> for T {
    type Output = T;

    fn mul(self, rhs: Unit<T>) -> Self::Output {
        self * rhs.value
    }
}

impl<T: Number> Div<Unit<T>> for T {
    type Output = T;

    fn div(self, rhs: Unit<T>) -> Self::Output {
        self / rhs.value
    }
}

// Операции между единицами
impl<T: Number> Mul<Unit<T>> for Unit<T> {
    type Output = T;

    fn mul(self, rhs: Unit<T>) -> Self::Output {
        self.value * rhs.value
    }
}

impl<T: Number> Div<Unit<T>> for Unit<T> {
    type Output = T;

    fn div(self, rhs: Unit<T>) -> Self::Output {
        self.value / rhs.value
    }
}

// Операции со степенью
impl<T: Number> Unit<T> {
    pub fn pow(self, exp: i32) -> T {
        let base: f64 = self.value.into();
        let result = base.powi(exp);
        T::from(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_creation() {
        let meter = Unit::new("m", 1.0);
        assert_eq!(meter.alias(), "m");
        assert_eq!(meter.value(), 1.0);
    }

    #[test]
    fn test_unit_operations() {
        let meter = Unit::new("m", 1.0);
        let second = Unit::new("s", 1.0);
        
        assert_eq!(meter * 5.0, 5.0);
        assert_eq!(meter * second, 1.0);
        assert_eq!(meter / second, 1.0);
        assert_eq!(meter.pow(2), 1.0);
    }
}