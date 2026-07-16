/*
 * =========================================================================
 * PROBLEM STATEMENT
 * =========================================================================
 *
 * Design a backend system for a new enterprise application that Flipkart is
 * launching, FlipFit. Flipkart is partnering up with gyms across Bangalore to
 * enter into the fitness space. For the Beta launch the requirements are:
 *
 *  - There are only 2 centers for now - Koramangala and Bellandur. We might
 *    expand to multiple others if we get traction.
 *  - Each center has 6 slots: 3 in the morning of an hour each from 6am to
 *    9am and similarly 3 in the evening from 6pm to 9pm. The centers are open
 *    7 days a week.
 *  - Each slot at a center can have only 2 possible workout variations for
 *    now - Weights and Cardio.
 *  - The number of people that can attend each workout at each slot for a
 *    given station is fixed. Assume default slot capacity as 3 (for every
 *    workout type across centers).
 *  - Same User cannot book at the same center at the same slot and the same
 *    workout type twice.
 *  - User operations: Register, View the workouts for a particular day,
 *    Book a workout (if seats are available in that time slot at that center),
 *    View his/her plan based on day as input.
 *  - For simplicity the workout info will be entered by the Admin only once.
 *  - Bonus: Build an Admin view to modify the workout info at a center/slot
 *    level.
 *
 * Commands / Requirements
 * -----------------------
 *  REGISTER
 *    Input : User name, User email, User Phone, User password
 *    Output: User Id (unique)
 *
 *  VIEW the workouts for a day
 *    Input : Day
 *    Output: Center (Center ID, Center Name), Slot information (Slot ID, Slot
 *            timing, workout type, slot capacity)
 *
 *  BOOKING: Check if the user is registered, check for slot availability and
 *  finally book the slot.
 *    Input : User ID, Center ID, Slot ID, Day
 *    Output (Success): Booking ID, Slot information (Slot ID, Slot timing,
 *            workout type), Center information (Center ID, Center Name)
 *
 *  VIEW user plan according to the day
 *    Input : User ID, Day
 *    Output: User information, Slot information (Slot timing, workout type),
 *            Center information (Center Name)
 *
 *  Bonus Commands
 *    ADMIN Update -> Input: Center Id, Slot Id, Slot Capacity
 *    ADMIN ADD    -> Input: Center Id, Slot timing, Workout type, Slot Capacity
 *
 * Sample Test Cases
 * -----------------
 *  registerUser("Aashi", "abc@gmail.com", "123", "xyz")  => aash123
 *
 *  viewWorkoutsForDay(12-03-20) =>
 *    Center ID: 1, Center Name: Kormangla, Slot ID: 1,
 *    Slot Timing: 06 AM - 07 AM, Workout type: Weights, Slot capacity: 2
 *    ... (one block per slot across all centers) ...
 *
 *  bookWorkout(aash123, 1, 1, 12-03-20) =>
 *    Booking ID: 123, Slot ID: 1, Slot timing: 06 AM - 07 AM,
 *    Workout Type: Weights, Center ID: 1, Center Name: Kormangla
 *
 *  viewUserPlan(aash123, 12-03-20) =>
 *    User Name: Aashi, Center Name: Kormangla,
 *    Slot Timing: 06 AM - 07 AM, Workout type: Weights
 *
 * Constraints
 * -----------
 *  - No database / NoSQL store; use an in-memory store.
 *  - No UI.
 *  - Provide a driver class for demo purposes that executes all commands and
 *    test cases in one place.
 *  - Prioritize compilation, execution and completion.
 *  - Prefer good design over optimization; modular, object-oriented, good
 *    separation of concerns, extensible, graceful edge-case handling, readable.
 * =========================================================================
 */

//! FlipFit — a small in-memory gym-slot booking backend (Beta).
//!
//! Design notes
//! ------------
//! The atomic bookable unit is a `Slot`, which is the combination of
//! (Center, Timing, WorkoutType) and carries its own capacity. This matches
//! the spec's sample output, where each `Slot ID` uniquely identifies one
//! (timing, workout-type) pairing within a center, so a single physical time
//! window such as "06 AM - 07 AM" exists as two bookable slots (Weights and
//! Cardio).
//!
//! Capacity is tracked PER DAY: booking a slot on one day does not consume
//! seats on another. Consequently the "a user cannot book the same slot
//! twice" rule is interpreted per (user, slot, day) — booking the same slot
//! on two different days is allowed.
//!
//! Layers:
//!   * Domain types  — User, Center, Slot, Booking, WorkoutType
//!   * Error type    — FlipFitError (every fallible op returns Result)
//!   * Service       — FlipFit, the in-memory store + business logic
//!   * Driver        — main(), which exercises every command and edge case

#![allow(dead_code)] // some stored fields (email, phone, ...) aren't read in the demo

use std::collections::HashMap;
use std::fmt;

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WorkoutType {
    Weights,
    Cardio,
}

impl fmt::Display for WorkoutType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            WorkoutType::Weights => "Weights",
            WorkoutType::Cardio => "Cardio",
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug, Clone)]
struct User {
    id: String,
    name: String,
    email: String,
    phone: String,
    password: String,
}

#[derive(Debug, Clone)]
struct Center {
    id: u32,
    name: String,
}

/// A single bookable session: a (center, timing, workout-type) with a capacity.
#[derive(Debug, Clone)]
struct Slot {
    id: u32,
    center_id: u32,
    timing: String,
    workout_type: WorkoutType,
    capacity: u32,
}

#[derive(Debug, Clone)]
struct Booking {
    id: u32,
    user_id: String,
    slot_id: u32,
    day: String,
}

// ---------------------------------------------------------------------------
// Errors — so the service fails gracefully instead of panicking
// ---------------------------------------------------------------------------

#[derive(Debug)]
enum FlipFitError {
    EmailAlreadyRegistered(String),
    UserNotFound(String),
    CenterNotFound(u32),
    SlotNotFound(u32),
    SlotNotInCenter { slot_id: u32, center_id: u32 },
    SlotFull(u32),
    DuplicateBooking { user_id: String, slot_id: u32, day: String },
}

impl fmt::Display for FlipFitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FlipFitError::EmailAlreadyRegistered(email) => {
                write!(f, "a user with email '{}' is already registered", email)
            }
            FlipFitError::UserNotFound(id) => write!(f, "no registered user with id '{}'", id),
            FlipFitError::CenterNotFound(id) => write!(f, "no center with id {}", id),
            FlipFitError::SlotNotFound(id) => write!(f, "no slot with id {}", id),
            FlipFitError::SlotNotInCenter { slot_id, center_id } => {
                write!(f, "slot {} does not belong to center {}", slot_id, center_id)
            }
            FlipFitError::SlotFull(id) => {
                write!(f, "slot {} is fully booked for the requested day", id)
            }
            FlipFitError::DuplicateBooking { user_id, slot_id, day } => write!(
                f,
                "user '{}' has already booked slot {} on {}",
                user_id, slot_id, day
            ),
        }
    }
}

impl std::error::Error for FlipFitError {}

// ---------------------------------------------------------------------------
// Read-model / view structs returned to callers (keeps the service decoupled
// from how results are printed or transported)
// ---------------------------------------------------------------------------

#[derive(Debug)]
struct WorkoutView {
    center_id: u32,
    center_name: String,
    slot_id: u32,
    timing: String,
    workout_type: WorkoutType,
    available_capacity: u32,
}

#[derive(Debug)]
struct BookingConfirmation {
    booking_id: u32,
    slot_id: u32,
    timing: String,
    workout_type: WorkoutType,
    center_id: u32,
    center_name: String,
}

#[derive(Debug)]
struct PlanItem {
    user_name: String,
    center_name: String,
    timing: String,
    workout_type: WorkoutType,
}

// ---------------------------------------------------------------------------
// Service — the in-memory store and all business logic
// ---------------------------------------------------------------------------

struct FlipFit {
    users: HashMap<String, User>,
    centers: HashMap<u32, Center>,
    slots: HashMap<u32, Slot>,
    bookings: Vec<Booking>,
    next_center_id: u32,
    next_slot_id: u32,
    next_booking_id: u32,
    user_seq: u32,
}

impl FlipFit {
    fn new() -> Self {
        FlipFit {
            users: HashMap::new(),
            centers: HashMap::new(),
            slots: HashMap::new(),
            bookings: Vec::new(),
            next_center_id: 1,
            next_slot_id: 1,
            next_booking_id: 1,
            user_seq: 1,
        }
    }

    // ----- Registration -------------------------------------------------

    fn register(
        &mut self,
        name: &str,
        email: &str,
        phone: &str,
        password: &str,
    ) -> Result<String, FlipFitError> {
        if self.users.values().any(|u| u.email == email) {
            return Err(FlipFitError::EmailAlreadyRegistered(email.to_string()));
        }
        let id = self.generate_user_id(name);
        self.users.insert(
            id.clone(),
            User {
                id: id.clone(),
                name: name.to_string(),
                email: email.to_string(),
                phone: phone.to_string(),
                password: password.to_string(),
            },
        );
        Ok(id)
    }

    /// Readable, collision-free id: up to 4 lowercase alphanumerics of the
    /// name + a monotonically increasing sequence number (e.g. "aash1").
    fn generate_user_id(&mut self, name: &str) -> String {
        let prefix: String = name
            .chars()
            .filter(|c| c.is_alphanumeric())
            .take(4)
            .collect::<String>()
            .to_lowercase();
        let id = format!("{}{}", prefix, self.user_seq);
        self.user_seq += 1;
        id
    }

    // ----- Admin / setup -------------------------------------------------

    fn add_center(&mut self, name: &str) -> u32 {
        let id = self.next_center_id;
        self.next_center_id += 1;
        self.centers.insert(id, Center { id, name: name.to_string() });
        id
    }

    /// ADMIN ADD — register a new bookable slot at a center.
    fn add_slot(
        &mut self,
        center_id: u32,
        timing: &str,
        workout_type: WorkoutType,
        capacity: u32,
    ) -> Result<u32, FlipFitError> {
        if !self.centers.contains_key(&center_id) {
            return Err(FlipFitError::CenterNotFound(center_id));
        }
        let id = self.next_slot_id;
        self.next_slot_id += 1;
        self.slots.insert(
            id,
            Slot { id, center_id, timing: timing.to_string(), workout_type, capacity },
        );
        Ok(id)
    }

    /// ADMIN UPDATE — change the capacity of an existing slot.
    fn update_slot_capacity(
        &mut self,
        center_id: u32,
        slot_id: u32,
        capacity: u32,
    ) -> Result<(), FlipFitError> {
        let slot = self.slots.get_mut(&slot_id).ok_or(FlipFitError::SlotNotFound(slot_id))?;
        if slot.center_id != center_id {
            return Err(FlipFitError::SlotNotInCenter { slot_id, center_id });
        }
        slot.capacity = capacity;
        Ok(())
    }

    // ----- Queries -------------------------------------------------------

    fn booked_count(&self, slot_id: u32, day: &str) -> u32 {
        self.bookings
            .iter()
            .filter(|b| b.slot_id == slot_id && b.day == day)
            .count() as u32
    }

    /// VIEW workouts for a day — all slots with remaining capacity for `day`.
    fn view_workouts_for_day(&self, day: &str) -> Vec<WorkoutView> {
        let mut slots: Vec<&Slot> = self.slots.values().collect();
        slots.sort_by_key(|s| s.id); // deterministic order for the demo
        slots
            .into_iter()
            .filter_map(|s| {
                let center = self.centers.get(&s.center_id)?;
                let booked = self.booked_count(s.id, day);
                Some(WorkoutView {
                    center_id: center.id,
                    center_name: center.name.clone(),
                    slot_id: s.id,
                    timing: s.timing.clone(),
                    workout_type: s.workout_type,
                    available_capacity: s.capacity.saturating_sub(booked),
                })
            })
            .collect()
    }

    // ----- Booking -------------------------------------------------------

    /// BOOKING — validate user, slot, duplicate, and capacity; then book.
    fn book_workout(
        &mut self,
        user_id: &str,
        center_id: u32,
        slot_id: u32,
        day: &str,
    ) -> Result<BookingConfirmation, FlipFitError> {
        // 1. user must be registered
        if !self.users.contains_key(user_id) {
            return Err(FlipFitError::UserNotFound(user_id.to_string()));
        }
        // 2. slot must exist and belong to the requested center
        let slot = self.slots.get(&slot_id).ok_or(FlipFitError::SlotNotFound(slot_id))?.clone();
        if slot.center_id != center_id {
            return Err(FlipFitError::SlotNotInCenter { slot_id, center_id });
        }
        // 3. no duplicate booking for the same (user, slot, day)
        let already = self
            .bookings
            .iter()
            .any(|b| b.user_id == user_id && b.slot_id == slot_id && b.day == day);
        if already {
            return Err(FlipFitError::DuplicateBooking {
                user_id: user_id.to_string(),
                slot_id,
                day: day.to_string(),
            });
        }
        // 4. capacity check for that day
        if self.booked_count(slot_id, day) >= slot.capacity {
            return Err(FlipFitError::SlotFull(slot_id));
        }
        // 5. commit the booking
        let booking_id = self.next_booking_id;
        self.next_booking_id += 1;
        self.bookings.push(Booking {
            id: booking_id,
            user_id: user_id.to_string(),
            slot_id,
            day: day.to_string(),
        });

        let center = self.centers.get(&center_id).expect("center validated above");
        Ok(BookingConfirmation {
            booking_id,
            slot_id: slot.id,
            timing: slot.timing.clone(),
            workout_type: slot.workout_type,
            center_id: center.id,
            center_name: center.name.clone(),
        })
    }

    /// VIEW user plan for a day.
    fn view_user_plan(&self, user_id: &str, day: &str) -> Result<Vec<PlanItem>, FlipFitError> {
        let user = self.users.get(user_id).ok_or(FlipFitError::UserNotFound(user_id.to_string()))?;
        let mut plan: Vec<PlanItem> = self
            .bookings
            .iter()
            .filter(|b| b.user_id == user_id && b.day == day)
            .filter_map(|b| {
                let slot = self.slots.get(&b.slot_id)?;
                let center = self.centers.get(&slot.center_id)?;
                Some(PlanItem {
                    user_name: user.name.clone(),
                    center_name: center.name.clone(),
                    timing: slot.timing.clone(),
                    workout_type: slot.workout_type,
                })
            })
            .collect();
        plan.sort_by(|a, b| a.timing.cmp(&b.timing));
        Ok(plan)
    }
}

// ---------------------------------------------------------------------------
// Seeding — set up the two Beta centers, each with 6 windows x 2 workout types
// ---------------------------------------------------------------------------

const DEFAULT_CAPACITY: u32 = 3;

fn seed(flip: &mut FlipFit) {
    let timings = [
        "06 AM - 07 AM",
        "07 AM - 08 AM",
        "08 AM - 09 AM",
        "06 PM - 07 PM",
        "07 PM - 08 PM",
        "08 PM - 09 PM",
    ];
    let workout_types = [WorkoutType::Weights, WorkoutType::Cardio];

    for center_name in ["Koramangala", "Bellandur"] {
        let center_id = flip.add_center(center_name);
        for timing in timings.iter() {
            for workout_type in workout_types.iter() {
                flip.add_slot(center_id, timing, *workout_type, DEFAULT_CAPACITY)
                    .expect("center was just created");
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Pretty-printers for the driver (presentation concern lives outside service)
// ---------------------------------------------------------------------------

fn print_workouts(views: &[WorkoutView]) {
    for v in views {
        println!("Center ID: {}", v.center_id);
        println!("Center Name: {}", v.center_name);
        println!("Slot ID: {}", v.slot_id);
        println!("Slot Timing: {}", v.timing);
        println!("Workout type: {}", v.workout_type);
        println!("Slot capacity: {}", v.available_capacity);
        println!();
    }
}

fn print_confirmation(c: &BookingConfirmation) {
    println!("Booking ID: {}", c.booking_id);
    println!("Slot ID: {}", c.slot_id);
    println!("Slot timing: {}", c.timing);
    println!("Workout Type: {}", c.workout_type);
    println!("Center ID: {}", c.center_id);
    println!("Center Name: {}", c.center_name);
}

fn print_plan(items: &[PlanItem]) {
    if items.is_empty() {
        println!("(no bookings for this day)");
        return;
    }
    for p in items {
        println!("User Name: {}", p.user_name);
        println!("Center Name: {}", p.center_name);
        println!("Slot Timing: {}", p.timing);
        println!("Workout type: {}", p.workout_type);
        println!();
    }
}

fn section(title: &str) {
    println!("\n===== {} =====", title);
}

// ---------------------------------------------------------------------------
// Driver — runs every command plus deliberate edge cases
// ---------------------------------------------------------------------------

fn main() {
    let mut flip = FlipFit::new();
    seed(&mut flip);
    let day = "12-03-20";

    // --- REGISTER ---------------------------------------------------------
    section("REGISTER");
    let aashi = flip.register("Aashi", "abc@gmail.com", "123", "xyz").unwrap();
    println!("registerUser(\"Aashi\", ...)  => {}", aashi);
    let rahul = flip.register("Rahul", "rahul@gmail.com", "999", "pw").unwrap();
    println!("registerUser(\"Rahul\", ...)  => {}", rahul);
    let meera = flip.register("Meera", "meera@x.com", "1", "p").unwrap();
    let kabir = flip.register("Kabir", "kabir@x.com", "2", "p").unwrap();
    let sara = flip.register("Sara", "sara@x.com", "3", "p").unwrap();
    let dev = flip.register("Dev", "dev@x.com", "4", "p").unwrap();
    println!("registered also: {}, {}, {}, {}", meera, kabir, sara, dev);
    // edge case: duplicate email
    match flip.register("Imposter", "abc@gmail.com", "000", "pw") {
        Ok(id) => println!("=> {}", id),
        Err(e) => println!("[expected failure] duplicate email: {}", e),
    }

    // --- VIEW WORKFLOWS FOR A DAY ----------------------------------------
    section("VIEW WORKOUTS FOR DAY (showing first 4 of all slots)");
    let views = flip.view_workouts_for_day(day);
    let preview = &views[..views.len().min(4)];
    print_workouts(preview);
    println!("(total slots across all centers: {})", views.len());

    // --- BOOKING ----------------------------------------------------------
    section("BOOKING");
    match flip.book_workout(&aashi, 1, 1, day) {
        Ok(c) => print_confirmation(&c),
        Err(e) => println!("booking failed: {}", e),
    }

    // Fill slot 2 (Koramangala 06-07 AM Cardio, capacity 3) to the brim
    section("FILL A SLOT TO CAPACITY (slot 2, capacity 3)");
    for (uid, label) in [(&meera, "Meera"), (&kabir, "Kabir"), (&sara, "Sara")] {
        match flip.book_workout(uid, 1, 2, day) {
            Ok(c) => println!("{} booked -> Booking ID {}", label, c.booking_id),
            Err(e) => println!("{} failed: {}", label, e),
        }
    }
    match flip.book_workout(&dev, 1, 2, day) {
        Ok(c) => println!("Dev booked -> Booking ID {}", c.booking_id),
        Err(e) => println!("[expected failure] Dev: {}", e),
    }

    // Edge cases around booking
    section("BOOKING EDGE CASES");
    match flip.book_workout(&aashi, 1, 1, day) {
        Ok(c) => println!("=> Booking ID {}", c.booking_id),
        Err(e) => println!("[expected failure] duplicate booking: {}", e),
    }
    match flip.book_workout(&aashi, 1, 1, "13-03-20") {
        Ok(c) => println!("same slot, different day -> Booking ID {}", c.booking_id),
        Err(e) => println!("failed: {}", e),
    }
    match flip.book_workout("ghost999", 1, 3, day) {
        Ok(c) => println!("=> Booking ID {}", c.booking_id),
        Err(e) => println!("[expected failure] unregistered user: {}", e),
    }
    match flip.book_workout(&rahul, 1, 13, day) {
        // slot 13 belongs to center 2, not center 1
        Ok(c) => println!("=> Booking ID {}", c.booking_id),
        Err(e) => println!("[expected failure] slot/center mismatch: {}", e),
    }
    match flip.book_workout(&rahul, 1, 999, day) {
        Ok(c) => println!("=> Booking ID {}", c.booking_id),
        Err(e) => println!("[expected failure] nonexistent slot: {}", e),
    }

    // --- VIEW USER PLAN ---------------------------------------------------
    section("VIEW USER PLAN — Aashi, 12-03-20");
    match flip.view_user_plan(&aashi, day) {
        Ok(plan) => print_plan(&plan),
        Err(e) => println!("{}", e),
    }

    // --- ADMIN (bonus) ----------------------------------------------------
    section("ADMIN UPDATE — set slot 3 capacity to 5");
    flip.update_slot_capacity(1, 3, 5).unwrap();
    if let Some(v) = flip.view_workouts_for_day(day).into_iter().find(|v| v.slot_id == 3) {
        println!("slot 3 capacity now: {}", v.available_capacity);
    }
    match flip.update_slot_capacity(2, 1, 10) {
        Ok(_) => println!("updated"),
        Err(e) => println!("[expected failure] wrong center for slot: {}", e),
    }

    section("ADMIN ADD — new slot at Bellandur (center 2)");
    match flip.add_slot(2, "09 PM - 10 PM", WorkoutType::Cardio, 4) {
        Ok(id) => println!("new slot id: {}", id),
        Err(e) => println!("failed: {}", e),
    }
    match flip.add_slot(99, "09 PM - 10 PM", WorkoutType::Cardio, 4) {
        Ok(id) => println!("new slot id: {}", id),
        Err(e) => println!("[expected failure] add to nonexistent center: {}", e),
    }

    section("DEMO COMPLETE");
}