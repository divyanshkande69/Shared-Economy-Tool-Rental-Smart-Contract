# Shared Economy Tool Rental Smart Contract

## Table of Contents
- [Project Title](#project-title)
- [Project Description](#project-description)
- [Project Vision](#project-vision)
- [Key Features](#key-features)
- [Contract Details](#contract-details)

## Project Title

**Shared Economy Tool Rental**

## Project Description

The **Shared Economy Tool Rental** smart contract enables individuals to rent tools to each other within a community-based platform. The system allows tool owners to list their tools for rent, and renters can request to rent tools for a specified period and at a given price. The platform facilitates secure transactions and ensures both parties fulfill their obligations through status updates.

## Project Vision

Our vision is to create a decentralized platform for the rental of tools and equipment in a shared economy model, empowering individuals to lend and borrow tools with transparency, fairness, and security. This will reduce costs, promote sustainability, and create a community-driven economy where resources are shared efficiently.

## Key Features

- **Create Rental Requests**: Renters can request to rent tools from owners, specifying the rental duration and price.
- **Rental Acceptance**: Tool owners can accept rental requests, confirming availability and terms.
- **Rental Completion**: Renters can mark the rental as complete when the tool is returned, ensuring both parties fulfill their obligations.
- **Tracking and Transparency**: All rental transactions are tracked on-chain, ensuring transparency and accountability.

## Contract Details

Contract Details: CCDWRA5UG6EAIBH3PGNFKX62UM7V4GIRLBQYV6YHA5QXFP7E3ZP3FJPG 

The smart contract is designed to manage tool rentals with the following functions:

1. **`request_rental`**: Renters can create a rental request, providing details such as tool name, rental price, and duration.
2. **`accept_rental`**: Tool owners can accept rental requests, changing the status of the rental to "Accepted."
3. **`complete_rental`**: Renters can complete the rental when the tool is returned, changing the status to "Completed."
4. **`get_rental`**: Allows anyone to query the details of a specific rental using the rental ID.

The smart contract uses the Soroban SDK to implement these functionalities, ensuring that the rental transactions are secure, transparent, and automated.

---
