#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// DairyFarm struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct DairyFarm {
    id: u64,
    name: String,
    location: String,
    owner: String,
    contact: String,
    email: String,
    total_cows: u64,
    created_at: u64,
}

// DairyAnimal struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct DairyAnimal {
    id: u64,
    dairy_farm_id: u64,
    tag_number: String,
    breed: String,
    age: u64,
    milk_yield: f64, // daily milk yield in liters
    health_status: String,
}

// MilkProduction struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct MilkProduction {
    id: u64,
    dairy_farm_id: u64,
    date: u64,
    total_milk: f64, // total milk produced in liters
    sold_milk: f64,
    remaining_milk: f64,
    revenue: f64,
}

// Expense struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Expense {
    id: u64,
    dairy_farm_id: u64,
    date: u64,
    category: String,
    amount: f64,
    description: String,
}

// Feed struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Feed {
    id: u64,
    dairy_farm_id: u64,
    feed_type: String,
    quantity: f64, // in kilograms
    cost: f64,
    date: u64,
}

// Payload structs
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CreateDairyFarmPayload {
    name: String,
    location: String,
    owner: String,
    contact: String,
    email: String,
    total_cows: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct RegisterDairyAnimalPayload {
    dairy_farm_id: u64,
    tag_number: String,
    breed: String,
    age: u64,
    milk_yield: f64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct RecordMilkProductionPayload {
    dairy_farm_id: u64,
    total_milk: f64,
    sold_milk: f64,
    revenue_per_liter: f64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct RecordExpensePayload {
    dairy_farm_id: u64,
    category: String,
    amount: f64,
    description: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct RecordFeedPayload {
    dairy_farm_id: u64,
    feed_type: String,
    quantity: f64,
    cost: f64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
enum Message {
    Success(String),
    Error(String),
    NotFound(String),
    InvalidPayload(String),
}

// Implementing Storable for DairyFarm
impl Storable for DairyFarm {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for DairyFarm {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for DairyAnimal
impl Storable for DairyAnimal {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for DairyAnimal {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for MilkProduction
impl Storable for MilkProduction {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for MilkProduction {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for Expense
impl Storable for Expense {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Expense {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for Feed
impl Storable for Feed {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Feed {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Memory management
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static DAIRY_FARMS: RefCell<StableBTreeMap<u64, DairyFarm, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(10)))
        ));

    static DAIRY_ANIMALS: RefCell<StableBTreeMap<u64, DairyAnimal, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(11)))
        ));

    static MILK_PRODUCTIONS: RefCell<StableBTreeMap<u64, MilkProduction, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(12)))
        ));

    static EXPENSES: RefCell<StableBTreeMap<u64, Expense, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(13)))
        ));

    static FEEDS: RefCell<StableBTreeMap<u64, Feed, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(14)))
        ));
}

// Functions

// Create Dairy Farm
#[ic_cdk::update]
fn create_dairy_farm(payload: CreateDairyFarmPayload) -> Result<DairyFarm, Message> {
    if payload.name.is_empty() || payload.contact.is_empty() || payload.email.is_empty() {
        return Err(Message::InvalidPayload("Missing required fields".to_string()));
    }

    let dairy_farm_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let dairy_farm = DairyFarm {
        id: dairy_farm_id,
        name: payload.name,
        location: payload.location,
        owner: payload.owner,
        contact: payload.contact,
        email: payload.email,
        total_cows: payload.total_cows,
        created_at: time(),
    };

    DAIRY_FARMS.with(|farms| {
        farms.borrow_mut().insert(dairy_farm_id, dairy_farm.clone());
    });

    Ok(dairy_farm)
}

// Register Dairy Animal
#[ic_cdk::update]
fn register_dairy_animal(payload: RegisterDairyAnimalPayload) -> Result<DairyAnimal, Message> {
    if payload.tag_number.is_empty() || payload.breed.is_empty() {
        return Err(Message::InvalidPayload("Missing required fields".to_string()));
    }

    let farm_exists = DAIRY_FARMS.with(|farms| farms.borrow().contains_key(&payload.dairy_farm_id));
    if !farm_exists {
        return Err(Message::NotFound("Dairy farm not found".to_string()));
    }

    let animal_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let dairy_animal = DairyAnimal {
        id: animal_id,
        dairy_farm_id: payload.dairy_farm_id,
        tag_number: payload.tag_number,
        breed: payload.breed,
        age: payload.age,
        milk_yield: payload.milk_yield,
        health_status: "Healthy".to_string(),
    };

    DAIRY_ANIMALS.with(|animals| {
        animals.borrow_mut().insert(animal_id, dairy_animal.clone());
    });

    Ok(dairy_animal)
}

// Record Milk Production
#[ic_cdk::update]
fn record_milk_production(payload: RecordMilkProductionPayload) -> Result<MilkProduction, Message> {
    if payload.total_milk == 0.0 || payload.sold_milk > payload.total_milk {
        return Err(Message::InvalidPayload("Invalid milk production data".to_string()));
    }

    let farm_exists = DAIRY_FARMS.with(|farms| farms.borrow().contains_key(&payload.dairy_farm_id));
    if !farm_exists {
        return Err(Message::NotFound("Dairy farm not found".to_string()));
    }

    let production_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let remaining_milk = payload.total_milk - payload.sold_milk;
    let revenue = payload.sold_milk * payload.revenue_per_liter;

    let production = MilkProduction {
        id: production_id,
        dairy_farm_id: payload.dairy_farm_id,
        date: time(),
        total_milk: payload.total_milk,
        sold_milk: payload.sold_milk,
        remaining_milk,
        revenue,
    };

    MILK_PRODUCTIONS.with(|productions| {
        productions.borrow_mut().insert(production_id, production.clone());
    });

    Ok(production)
}

// Record Expense
#[ic_cdk::update]
fn record_expense(payload: RecordExpensePayload) -> Result<Expense, Message> {
    if payload.amount <= 0.0 {
        return Err(Message::InvalidPayload("Invalid expense amount".to_string()));
    }

    let farm_exists = DAIRY_FARMS.with(|farms| farms.borrow().contains_key(&payload.dairy_farm_id));
    if !farm_exists {
        return Err(Message::NotFound("Dairy farm not found".to_string()));
    }

    let expense_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let expense = Expense {
        id: expense_id,
        dairy_farm_id: payload.dairy_farm_id,
        date: time(),
        category: payload.category,
        amount: payload.amount,
        description: payload.description,
    };

    EXPENSES.with(|expenses| {
        expenses.borrow_mut().insert(expense_id, expense.clone());
    });

    Ok(expense)
}

// Record Feed
#[ic_cdk::update]
fn record_feed(payload: RecordFeedPayload) -> Result<Feed, Message> {
    if payload.quantity <= 0.0 || payload.cost <= 0.0 {
        return Err(Message::InvalidPayload("Invalid feed data".to_string()));
    }

    let farm_exists = DAIRY_FARMS.with(|farms| farms.borrow().contains_key(&payload.dairy_farm_id));
    if !farm_exists {
        return Err(Message::NotFound("Dairy farm not found".to_string()));
    }

    let feed_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let feed = Feed {
        id: feed_id,
        dairy_farm_id: payload.dairy_farm_id,
        feed_type: payload.feed_type,
        quantity: payload.quantity,
        cost: payload.cost,
        date: time(),
    };

    FEEDS.with(|feeds| {
        feeds.borrow_mut().insert(feed_id, feed.clone());
    });

    Ok(feed)
}

// Calculate Total Revenue
#[ic_cdk::query]
fn calculate_total_revenue(dairy_farm_id: u64) -> Result<f64, Message> {
    let farm_exists = DAIRY_FARMS.with(|farms| farms.borrow().contains_key(&dairy_farm_id));
    if !farm_exists {
        return Err(Message::NotFound("Dairy farm not found".to_string()));
    }

    let total_revenue: f64 = MILK_PRODUCTIONS.with(|productions| {
        productions
            .borrow()
            .iter()
            .filter(|(_, production)| production.dairy_farm_id == dairy_farm_id)
            .map(|(_, production)| production.revenue)
            .sum()
    });

    Ok(total_revenue)
}

// Calculate Total Expenses
#[ic_cdk::query]
fn calculate_total_expenses(dairy_farm_id: u64) -> Result<f64, Message> {
    let farm_exists = DAIRY_FARMS.with(|farms| farms.borrow().contains_key(&dairy_farm_id));
    if !farm_exists {
        return Err(Message::NotFound("Dairy farm not found".to_string()));
    }

    let total_expenses: f64 = EXPENSES.with(|expenses| {
        expenses
            .borrow()
            .iter()
            .filter(|(_, expense)| expense.dairy_farm_id == dairy_farm_id)
            .map(|(_, expense)| expense.amount)
            .sum()
    });

    Ok(total_expenses)
}

// Calculate Total Feed Cost
#[ic_cdk::query]
fn calculate_total_feed_cost(dairy_farm_id: u64) -> Result<f64, Message> {
    let farm_exists = DAIRY_FARMS.with(|farms| farms.borrow().contains_key(&dairy_farm_id));
    if !farm_exists {
        return Err(Message::NotFound("Dairy farm not found".to_string()));
    }

    let total_feed_cost: f64 = FEEDS.with(|feeds| {
        feeds
            .borrow()
            .iter()
            .filter(|(_, feed)| feed.dairy_farm_id == dairy_farm_id)
            .map(|(_, feed)| feed.cost)
            .sum()
    });

    Ok(total_feed_cost)
}

// Exporting the candid interface
ic_cdk::export_candid!();
