### The P2P Marketplace Escrow smart contract facilitates secure and efficient transactions between buyers and sellers trading USDC stablecoins for Fiat. The contract acts as an escrow, holding USDC until both parties confirm the transaction.

### User Story

User Story: Buyer on Bebop App P2P Marketplace

As a Buyer who wants to purchase cryptocurrency through Bebop App’s P2P Marketplace, I want a simple, secure, and reliable way to purchase digital assets directly from other users so that I can acquire crypto quickly using my preferred payment method.

Discovery and Selection:

I log into my Bebop App account and navigate to the P2P platform.
I filter by the cryptocurrency I want to buy, as well as my preferred currency and payment method (e.g., bank transfer, PayPal).
I view a list of available sellers along with their rates, completion percentages, limits, and user ratings.
I select a seller who has good ratings, offers my preferred payment method, and has a competitive rate.
Initiating the Trade:

I enter the amount of cryptocurrency I wish to purchase and click “Buy.”
The trade is locked, and the cryptocurrency amount is temporarily held in escrow by a smart contract, ensuring my payment is secure.
Making Payment:

I follow the instructions provided by the seller to make the payment through the agreed payment method.
After sending the payment, I return to the Bebop App and click “Mark as Paid,” signaling the seller to verify the payment
Confirming Receipt:

Once the seller confirms the payment, the smart contract releases the cryptocurrency from escrow and into my wallet.
I receive a notification, check my wallet to ensure the funds are there, and rate the seller based on my experience.
Outcome:

I have successfully purchased cryptocurrency through a quick and secure process.
User Story: Seller on Bebop App P2P Marketplace

As a Seller who wants to exchange cryptocurrency for fiat currency using Bebop App P2PMarketplace, I want to be able to post offers, screen buyers, and securely receive payment before releasing my crypto so that I can confidently transact without risking my assets.

### Setting Up an Offer:

I log into my Bebop App account and navigate to the P2P Marketplace.
I create an offer by setting my cryptocurrency rate, limits, payment methods, and any additional instructions for buyers.
My offer appears in the list of available trades, where potential buyers can find it.
Responding to a Trade Request:

When a buyer initiates a trade, I receive a notification and review the trade details.
The cryptocurrency amount is temporarily held in a smart contract escrow, ensuring that it remains secure until payment is verified.
Verifying Payment:

I wait for the buyer to make the payment and mark it as “Paid.”
I confirm payment in my account through the selected payment method.
After verifying the payment, I release the cryptocurrency from escrow to the buyer’s account.
Rating and Review:

After the trade is complete, I rate the buyer based on our interaction, helping maintain a reputable marketplace.
Outcome:

I have securely exchanged my cryptocurrency for fiat currency and can repeat this process with other buyers on the marketplace.
Key Features

Secure Escrow: Holds USDC in escrow until transaction confirmation.
Role Management: Differentiates between buyers and sellers.
Payment Confirmation: Ensures payment is confirmed before releasing funds.
Time Constraints: Enforces deadlines for payment confirmation.
Dispute Resolution: Mechanism for handling transaction disputes.



# Decentralized Escrow Service

## Project Overview

The Decentralized Escrow Service is a smart contract built on the Soroban platform designed to facilitate secure and transparent peer-to-peer transactions. It acts as a trusted intermediary that holds funds in escrow until the transaction terms are fulfilled, providing a solution for mitigating trust and risk issues in various use cases such as e-commerce, freelance services, and real estate.

### Key Features

- **Secure Fund Management**: Lock funds in escrow until conditions are met.
- **Dispute Resolution**: Mechanisms for raising and resolving disputes with optional arbitration.
- **Auditability**: Transparent logging of all actions for accountability.

### Prerequisites

- Rust and Cargo installed on your system.
- Familiarity with the Soroban SDK and smart contract development.

## Setup & Installation

### Dependencies

Ensure you have the following dependencies in your `Cargo.toml`:


### Build Instructions

1. Clone the repository:
   
2. Build the project:
   
### Deployment Steps

1. Deploy the contract using the Soroban CLI or your preferred deployment tool.
2. Initialize the contract by calling the `initialize` function.

## Usage Guide

### Main Functions

- **create_agreement**: Set up a new transaction agreement with specified terms.
- **deposit_funds**: Deposit funds into escrow for a transaction.
- **release_funds**: Release funds to the seller upon fulfillment of conditions.
- **raise_dispute**: Raise a dispute for a transaction.
- **resolve_dispute**: Resolve a dispute through arbitration.
- **record_action**: Log actions for audit purposes.

### Common Operations

#### Creating an Agreement


#### Depositing Funds


#### Releasing Funds


## Contract Structure

### Key Files

- **src/lib.rs**: Main entry point for the contract.
- **src/agreement.rs**: Manages transaction agreements.
- **src/fund_management.rs**: Handles fund deposits and releases.
- **src/dispute_resolution.rs**: Manages disputes and arbitration.
- **src/audit.rs**: Logs actions for audit purposes.
- **src/types.rs**: Defines data structures.
- **src/errors.rs**: Defines error types.
- **src/utils.rs**: Provides utility functions.

### Main Components

- **Agreement Module**: Handles agreement setup.
- **Fund Management Module**: Manages escrow funds.
- **Dispute Resolution Module**: Handles disputes.
- **Audit Module**: Logs actions for transparency.

## Testing

### Running Tests

Run the tests using Cargo:


### Test Coverage Overview

- **Agreement Tests**: Verify agreement creation and retrieval.
- **Fund Management Tests**: Verify fund deposits and releases.
- **Dispute Resolution Tests**: Verify dispute handling and resolution.
- **Audit Tests**: Verify action logging.
- **Utils Tests**: Verify utility functions.

## Security Considerations

- Ensure proper authorization for sensitive actions.
- Validate inputs to prevent unauthorized access.
- Implement robust error handling.

## Troubleshooting

- Ensure all dependencies are correctly installed.
- Verify the contract is properly deployed and initialized.
- Check logs for detailed error messages.

This documentation provides a comprehensive guide to understanding, setting up, and using the decentralized escrow service smart contract. For further assistance, refer to the Soroban SDK documentation or reach out to the community.
# escrowcotract
