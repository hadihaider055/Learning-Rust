// Topic: Smart Pointers & RefCell
//
// Summary:
//   A vehicle rental company wants to access the rentals available
//   at storefront locations. Create a program that provides access
//   to storefront rentals from the corporate headquarters.
//
// Requirements:
// * Corporate must be able to access the rentals at a storefront
// * Storefronts must be able to rent out vehicles
// * Rentals have the following attributes:
//   - Type of vehicle
//   - Vehicle Identification Number (VIN)
//   - Vehicle status:
//     * Available, Unavailable, Maintenance, Rented
//
// Notes:
// * Use Rc and RefCell to create shared mutable data structures
// * Create at least two rentals and ensure that Corporate and StoreFront
//   can both access the rental information
// * Test your program by changing the vehicle status from both a storefront
//   and from corporate

use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq)]
enum VehicleStatus {
    Available,
    Unavailable,
    Maintenance,
    Rented,
}

enum VehicleType {
    Bus,
    Car,
}

struct Rental {
    vehicle_type: VehicleType,
    vin: String,
    status: VehicleStatus,
}

struct StoreFront(Rc<RefCell<Vec<Rental>>>);

struct Corporate(Rc<RefCell<Vec<Rental>>>);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn update_status() {
        let vehicles = vec![
            Rental {
                vehicle_type: VehicleType::Car,
                vin: "21312".to_owned(),
                status: VehicleStatus::Available,
            },
            Rental {
                vehicle_type: VehicleType::Bus,
                vin: "234156".to_owned(),
                status: VehicleStatus::Maintenance,
            },
        ];
        let vehicles = Rc::new(RefCell::new(vehicles));
        let corporate = Corporate(Rc::clone(&vehicles));
        let store_front = StoreFront(Rc::clone(&vehicles));

        {
            let mut rentals = store_front.0.borrow_mut();
            if let Some(car) = rentals.get_mut(0) {
                assert_eq!(car.status, VehicleStatus::Available);
                car.status = VehicleStatus::Rented;
            }
        }

        {
            let mut rentals = corporate.0.borrow_mut();
            if let Some(car) = rentals.get_mut(0) {
                assert_eq!(car.status, VehicleStatus::Rented);
                car.status = VehicleStatus::Available;
            }
        }

        {
            let mut rentals = store_front.0.borrow();
            if let Some(car) = rentals.get(0) {
                assert_eq!(car.status, VehicleStatus::Available);
            }
        }
    }
}

fn main() {}
