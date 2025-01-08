//! Compartment management

use super::{Seat, WagonType};

/// SeatManagement trait
pub trait SeatManagement {
    /// Get rows
    /// # Note
    /// Needed for default implementation
    fn rows(&self) -> &[Vec<Seat>];
    /// Get mutable rows
    /// # Note
    /// Needed for default implementation
    fn rows_mut(&mut self) -> &mut [Vec<Seat>];

    /// Occupies seat
    fn occupy_seat(&mut self, seat_number: u32) -> Result<(), String> {
        for row in self.rows_mut() {
            if let Some(seat) = row.iter_mut().find(|s| s.number == seat_number) {
                if seat.is_occupied {
                    return Err("Seat already occupied.".to_string());
                }
                seat.is_occupied = true;
                return Ok(());
            }
        }

        Err("Invalid seat number.".to_string())
    }

    /// Checks if seat is occupied
    fn is_seat_occupied(&self, seat_number: u32) -> bool {
        self.rows()
            .iter()
            .any(|row| row.iter().any(|s| s.number == seat_number && s.is_occupied))
    }

    /// List available seats
    fn list_available_seats(&self) -> Vec<u32> {
        self.rows()
            .iter()
            .flat_map(|row| row.iter().filter(|s| !s.is_occupied).map(|s| s.number))
            .collect()
    }
}

/// Compartment structure having 6 seats
#[derive(Debug, Clone, PartialEq)]
pub struct Compartment6Seats {
    pub number: u8,
    pub rows: [Vec<Seat>; 3],
}

/// Compartment structure having 8 seats
#[derive(Debug, Clone, PartialEq)]
pub struct Compartment8Seats {
    pub number: u8,
    pub rows: [Vec<Seat>; 4],
}

/// Compartment structure in open space having 8 seats
#[derive(Debug, Clone, PartialEq)]
pub struct CompartmentOpenSpace {
    pub number: u8,
    pub rows: [Vec<Seat>; 4],
}

/// Compartment type combining compartment types
#[derive(Debug, Clone, PartialEq)]
pub enum CompartmentType {
    Seat6(Compartment6Seats),
    Seat8(Compartment8Seats),
    OpenSpace(CompartmentOpenSpace),
}

impl CompartmentType {
    /// Create new compartment
    pub fn new(wagon_type: &WagonType, number: u8) -> Self {
        match wagon_type {
            WagonType::Seat6 => CompartmentType::Seat6(Compartment6Seats::new(number)),
            WagonType::Seat8 => CompartmentType::Seat8(Compartment8Seats::new(number)),
            WagonType::OpenSpace => CompartmentType::OpenSpace(CompartmentOpenSpace::new(number)),
        }
    }

    /// List available seats for the specific compartment
    pub fn list_available_seats(&self) -> Vec<u32> {
        match self {
            CompartmentType::Seat6(comp) => comp.list_available_seats(),
            CompartmentType::Seat8(comp) => comp.list_available_seats(),
            CompartmentType::OpenSpace(comp) => comp.list_available_seats(),
        }
    }
}

impl SeatManagement for Compartment6Seats {
    fn rows(&self) -> &[Vec<Seat>] {
        &self.rows
    }

    fn rows_mut(&mut self) -> &mut [Vec<Seat>] {
        &mut self.rows
    }
}

impl SeatManagement for Compartment8Seats {
    fn rows(&self) -> &[Vec<Seat>] {
        &self.rows
    }

    fn rows_mut(&mut self) -> &mut [Vec<Seat>] {
        &mut self.rows
    }
}

impl SeatManagement for CompartmentOpenSpace {
    fn rows(&self) -> &[Vec<Seat>] {
        &self.rows
    }

    fn rows_mut(&mut self) -> &mut [Vec<Seat>] {
        &mut self.rows
    }
}

impl Compartment6Seats {
    pub fn new(number: u8) -> Self {
        Self {
            number,
            rows: [
                vec![Seat::new(5), Seat::new(6)],
                vec![Seat::new(3), Seat::new(4)],
                vec![Seat::new(1), Seat::new(2)],
            ],
        }
    }
}

impl Compartment8Seats {
    pub fn new(number: u8) -> Self {
        Self {
            number,
            rows: [
                vec![Seat::new(5), Seat::new(6)],
                vec![Seat::new(7), Seat::new(4)],
                vec![Seat::new(3), Seat::new(8)],
                vec![Seat::new(1), Seat::new(2)],
            ],
        }
    }
}

impl CompartmentOpenSpace {
    pub fn new(number: u8) -> Self {
        if number == 1 {
            Self {
                number,
                rows: [
                    vec![Seat::new(6)],
                    vec![Seat::new(4)],
                    vec![Seat::new(3), Seat::new(8)],
                    vec![Seat::new(1), Seat::new(2)],
                ],
            }
        } else if number == 10 {
            Self {
                number,
                rows: [
                    vec![Seat::new(5)],
                    vec![Seat::new(7)],
                    vec![Seat::new(3)],
                    vec![Seat::new(1)],
                ],
            }
        } else {
            Self {
                number,
                rows: Compartment8Seats::new(number).rows,
            }
        }
    }
}
