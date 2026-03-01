from typing import Any


class UnitError(Exception):
    """Ошибка недопустимых операций с единицами измерения"""

    pass


class Unit:
    """Единица измерения"""

    __slots__ = ("alias", "value")

    NUMBERS = (float, int)

    def __init__(self, alias: str, value: float) -> None:
        """Инициализатор единицы измерения"""
        if not isinstance(alias, str):
            raise TypeError(f"{type(alias)=} must be str")
        self.alias = alias
        if not isinstance(value, self.NUMBERS):
            raise TypeError(f"{type(value)=} must be numeric")
        self.value = value

    def __repr__(self) -> str:
        return f"{self.alias} {self.value}"

    @staticmethod
    def raise_error(other: Any, operation: str) -> None:
        """Единый метод для выброса ошибки"""
        raise UnitError(f"Операция '{operation}' не поддерживается между Unit и {type(other).__name__}")

    def __mul__(self, other) -> float:
        if isinstance(other, self.NUMBERS):
            return self.value * other
        elif isinstance(other, Unit):
            return self.value * other.value
        else:
            self.raise_error("*", other)

    def __rmul__(self, other) -> float:
        if isinstance(other, self.NUMBERS):
            return other * self.value
        elif isinstance(other, Unit):
            return self.value * other.value
        else:
            self.raise_error("*", other)

    def __truediv__(self, other):
        if isinstance(other, self.NUMBERS):
            return self.value / other
        elif isinstance(other, Unit):
            return self.value / other.value
        else:
            self.raise_error("/", other)

    def __rtruediv__(self, other):
        if isinstance(other, self.NUMBERS):
            return other / self.value
        elif isinstance(other, Unit):
            return other.value / self.value
        else:
            self.raise_error("/", other)

    def __pow__(self, other):
        if isinstance(other, self.NUMBERS):
            return self.value**other
        else:
            self.raise_error("**", other)


# Базовые единицы СИ
meter = Unit("m", 1.0)
kilogram = Unit("kg", 1.0)
second = Unit("s", 1.0)
ampere = Unit("A", 1.0)
kelvin = Unit("K", 1.0)
mole = Unit("mol", 1.0)
candela = Unit("cd", 1.0)
radian = Unit("rad", 1.0)

# Производные единицы СИ
## Механика
newton = Unit("N", kilogram.value * meter.value / (second.value * second.value))
pascal = Unit("Pa", newton.value / (meter.value * meter.value))
joule = Unit("J", newton.value * meter.value)
watt = Unit("W", joule.value / second.value)
## Электричество и магнетизм
coulomb = Unit("C", second.value * ampere.value)
volt = Unit("V", watt.value / ampere.value)
farad = Unit("F", coulomb.value / volt.value)
ohm = Unit("Ω", volt.value / ampere.value)
siemens = Unit("S", ampere.value / volt.value)
weber = Unit("Wb", volt.value * second.value)
tesla = Unit("T", weber.value / (meter.value * meter.value))
henry = Unit("H", weber.value / ampere.value)
## Оптика
lumen = Unit("lm", candela.value * radian.value)
lux = Unit("lx", lumen.value / (meter.value * meter.value))
## Радиоактивность
becquerel = Unit("Bq", 1.0 / second.value)
gray = Unit("Gy", (meter.value * meter.value) / (second.value * second.value))
sievert = Unit("Sv", (meter.value * meter.value) / (second.value * second.value))
## Химия
katal = Unit("kat", mole.value / second.value)

# Внесистемные единицы измерения
## Время
minute = Unit("min", 60.0 * second.value)
hour = Unit("h", 60.0 * minute.value)
day = Unit("d", 24.0 * hour.value)
## Объём
liter = Unit("L", 0.001 * meter.value * meter.value * meter.value)
## Масса
gram = Unit("g", 0.001 * kilogram.value)
tonne = Unit("t", 1000.0 * kilogram.value)
dalton = Unit("Da", 1.66053906660e-27 * kilogram.value)
atomicMassUnit = Unit("u", dalton.value)
## Давление
bar = Unit("bar", 100000.0 * pascal.value)
atmosphere = Unit("atm", 101325.0 * pascal.value)
mmHg = Unit("mmHg", 133.322 * pascal.value)
torr = Unit("Torr", mmHg.value)
## Энергия
electronVolt = Unit("eV", 1.602176634e-19 * joule.value)
## Мощность
horsepower = Unit("hp", 745.69987158227022 * watt.value)
## Площадь
hectare = Unit("ha", 10000.0 * meter.value * meter.value)
are = Unit("a", 100.0 * meter.value * meter.value)
## Длина
angstrom = Unit("Å", 1e-10 * meter.value)
mile = Unit("mi", 1609.344 * meter.value)
yard = Unit("yd", 0.9144 * meter.value)
foot = Unit("ft", 0.3048 * meter.value)
inch = Unit("in", 0.0254 * meter.value)


if __name__ == "__main__":
    print(f"{25 * meter}")
    print(meter * 25.0)
    print(25 / meter)
    print(meter / 25.0)
    print(meter**2)
