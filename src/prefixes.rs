//! Десятичные приставки СИ
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    /// Таблица десятичных приставок СИ
    pub static ref PREFIXES: HashMap<&'static str, f64> = {
        let mut map = HashMap::new();
        map.insert("q", 1e-30);  // квекто
        map.insert("r", 1e-27);  // ронто
        map.insert("y", 1e-24);  // иокто
        map.insert("z", 1e-21);  // зепто
        map.insert("a", 1e-18);  // атто
        map.insert("f", 1e-15);  // фемто
        map.insert("p", 1e-12);  // пико
        map.insert("n", 1e-9);   // нано
        map.insert("u", 1e-6);   // микро
        map.insert("m", 1e-3);   // милли
        map.insert("c", 1e-2);   // санти
        map.insert("d", 1e-1);   // деци
        map.insert("", 1.0);
        map.insert("da", 1e1);   // дека
        map.insert("h", 1e2);    // гекто
        map.insert("k", 1e3);    // кило
        map.insert("M", 1e6);    // мега
        map.insert("G", 1e9);    // гига
        map.insert("T", 1e12);   // тера
        map.insert("P", 1e15);   // пета
        map.insert("E", 1e18);   // экса
        map.insert("Z", 1e21);   // зетта
        map.insert("Y", 1e24);   // иотта
        map.insert("R", 1e27);   // ронна
        map.insert("Q", 1e30);   // кветта
        map
    };
}

/// Получить множитель для приставки
pub fn get_prefix(prefix: &str) -> Option<f64> {
    PREFIXES.get(prefix).copied()
}