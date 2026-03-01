package units

import "testing"

func TestUnits(t *testing.T) {
	tests := []struct {
		name string
		got  float64
		want float64
	}{
		// Проверка префиксов
		{"1", 5 * Meter, 5_000 * Prefixes["m"] * Meter},
		{"2", Prefixes["k"] * Watt, 1000 * Joule / Second},

		// Базовые единицы
		{"meter", Meter, 1.0},
		{"kilogram", Kilogram, 1.0},
		{"second", Second, 1.0},

		// Производные единицы механики
		{"newton", Newton, Kilogram * Meter / (Second * Second)},
		{"pascal", Pascal, Newton / (Meter * Meter)},
		{"joule", Joule, Newton * Meter},
		{"watt", Watt, Joule / Second},

		// Электричество
		{"coulomb", Coulomb, Second * Ampere},
		{"volt", Volt, Watt / Ampere},
		{"farad", Farad, Coulomb / Volt},
		{"ohm", Ohm, Volt / Ampere},
		{"siemens", Siemens, Ampere / Volt},
		{"weber", Weber, Volt * Second},
		{"tesla", Tesla, Weber / (Meter * Meter)},
		{"henry", Henry, Weber / Ampere},

		// Оптика
		{"lumen", Lumen, Candela * Radian},
		{"lux", Lux, Lumen / (Meter * Meter)},

		// Радиоактивность
		{"becquerel", Becquerel, 1.0 / Second},
		{"gray", Gray, (Meter * Meter) / (Second * Second)},
		{"sievert", Sievert, (Meter * Meter) / (Second * Second)},

		// Химия
		{"katal", Katal, Mole / Second},

		// Время
		{"minute", Minute, 60 * Second},
		{"hour", Hour, 60 * Minute},
		{"day", Day, 24 * Hour},

		// Объём
		{"liter", Liter, 0.001 * Meter * Meter * Meter},

		// Масса
		{"gram", Gram, 0.001 * Kilogram},
		{"tonne", Tonne, 1000 * Kilogram},
		{"dalton", Dalton, 1.66053906660e-27 * Kilogram},
		{"atomic_mass_unit", AtomicMassUnit, Dalton},

		// Давление
		{"bar", Bar, 100000 * Pascal},
		{"atmosphere", Atmosphere, 101325 * Pascal},
		{"mmHg", MmHg, 133.322 * Pascal},
		{"torr", Torr, MmHg},

		// Энергия
		{"electron_volt", ElectronVolt, 1.602176634e-19 * Joule},

		// Мощность
		{"horsepower", Horsepower, 745.69987158227022 * Watt},

		// Площадь
		{"hectare", Hectare, 10000 * Meter * Meter},
		{"are", Are, 100 * Meter * Meter},

		// Длина
		{"angstrom", Angstrom, 1e-10 * Meter},
		{"mile", Mile, 1609.344 * Meter},
		{"yard", Yard, 0.9144 * Meter},
		{"foot", Foot, 0.3048 * Meter},
		{"inch", Inch, 0.0254 * Meter},

		// Комбинации префиксов и единиц
		{"kilometer", Prefixes["k"] * Meter, 1000 * Meter},
		{"millimeter", Prefixes["m"] * Meter, 0.001 * Meter},
		{"kilowatt", Prefixes["k"] * Watt, 1000 * Watt},
		{"megawatt", Prefixes["M"] * Watt, 1000000 * Watt},
		{"nanometer", Prefixes["n"] * Meter, 1e-9 * Meter},
		{"microsecond", Prefixes["u"] * Second, 1e-6 * Second},

		// Сложные выражения
		{"km/h", (Prefixes["k"] * Meter) / Hour, 1000 * Meter / (3600 * Second)},
		{"kW*h", (Prefixes["k"] * Watt) * Hour, 1000 * Watt * 3600 * Second},
		{"g/cm³", Gram / (Prefixes["c"] * Meter * Prefixes["c"] * Meter * Prefixes["c"] * Meter),
			0.001 * Kilogram / (0.01 * 0.01 * 0.01 * Meter * Meter * Meter)},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			if test.got-test.want > 1e-9 {
				t.Errorf("%s: %v != %v", test.name, test.got, test.want)
			}
		})
	}
}
