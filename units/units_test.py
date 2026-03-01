import pytest

from .units import Unit, UnitError


class TestUnit:
    """Тесты класса Unit"""

    @pytest.mark.parametrize(
        "alias, value",
        [
            ("m", 5.0),  # float
            ("kg", 10),  # int
            ("s", 0),  # zero
            ("m", -3.5),  # negative
        ],
    )
    def test_init(self, alias, value):
        """Тест корректной инициализации с валидными параметрами"""
        unit = Unit(alias, value)
        assert unit.alias == alias
        assert unit.value == value

    @pytest.mark.parametrize(
        "alias, value, Error",
        [
            (123, 123, TypeError),  # not str alias
            ("m", "5.0", TypeError),  # not numeric value
        ],
    )
    def test_init_error(self, alias, value, Error):
        """Тест некорректной инициализации"""
        with pytest.raises(Error):
            Unit(alias, value)

    def test_slots(self):
        """Тест использования __slots__"""
        unit = Unit("m", 5.0)
        with pytest.raises(AttributeError):
            unit.new_attr = "test"

    @pytest.mark.parametrize(
        "value, multiplier, expected",
        [
            (5.0, 2, 10.0),
            (3.5, 1.5, 5.25),
            (-2.0, 3, -6.0),
            (0, 5, 0),
            (10, 0.5, 5.0),
        ],
    )
    def test_mul(self, value, multiplier, expected):
        """Тест умножения"""
        unit = Unit("m", value)

        result = unit * multiplier
        rresult = multiplier * unit
        uresult = unit * unit

        assert result == expected
        assert rresult == expected
        assert uresult == value * value
        assert isinstance(result, (float, int))
        assert isinstance(rresult, (float, int))
        assert isinstance(uresult, (float, int))

    @pytest.mark.parametrize(
        "invalid_operand",
        [
            "string",
            [1, 2, 3],
            {"key": "value"},
            None,
            (1, 2),
        ],
    )
    def test_mul_error(self, invalid_operand):
        """Тест умножения с ошибкой"""
        unit = Unit("m", 5.0)
        with pytest.raises(UnitError):
            unit * invalid_operand
        with pytest.raises(UnitError):
            invalid_operand * unit

    @pytest.mark.parametrize(
        "value, number, expected",
        [
            (10.0, 2, 5.0),
            (7.5, 1.5, 5.0),
            (-6.0, 3, -2.0),
            (5, 2, 2.5),
        ],
    )
    def test_truediv(self, value, number, expected):
        """Тест деления"""
        unit = Unit("m", value)

        result = unit / number
        rresult = number / unit

        assert result == expected
        assert rresult == 1 / expected
        assert isinstance(result, float)
        assert isinstance(rresult, float)

    def test_division_by_zero(self):
        """Тест деления на ноль"""
        unit = Unit("m", 5.0)
        zero_unit = Unit("s", 0)

        with pytest.raises(ZeroDivisionError):
            unit / 0

        with pytest.raises(ZeroDivisionError):
            5 / zero_unit

    @pytest.mark.parametrize(
        "invalid_operand",
        [
            "string",
            [1, 2, 3],
            {"key": "value"},
            None,
        ],
    )
    def test_truediv_error(self, invalid_operand):
        """Тест деления на недопустимый тип"""
        unit = Unit("m", 5.0)
        with pytest.raises(UnitError):
            unit / invalid_operand

    @pytest.mark.parametrize(
        "value, exponent, expected",
        [
            (2.0, 3, 8.0),
            (3.0, 2, 9.0),
            (4.0, 0.5, 2.0),
            (5.0, 0, 1.0),
            (-2.0, 2, 4.0),
        ],
    )
    def test_pow(self, value, exponent, expected):
        """Тест возведения Unit в степень"""
        unit = Unit("m", value)
        result = unit**exponent
        assert result == expected
        assert isinstance(result, float)

    @pytest.mark.parametrize(
        "invalid_operand",
        [
            "string",
            [1, 2, 3],
            {"key": "value"},
            None,
        ],
    )
    def test_pow_error(self, invalid_operand):
        """Тест возведения в степень с недопустимым типом"""
        unit = Unit("m", 2.0)
        with pytest.raises(UnitError):
            unit**invalid_operand

    def test_pow_with_unit_not_supported(self):
        """Тест возведения Unit в степень Unit (не поддерживается)"""
        unit1 = Unit("m", 2.0)
        unit2 = Unit("s", 3.0)

        with pytest.raises(UnitError):
            unit1**unit2

    def test_large_numbers(self):
        """Тест с очень большими числами"""
        unit = Unit("m", 1e308)
        result = unit * 2
        assert result == 2e308

    def test_small_numbers(self):
        """Тест с очень маленькими числами"""
        unit = Unit("m", 1e-308)
        result = unit * 2
        assert result == 2e-308

    def test_chained_operations(self):
        """Тест цепочки операций"""
        unit = Unit("m", 5.0)
        result = (unit * 2) / 4 * 3
        assert result == 7.5
        assert isinstance(result, float)
