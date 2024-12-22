# Dairy Farm Management System

The Dairy Farm Management System is a robust application designed to streamline and manage operations in a dairy farming environment. It includes functionalities to manage dairy farms, dairy animals, milk production, expenses, and feed records, along with various calculations and queries for efficient farm operations.

## Features

### 1. Dairy Farm Management
- Create and manage dairy farms with details such as name, location, owner, contact, email, and total cows.

### 2. Dairy Animal Management
- Register dairy animals, including details like tag number, breed, age, milk yield, and health status.

### 3. Milk Production Tracking
- Record daily milk production with details on total milk produced, sold milk, remaining milk, and revenue generated.

### 4. Expense Management
- Record expenses by category, amount, and description for better financial tracking.

### 5. Feed Management
- Record feed usage with details on feed type, quantity, and cost.
- Track total feed costs for the farm.

### 6. Financial Calculations
- Calculate total revenue from milk sales.
- Calculate total expenses incurred by the farm.
- Calculate total feed costs.

### 7. Query and Reporting
- Retrieve detailed information about farms, dairy animals, milk production records, expenses, and feeds.

---

## Data Structures

### DairyFarm
Holds information about a dairy farm, such as:
- `id`, `name`, `location`, `owner`, `contact`, `email`, `total_cows`, `created_at`.

### DairyAnimal
Represents a dairy animal with attributes:
- `id`, `dairy_farm_id`, `tag_number`, `breed`, `age`, `milk_yield`, `health_status`.

### MilkProduction
Records daily milk production:
- `id`, `dairy_farm_id`, `date`, `total_milk`, `sold_milk`, `remaining_milk`, `revenue`.

### Expense
Tracks financial expenses:
- `id`, `dairy_farm_id`, `date`, `category`, `amount`, `description`.

### Feed
Records feed usage and cost:
- `id`, `dairy_farm_id`, `feed_type`, `quantity`, `cost`, `date`.

---

## Endpoints

### Create Operations
1. **Create Dairy Farm**: `create_dairy_farm(payload)`
2. **Register Dairy Animal**: `register_dairy_animal(payload)`
3. **Record Milk Production**: `record_milk_production(payload)`
4. **Record Expense**: `record_expense(payload)`
5. **Record Feed**: `record_feed(payload)`

### Query Operations
1. **Calculate Total Revenue**: `calculate_total_revenue(dairy_farm_id)`
2. **Calculate Total Expenses**: `calculate_total_expenses(dairy_farm_id)`
3. **Calculate Total Feed Cost**: `calculate_total_feed_cost(dairy_farm_id)`

---


## Requirements
* rustc 1.64 or higher
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ source "$HOME/.cargo/env"
```
* rust wasm32-unknown-unknown targetz
```bash
$ rustup target add wasm32-unknown-unknown
```
* candid-extractor
```bash
$ cargo install candid-extractor
```
* install `dfx`
```bash
$ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
$ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
$ source ~/.bashrc
$ dfx start --background
```

If you want to start working on your project right away, you might want to try the following commands:

```bash
$ cd icp_rust_boilerplate/
$ dfx help
$ dfx canister --help
```

## Update dependencies

update the `dependencies` block in `/src/{canister_name}/Cargo.toml`:
```
[dependencies]
candid = "0.9.9"
ic-cdk = "0.11.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1.0"
ic-stable-structures = { git = "https://github.com/lwshang/stable-structures.git", branch = "lwshang/update_cdk"}
```

## did autogenerate

Add this script to the root directory of the project:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh
```

Update line 16 with the name of your canister:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh#L16
```

After this run this script to generate Candid.
Important note!

You should run this script each time you modify/add/remove exported functions of the canister.
Otherwise, you'll have to modify the candid file manually.

Also, you can add package json with this content:
```
{
    "scripts": {
        "generate": "./did.sh && dfx generate",
        "gen-deploy": "./did.sh && dfx generate && dfx deploy -y"
      }
}
```

and use commands `npm run generate` to generate candid or `npm run gen-deploy` to generate candid and to deploy a canister.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
$ dfx start --background

# Deploys your canisters to the replica and generates your candid interface
$ dfx deploy
```