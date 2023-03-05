struct Planet {
    water_percent: f64,
    pollution_percent: f64,
    best_continent: Continent,
    name: String,
}
struct Continent {
    best_country: Country,
    name: String,
    size: u64,
}
struct Country {
    best_city: City,
    name: String,
    army_power: u64,
}
struct City {
    name: String,
    population: u64,
    gdp: u64
}
fn main() {
    let new_york = City {
        name: String::from("New York"),
        population: 10000,
        gdp: 1_000_000_000,
    };
    let united_states = Country {
        name: String::from("United Status"),
        best_city: new_york,
        army_power: 1_000_000_000
    };
    let north_america = Continent {
        best_country: united_states,
        name: String::from("North America"), 
        size: 1_000_000_000_000,
    };
    let earth = Planet {
       best_continent: north_america,
       water_percent: 70.0,
       pollution_percent: 95.0,
       name: String::from("earth")
    };
    println!("Best_City: {}, Best_Country: {}, Best_Continent: {}, Best_Planet: {}", earth.best_continent.best_country.best_city.name, earth.best_continent.best_country.name, earth.best_continent.name, earth.name);
    println!("Sum: {}", earth.water_percent as u64 + earth.best_continent.size + earth.best_continent.best_country.army_power + earth.best_continent.best_country.best_city.gdp);
}
