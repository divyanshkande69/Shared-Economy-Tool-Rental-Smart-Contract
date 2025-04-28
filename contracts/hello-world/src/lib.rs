#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Address, String, symbol_short};

// ToolRental structure to store the details of the tool rental
#[contracttype]
#[derive(Clone, PartialEq)]
pub struct ToolRental {
    pub rental_id: u64,
    pub tool_name: String,
    pub renter: Address,
    pub owner: Address,
    pub price_per_day: u64,    // Price in the smallest unit of currency
    pub rental_duration: u64,  // Duration of rental in days
    pub rental_status: RentalStatus,
}

// Enum to represent the current status of a rental
#[contracttype]
#[derive(Clone, PartialEq)]
pub enum RentalStatus {
    Requested,   // The rental is requested but not accepted yet
    Accepted,    // The rental has been accepted
    Completed,   // The rental has been completed and tool returned
    Cancelled,   // The rental request was cancelled
}

// Contract for the Shared Economy Tool Rental platform
#[contract]
pub struct SharedEconomyToolRental;

#[contractimpl]
impl SharedEconomyToolRental {
    // Create a new rental request
    pub fn request_rental(
        env: Env,
        renter: Address,
        owner: Address,
        tool_name: String,
        price_per_day: u64,
        rental_duration: u64,
    ) -> u64 {
        // Authenticate the renter
        renter.require_auth();

        // Validate the rental parameters
        if price_per_day == 0 || rental_duration == 0 {
            log!(&env, "Invalid rental parameters");
            panic!("Invalid rental parameters");
        }

        // Generate a unique rental ID
        let rental_id = env.ledger().timestamp(); // Use timestamp as rental ID for simplicity

        // Create new rental request
        let rental = ToolRental {
            rental_id,
            tool_name,
            renter: renter.clone(),
            owner: owner.clone(),
            price_per_day,
            rental_duration,
            rental_status: RentalStatus::Requested,
        };

        // Store the rental request
        env.storage().instance().set(&rental_id, &rental);

        log!(&env, "Rental request created with ID: {}", rental_id);

        rental_id
    }

    // Accept a rental request by the owner
    pub fn accept_rental(env: Env, owner: Address, rental_id: u64) -> bool {
        // Authenticate the owner
        owner.require_auth();

        // Get the rental request
        let mut rental = Self::get_rental(env.clone(), rental_id);

        // Validate the rental status
        if rental.rental_status != RentalStatus::Requested {
            log!(&env, "Rental request is not in requested status");
            return false;
        }

        // Ensure the caller is the owner
        if rental.owner != owner {
            log!(&env, "Only the owner can accept this rental request");
            return false;
        }

        // Update rental status to Accepted
        rental.rental_status = RentalStatus::Accepted;

        // Store the updated rental
        env.storage().instance().set(&rental_id, &rental);

        log!(&env, "Rental request ID: {} accepted by owner", rental_id);

        true
    }

    // Complete the rental by returning the tool
    pub fn complete_rental(env: Env, renter: Address, rental_id: u64) -> bool {
        // Authenticate the renter
        renter.require_auth();

        // Get the rental request
        let mut rental = Self::get_rental(env.clone(), rental_id);

        // Validate the rental status
        if rental.rental_status != RentalStatus::Accepted {
            log!(&env, "Rental request is not in accepted status");
            return false;
        }

        // Ensure the caller is the renter
        if rental.renter != renter {
            log!(&env, "Only the renter can complete the rental");
            return false;
        }

        // Update rental status to Completed
        rental.rental_status = RentalStatus::Completed;

        // Store the updated rental
        env.storage().instance().set(&rental_id, &rental);

        log!(&env, "Rental ID: {} completed", rental_id);

        true
    }

    // Get the details of a rental by rental ID
    pub fn get_rental(env: Env, rental_id: u64) -> ToolRental {
        env.storage().instance().get(&rental_id).unwrap_or(ToolRental {
            rental_id: 0,
            tool_name: String::from_str(&env, ""),
            renter: Address::from_str(&env ,"GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"),
            owner: Address::from_str(&env ,"GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"),
            price_per_day: 0,
            rental_duration: 0,
            rental_status: RentalStatus::Cancelled,
        })
    }
}
