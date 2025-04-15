# Detailed Explanation of Worker and Relayer Processes

The system actually sets up four workers with different responsibilities, which together with the relayer form a complete transaction processing flow.

## Four Types of Workers

From `initialize_workers.rs`, you can clearly see the four specialized workers:

1. **Transaction Request Worker** (`TRANSACTION_REQUEST`)
   - Processes initial transaction requests
   - Calls the `transaction_request_handler` function
   - Responsible for the initial validation and handling of transactions

2. **Transaction Submission Worker** (`TRANSACTION_SENDER`)
   - Handles the actual submission of transactions
   - Calls the `transaction_submission_handler` function
   - Responsible for signing transactions and sending them to the blockchain network

3. **Transaction Status Worker** (`TRANSACTION_STATUS_CHECKER`)
   - Monitors the status of submitted transactions
   - Calls the `transaction_status_handler` function
   - Checks if transactions have been confirmed or failed

4. **Notification Worker** (`NOTIFICATION_SENDER`)
   - Handles the sending of notifications
   - Calls the `notification_handler` function
   - Sends webhook notifications when transaction status changes

**Detailed Process**:

1. **Transaction Request Phase**:
   - The user sends a transaction request through the API
   - The Transaction Request Worker receives and validates the request
   - Creates an initial transaction record (Pending status)
   - Generates a Transaction Submission task and adds it to the queue

2. **Transaction Submission Phase**:
   - The Transaction Submission Worker receives the submission task
   - Determines which relayer to use for processing the transaction
   - Prepares the transaction using the relayer (calculating nonce, signing, etc.)
   - Submits the transaction to the blockchain network
   - Updates the transaction status to Submitted
   - Generates a Transaction Status check task

3. **Transaction Status Monitoring Phase**:
   - The Transaction Status Worker periodically checks the transaction status
   - Calls the blockchain RPC to query the transaction receipt
   - Updates the transaction status based on the results (Mined/Confirmed/Failed)
   - If unconfirmed for too long, generates a resubmission task for the Transaction Submission Worker
   - Generates a notification task when the status changes

4. **Notification Phase**:
   - The Notification Worker processes notification tasks
   - Sends transaction status updates to the configured webhook endpoint

### Resubmission Mechanism

When a transaction remains unconfirmed for an extended period:
1. The Transaction Status Worker detects a timeout
2. Generates a resubmission task
3. The Transaction Submission Worker receives the task
4. Calls `calculate_bumped_gas_price` to increase the gas price
5. Resubmits the transaction using the same nonce

## Relationship Between Relayer and Worker

- **Relayer** is the core of blockchain interaction, maintaining identity, private keys, and nonce
- **Worker** is the unit of task execution, handling specific types of tasks in the queue
- Workers **use** Relayers to perform actual blockchain operations
- Each Worker instance can use any available Relayer
- The system uses queues to decouple request handling from actual execution

## Worker Configuration Details

Each Worker is configured with:
- Concurrency (DEFAULT_CONCURRENCY = 2)
- Rate limiting (20 requests per second)
- Error handling layer (ErrorHandlingLayer)
- Retry strategy (BackoffRetryPolicy)
- Application state data (app_state)

This architectural design enables the system to efficiently handle a large number of transactions while maintaining scalability and reliability, with specialized workers processing tasks at different stages.

# How is the nonce reset issue handled in case of transaction failure?

It is not handled; the transaction is directly marked as failed.

# Does one relayer correspond to multiple signers? Or do multiple relayers correspond to one signer?

Good question. From the structure of the configuration file, the relationship here is:
One relayer corresponds to one signer.
From the example configuration file, you can see:
Each relayer has a `signer_id` field that points to a specific signer.
Each signer has a corresponding key file.
```json
{
  "id": "sepolia-example",
  "name": "Sepolia Example",
  "signer_id": "local-signer",   // References a specific signer
  ...
}
```

This way, each relayer will use a different blockchain address to send transactions, which makes it clearer when tracking transactions, managing balances, and nonce. If multiple relayers shared one signer, it would lead to confusion in nonce management as they would share the same sending address.

# What operations are performed when the transaction status is `TransactionStatus::Failed`?

When the transaction status changes to `TransactionStatus::Failed`, the system performs the following operations:

1. **Update Transaction Status**:
   In the `handle_final_state` method, the system calls `update_transaction_status_if_needed` to update the transaction status to `Failed`:
   ```rust
   async fn handle_final_state(
       &self,
       tx: TransactionRepoModel,
       status: TransactionStatus,
   ) -> Result<TransactionRepoModel, TransactionError> {
       self.update_transaction_status_if_needed(tx, status).await
   }
   ```

2. **Update Memory Record**:
   Updates the transaction record in the `update_transaction_status` method:
   ```rust
   let update_request = TransactionUpdateRequest {
       status: Some(new_status),
       confirmed_at: None, // No confirmed_at for Failed status
       ..Default::default()
   };
   
   let updated_tx = self
       .transaction_repository()
       .partial_update(tx.id.clone(), update_request)
       .await?;
   ```

3. **Send Notification**:
   If notifications are configured, the system sends a notification about the transaction failure:
   ```rust
   self.send_transaction_update_notification(&updated_tx).await?;
   ```

4. **No Adjustment of Nonce**:
   Importantly, the system **does not** adjust or reset the used nonce. When a transaction fails:
   - The `decrement` method to decrease the nonce count is not called
   - The system does not automatically resubmit a transaction with the same nonce

5. **No Automatic Retry**:
   - Failed transactions are marked with a final status and do not enter a retry loop
   - If you need to retry the same operation, a new transaction must be submitted (using a new nonce)

This design aligns with the nonce mechanism of blockchains like Ethereum: even if a transaction fails, that nonce is considered used and cannot be reused for another transaction.

If you need to execute specific operations after a transaction fails (such as retry logic), you need to implement those logics at the application level, for example:
1. Monitor the transaction status
2. When `Failed` status is detected, submit a new transaction (using a new nonce)
3. If possible, adjust transaction parameters (such as gas price, gas limit, etc.) to increase the chance of success.

# When should `should_resubmit` be called?

According to the implementation in `src/domain/transaction/evm/status.rs`, it will return `true` only if all the following conditions are met:

1. **The transaction must be in `Submitted` status**:
   ```rust
   if tx.status != TransactionStatus::Submitted {
       return Err(TransactionError::UnexpectedError(format!(
           "Transaction must be in Submitted status to resubmit, found: {:?}",
           tx.status
       )));
   }
   ```

2. **The transaction has been pending long enough**:
   ```rust
   let age = get_age_of_sent_at(tx)?;
   let timeout = match tx.network_data.get_evm_transaction_data() {
       Ok(data) => get_resubmit_timeout_for_speed(&data.speed),
       Err(e) => return Err(e),
   };
   
   let timeout_with_backoff = get_resubmit_timeout_with_backoff(timeout, tx.hashes.len());
   if age > Duration::milliseconds(timeout_with_backoff) {
       info!("Transaction has been pending for too long, resubmitting");
       return Ok(true);
   }
   ```

### Timeout Calculation Logic

1. **Basic timeout is determined by transaction speed**:
   - Use `get_resubmit_timeout_for_speed` to get the basic timeout duration
   - Returns different timeout durations based on the set transaction speed (e.g., `slow`, `medium`, `fast`)

2. **Adding a backoff mechanism**:
   - `get_resubmit_timeout_with_backoff` increases the wait time based on the number of retries
   - The more retries, the longer the wait time, to prevent frequent resubmission
   - The number of retries is judged by `tx.hashes.len()` (each resubmission records a new transaction hash)

### Specific Timeout Durations

From the project's code, the timeout settings for different speeds are roughly as follows:
- `slow`: longer timeout (possibly several tens of minutes)
- `medium`: moderate timeout (a few minutes)
- `fast`: shorter timeout (about 1 minute)

Then, as the number of retries increases, these basic timeout values will be gradually increased, forming a backoff strategy.

### Calling Process

1. The transaction state machine checks if resubmission is needed in `handle_submitted_state`:
   ```rust
   async fn handle_submitted_state(
       &self,
       tx: TransactionRepoModel,
   ) -> Result<TransactionRepoModel, TransactionError> {
       if self.should_resubmit(&tx).await? {
           return self.handle_resubmission(tx).await;
       }
       // ...
   }
   ```

2. If resubmission is needed, it calls `handle_resubmission` to handle it:
   - Possibly creates a NOOP transaction (if needed)
   - Sends the resubmission job

In summary, a resubmission is triggered only when the transaction is in `Submitted` status and has been pending long enough (based on the set speed and number of retries). This is to address the issue of transactions not being confirmed for an extended period due to network congestion or insufficient gas prices.

# Will the gas price be increased and the transaction resubmitted in `handle_resubmission`?

Yes, during the `handle_resubmission` process, the system will increase the gas price for resending the transaction. The specific process is as follows:

1. First, `handle_resubmission` creates the resubmission job:
   ```rust
   async fn handle_resubmission(
       &self,
       tx: TransactionRepoModel,
   ) -> Result<TransactionRepoModel, TransactionError> {
       info!("Scheduling resubmit job for transaction: {}", tx.id);

       let tx_to_process = if self.should_noop(&tx).await? {
           self.process_noop_transaction(&tx).await?
       } else {
           tx
       };

       self.send_transaction_resubmit_job(&tx_to_process).await?;
       Ok(tx_to_process)
   }
   ```

2. When the resubmission job executes, it calls the `resubmit_transaction` method:
   ```rust
   async fn resubmit_transaction(
       &self,
       tx: TransactionRepoModel,
   ) -> Result<TransactionRepoModel, TransactionError> {
       info!("Resubmitting transaction: {:?}", tx.id);

       // Calculate the increased gas price
       let bumped_price_params = self
           .price_calculator
           .calculate_bumped_gas_price(&tx, self.relayer())
           .await?;

       if !bumped_price_params.is_min_bumped.is_some_and(|b| b) {
           warn!(
               "Bumped gas price does not meet minimum requirement, skipping resubmission: {:?}",
               bumped_price_params
           );
           return Ok(tx);
       }

       // Get transaction data
       let evm_data = tx.network_data.get_evm_transaction_data()?;

       // Create new transaction data with increased gas price
       let updated_evm_data = evm_data.with_price_params(bumped_price_params);

       // Sign the transaction
       let sig_result = self
           .signer
           .sign_transaction(NetworkTransactionData::Evm(updated_evm_data.clone()))
           .await?;

       let final_evm_data = updated_evm_data.with_signed_transaction_data(sig_result.into_evm()?);

       // Send the raw transaction
       let raw_tx = final_evm_data.raw.as_ref().ok_or_else(|| {
           TransactionError::InvalidType("Raw transaction data is missing".to_string())
       })?;

       self.provider.send_raw_transaction(raw_tx).await?;

       // Update transaction record
       // ...
   }
   ```

The key point in this process is the **mechanism for increasing the gas price**:

1. **A dedicated method is called to calculate the increased gas price**:
   ```rust
   let bumped_price_params = self
       .price_calculator
       .calculate_bumped_gas_price(&tx, self.relayer())
       .await?;
   ```

2. **Verify if the increase in gas is sufficient**:
   ```rust
   if !bumped_price_params.is_min_bumped.is_some_and(|b| b) {
       warn!("Bumped gas price does not meet minimum requirement, skipping resubmission");
       return Ok(tx);
   }
   ```

3. **Use the higher gas price to create a new transaction**:
   ```rust
   let updated_evm_data = evm_data.with_price_params(bumped_price_params);
   ```

The system maintains the same nonce but increases the gas price, which is the standard method for Ethereum to override previously unconfirmed transactions. The specific increase in gas price is determined by the `price_calculator` component, generally based on the following factors:

- Current network gas price
- Original transaction gas price
- Time that the transaction has been pending
- Previous number of retries
- Network congestion conditions

This mechanism ensures that during network congestion, the transaction's competitiveness can be gradually increased, thereby enhancing the likelihood of being packed by miners.

# What logic is involved in `calculate_bumped_gas_price`?

The main logic of the `calculate_bumped_gas_price` method is to compute the higher gas price that should be used when resubmitting a transaction, adopting different strategies based on the transaction type:

### Key Features

1. **Minimum Increase Requirement**: The gas price must be increased by at least 10% over the original, ensuring that the new transaction can replace the old one.
2. **Market Price Comparison**: If the current market price is higher, use that to improve the success rate.
3. **Gas Price Cap**: Ensures that the price does not exceed the configured maximum amount.
4. **State Tracking**: The `is_min_bumped` flag indicates whether the minimum increase requirement has been met.
5. **Additional Fees**: Some networks may require extra fees, which will also be calculated and included.

This design ensures that the resubmitted transaction has a sufficiently high gas price to replace the previous transaction while incorporating a cap mechanism to prevent excessively high fees.