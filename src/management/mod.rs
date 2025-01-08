//! Train management

use compartment::{CompartmentType, SeatManagement};
use rand::{seq::SliceRandom, Rng};

pub mod compartment;

/// Seat structure
/// When giving seat number it should be in format CompartmentNumber-SeatNumber like this:
/// compartment 1 seat 3: 13
#[derive(Debug, Clone, PartialEq)]
pub struct Seat {
    pub number: u32,
    pub is_occupied: bool,
}

impl Seat {
    /// Create new seat
    pub fn new(number: u32) -> Self {
        Self {
            number,
            is_occupied: false,
        }
    }
}

/// Wagon type corresponding to [`CompartmentType`]
#[derive(Debug, Clone, PartialEq)]
pub enum WagonType {
    Seat6,
    Seat8,
    OpenSpace,
}

/// Train wagon
#[derive(Debug, Clone, PartialEq)]
pub struct Wagon {
    pub number: u8,
    pub wagon_type: WagonType,
    pub compartments: Vec<CompartmentType>,
}

impl Wagon {
    /// Create new wagon
    pub fn new(wagon_type: WagonType, compartments_count: u8, number: u8) -> Self {
        let mut compartments = Vec::new();

        for i in 1..=compartments_count {
            compartments.push(CompartmentType::new(&wagon_type, i));
        }

        Self {
            number,
            wagon_type,
            compartments,
        }
    }

    /// List all available seats in all compartments
    pub fn list_all_available_seats(&self) -> Vec<Vec<u32>> {
        self.compartments
            .iter()
            .map(|compartment| match compartment {
                CompartmentType::Seat6(comp) => comp.list_available_seats(),
                CompartmentType::Seat8(comp) => comp.list_available_seats(),
                _ => Vec::new(),
            })
            .collect()
    }
}

/// Train structure
#[derive(Debug, Clone, PartialEq)]
pub struct Train {
    pub operator: String,
    pub wagons: Vec<Wagon>,
}

impl Train {
    /// Create new train
    pub fn new(operator: String, wagons: Vec<Wagon>) -> Self {
        Self { operator, wagons }
    }

    /// Generate a random train and occupy around 60% of the seats
    pub fn generate_random_train() -> Self {
        let mut rng = rand::thread_rng();
        let num_wagons = rng.gen_range(9..=15);
        let mut wagons = Vec::new();

        for _ in 0..num_wagons {
            let wagon_type = match rng.gen_range(0..3) {
                0 => WagonType::Seat6,
                1 => WagonType::Seat8,
                _ => WagonType::OpenSpace,
            };
            let compartments_count = rng.gen_range(8..=10);
            let wagon_number = rng.gen_range(1..=100);
            wagons.push(Wagon::new(wagon_type, compartments_count, wagon_number));
        }

        let mut train = Self {
            operator: "Random Train".to_string(),
            wagons,
        };

        // Occupy around 60% of the seats
        train.randomly_occupy_seats(1.0);

        train
    }

    pub fn randomly_occupy_seats(&mut self, occupancy_percentage: f64) {
        let total_seats: usize = self
            .wagons
            .iter()
            .flat_map(|wagon| wagon.list_all_available_seats())
            .count();

        let target_occupied = (total_seats as f64 * occupancy_percentage) as usize;
        let mut occupied = 0;
        let mut rng = rand::thread_rng();

        while occupied < target_occupied {
            let wagon_index = rng.gen_range(0..self.wagons.len());
            let wagon = &self.wagons[wagon_index];
            let compartment_index = rng.gen_range(0..wagon.compartments.len());
            let compartment = &wagon.compartments[compartment_index];
            let available_seats = compartment.list_available_seats();

            if available_seats.is_empty() {
                continue;
            }

            let seat_number = *available_seats.choose(&mut rng).unwrap();
            if let Err(_) = self.occupy_seat(wagon_index, compartment_index, seat_number) {
                continue;
            }

            occupied += 1;
        }
    }

    /// Occupy seat in compartment
    pub fn occupy_seat(
        &mut self,
        wagon_index: usize,
        compartment_index: usize,
        seat_number: u32,
    ) -> Result<(), String> {
        let Some(wagon) = self.wagons.get_mut(wagon_index) else {
            return Err("Invalid wagon index.".to_string());
        };

        let Some(compartment) = wagon.compartments.get_mut(compartment_index) else {
            return Err("Invalid compartment index.".to_string());
        };

        match compartment {
            CompartmentType::Seat6(comp) => comp.occupy_seat(seat_number),
            CompartmentType::Seat8(comp) => comp.occupy_seat(seat_number),
            _ => Err("Cannot occupy seat in this compartment type.".to_string()),
        }
    }

    /// List all available seats
    pub fn list_all_available_seats(&self) -> Vec<Vec<Vec<u32>>> {
        self.wagons
            .iter()
            .map(|wagon| wagon.list_all_available_seats())
            .collect()
    }
}
