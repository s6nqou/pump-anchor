# ![pump](https://github.com/user-attachments/assets/8ae7878a-d0de-4013-b96e-747627041fed) Pump.fun Anchor SDK

[![NPM Version](https://img.shields.io/npm/v/pump-anchor-idl)](https://www.npmjs.com/package/pump-anchor-idl)

An unofficial [Pump.fun](https://pump.fun/) Solana program SDK written in Rust and [Anchor](https://www.anchor-lang.com/).

## Features

- Typescript Anchor IDL
  - building pump transactions with code completion and minimum account inputs required
  - fetching and parsing accounts data like `global` accounts and `bonding_curve` accounts
  - listening to on-chain pump events for observing new pump creations, pump swaps, etc.

- Rust API
  - building your own on-chain program on top of pump via CPI methods
  - building pump transactions with structured accounts and arguments

## Usage

### Typescript Client

Install npm package `pump-anchor-idl`:

```bash
npm install --save pump-anchor-idl
```

#### Example

```typescript
import { Program } from "@coral-xyz/anchor";
import idl from "pump-anchor-idl";

const pumpProgram = new Program(idl, anchorProvider);

const FEE_RECIPIENT_ADDRESS = new PublicKey(
  "CebN5WGQ4jvEPvsVU4EoHEpgzq1VV7AbicfhtW4xC9iM"
);

async function pumpBuy(wallet: Keypair, mint: PublicKey, amount: BN, maxSolCost: BN) {
  const tokenAccount = getAssociatedTokenAddressSync(mint, wallet.publicKey, true);
  const createTokenAccountInstruction = createAssociatedTokenAccountInstruction(
    wallet.publicKey,
    tokenAccount,
    wallet.publicKey,
    mint,
  );

  return await pumpProgram.methods
    .buy(amount, maxSolCost)
    .accounts({
      mint,
      associatedUser: tokenAccount,
      feeRecipient: FEE_RECIPIENT_ADDRESS,
      program: pumpProgram.programId,
    })
    .preInstructions([createTokenAccountInstruction])
    .signers([wallet])
    .rpc();
}
```

### Rust Client or On-chain Program

Add dependencies in `Cargo.toml`:

```toml
[dependencies]
pump = { git = "https://github.com/s6nqou/pump-anchor", package = "pump", features = ["cpi"] }

# for on-chain program
anchor-lang = { version = "0.30.1" }

# for client
anchor-client = { version = "0.30.1" }
```

#### Example: Cross-program invocations (CPI)

```rust
pub fn custom_pump_buy(ctx: Context<CustomPumpBuy>, amount: u64, max_sol_cost: u64) -> Result<()> {
    pump::cpi::buy(
        CpiContext::new(
            ctx.accounts.pump_program.to_account_info(),
            pump::cpi::accounts::Buy {
                global: ctx.accounts.global.to_account_info(),
                fee_recipient: ctx.accounts.fee_recipient.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                bonding_curve: ctx.accounts.bonding_curve.to_account_info(),
                associated_bonding_curve: ctx.accounts.associated_bonding_curve.to_account_info(),
                associated_user: ctx.accounts.associated_user.to_account_info(),
                user: ctx.accounts.user.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
                event_authority: ctx.accounts.event_authority.to_account_info(),
                program: ctx.accounts.pump_program.to_account_info(),
            },
        ),
        amount,
        max_sol_cost,
    )?;
}
```

#### Example: Rust Anchor Client

See [anchor_client crate doc](https://docs.rs/anchor-client/latest/anchor_client/).

## Versions

- `anchor-lang = 0.30.1`
