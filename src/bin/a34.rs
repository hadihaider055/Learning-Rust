// Topic: Typestates
//
// Summary:
//   An airline wants to reduce the amount of lost luggage by
//   ensuring luggage is properly tracked.
//
// Requirements:
// * Implement a luggage tracking system using the typestate pattern
// * Each piece of luggage has a tracking id
// * Luggage goes through multiple states at the airport:
//   * Check-in        (passenger gives luggage to airport)
//   * OnLoading       (luggage is loaded onto correct plane)
//   * Offloading      (luggage is taken off plane at destination)
//   * AwaitingPickup  (luggage is at destination waiting for passenger pickup)
//   * EndCustody      (luggage was picked up by passenger)
// Notes:
// * Optionally use generics for each state

#[derive(Copy, Clone)]
struct LuggageId(usize);
struct Luggage(LuggageId);
struct CheckIn(LuggageId);
struct OnLoad(LuggageId);
struct OffLoad(LuggageId);
struct AwaitingPickup(LuggageId);
struct EndCustody(LuggageId);

impl Luggage {
    fn new(id: LuggageId) -> Self {
        Luggage(id)
    }

    fn check_in(self) -> CheckIn {
        CheckIn(self.0)
    }
}

impl CheckIn {
    fn on_load(self) -> OnLoad {
        OnLoad(self.0)
    }
}

impl OnLoad {
    fn off_load(self) -> OffLoad {
        OffLoad(self.0)
    }
}

impl OffLoad {
    fn carousel(self) -> AwaitingPickup {
        AwaitingPickup(self.0)
    }
}

impl AwaitingPickup {
    fn pickup(self) -> (Luggage, EndCustody) {
        (Luggage(self.0), EndCustody(self.0))
    }
}

fn main() {
    const ID: LuggageId = LuggageId(09000);
    let luggage = Luggage::new(ID);
    let luggage = luggage.check_in().on_load().off_load().carousel();
    luggage.pickup();
}

// METHOD # 2
// #[derive(Debug)]
// struct Luggage<State> {
//     tracking_id: String,
//     state: State,
// }
// #[derive(Debug)]
// struct CheckIn;
// struct OnLoading;
// struct OffLoading;
// struct AwaitingPickup;
// struct EndCustody;

// impl<State> Luggage<State> {
//     pub fn luggage_status<NextState>(self, state: NextState) -> Luggage<NextState> {
//         print!("{:?} dasds {:?}", state, self);
//         Luggage {
//             tracking_id: self.tracking_id,
//             state,
//         }
//     }
// }

// impl CheckIn {
//     fn check_in(tracking_id: String) -> Luggage<CheckIn> {
//         Luggage {
//             tracking_id,
//             state: CheckIn,
//         }
//     }
// }

// impl Luggage<OnLoading> {
//     fn on_loading(tracking_id: String) -> Self {
//         Luggage {
//             tracking_id,
//             state: OnLoading,
//         }
//     }
// }
// impl Luggage<OffLoading> {
//     fn off_loading(tracking_id: String) -> Self {
//         Luggage {
//             tracking_id,
//             state: OffLoading,
//         }
//     }
// }
// impl Luggage<AwaitingPickup> {
//     fn awaiting_pickup(tracking_id: String) -> Self {
//         Luggage {
//             tracking_id,
//             state: AwaitingPickup,
//         }
//     }
// }
// impl Luggage<EndCustody> {
//     fn end_custody(tracking_id: String) -> Self {
//         Luggage {
//             tracking_id,
//             state: EndCustody,
//         }
//     }
// }
