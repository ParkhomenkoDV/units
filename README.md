# Units

Пакет `units` предоставляет типизированные константы для работы с единицами измерения в системе СИ (Международная система единиц) и внесистемными единицами.

![](./assets/images/units.jpg)

## Install

### Python
```bash
pip install --upgrade git+https://github.com/ParkhomenkoDV/units.git@main
```

### Go
```bash
go get github.com/ParkhomenkoDV/units
```

### Rust
```toml
[dependencies]
units = { git = "https://github.com/ParkhomenkoDV/units.git" }
```

## Usage

### Python

```python
from units import prefixes
from units import meter, second, watt, newton, pascal, joule

def main() {
    # Использование базовых единиц
    distance = 5 * meter
    time = 10 * second
    speed = distance / time
    
    # Использование префиксов
    length = 3 * prefixes["k"] * meter # 3 километра
    power = 2.5 * prefixes["M"] * watt # 2.5 мегаватта
    
    # Использование производных единиц
    force = 10 * newton
    pressure = 2 * pascal
    energy = 100 * joule
    
    # Конвертация между единицами
    meters = 1609.344 * meter
    miles = meters / mile # = 1.0
}
```

### Go

```go
import "github.com/ParkhomenkoDV/units"

func main() {
    // Использование базовых единиц
    distance := 5 * units.Meter
    time := 10 * units.Second
    speed := distance / time
    
    // Использование префиксов
    length := 3 * units.Prefixes["k"] * units.Meter // 3 километра
    power := 2.5 * units.Prefixes["M"] * units.Watt // 2.5 мегаватта
    
    // Использование производных единиц
    force := 10 * units.Newton
    pressure := 2 * units.Pascal
    energy := 100 * units.Joule
    
    // Конвертация между единицами
    meters := 1609.344 * units.Meter
    miles := meters / units.Mile // = 1.0
}
```

### Rust

```rust
use units::{
    // Базовые единицы
    meter, second, 
    // Производные единицы
    watt, newton, pascal, joule,
    // Внесистемные единицы
    mile,
    // Префиксы
    prefixes
};

fn main() {
    // Использование базовых единиц
    let distance = 5.0 * meter;
    let time = 10.0 * second;
    let speed = distance / time; // 0.5
    
    println!("Скорость: {} м/с", speed);
    
    // Использование префиксов
    let length = 3.0 * prefixes::get("k").unwrap() * meter; // 3 километра = 3000 м
    let power = 2.5 * prefixes::get("M").unwrap() * watt;   // 2.5 мегаватта = 2_500_000 Вт
    
    println!("Длина: {} м", length);
    println!("Мощность: {} Вт", power);
    
    // Использование производных единиц
    let force = 10.0 * newton;
    let pressure = 2.0 * pascal;
    let energy = 100.0 * joule;
    
    println!("Сила: {} Н", force);
    println!("Давление: {} Па", pressure);
    println!("Энергия: {} Дж", energy);
    
    // Конвертация между единицами
    let meters = 1609.344 * meter;
    let miles = meters / mile; // = 1.0
    
    println!("{} м = {} миль", meters, miles);
    
    // Дополнительные примеры
    let area = 100.0 * meter * meter;
    println!("Площадь: {} м²", area);
    
    let speed_kmh = 100.0 * (prefixes::get("k").unwrap() * meter) / hour;
    println!("Скорость: {} км/ч", speed_kmh);
}
```

## Base units

- `Meter` - метр (длина)
- `Kilogram` - килограмм (масса)
- `Second` - секунда (время)
- `Ampere` - ампер (сила тока)
- `Kelvin` - кельвин (температура)
- `Mole` - моль (количество вещества)
- `Candela` - кандела (сила света)
- `Radian` - радиан (угол)

## Derived units

### Mechanics:
- `Newton` - ньютон (сила)
- `Pascal` - паскаль (давление)
- `Joule` - джоуль (энергия)
- `Watt` - ватт (мощность)

### Electricity:
- `Coulomb` - кулон (заряд)
- `Volt` - вольт (напряжение)
- `Farad` - фарад (ёмкость)
- `Ohm` - ом (сопротивление)
- `Siemens` - сименс (проводимость)
- `Weber` - вебер (магнитный поток)
- `Tesla` - тесла (магнитная индукция)
- `Henry` - генри (индуктивность)

### Optics:
- `Lumen` - люмен (световой поток)
- `Lux` - люкс (освещённость)

### Radioactivity:
- `Becquerel` - беккерель (активность)
- `Gray` - грей (поглощённая доза)
- `Sievert` - зиверт (эквивалентная доза)

### Chemistry:
- `Katal` - катал (каталитическая активность)

## Non-systemic units

### Time:
- `Minute` - минута (60 с)
- `Hour` - час (3600 с)
- `Day` - сутки (86400 с)

### Volume:
- `Liter` - литр (0.001 м³)

### Mass:
- `Gram` - грамм (0.001 кг)
- `Tonne` - тонна (1000 кг)
- `Dalton`, `AtomicMassUnit` - атомная единица массы

### Pressure:
- `Bar` - бар (10⁵ Па)
- `Atmosphere` - атмосфера (101325 Па)
- `MmHg`, `Torr` - миллиметр ртутного столба

### Energy:
- `ElectronVolt` - электронвольт

### Power:
- `Horsepower` - лошадиная сила

### Square:
- `Hectare` - гектар (10⁴ м²)
- `Are` - ар (100 м²)

### Length:
- `Angstrom` - ангстрем (10⁻¹⁰ м)
- `Mile` - миля
- `Yard` - ярд
- `Foot` - фут
- `Inch` - дюйм