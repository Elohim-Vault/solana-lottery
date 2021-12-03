use anchor_lang::prelude::*;


declare_id!("FACmnANFTacjhDWgrdJsU8wAQE1ZGthF89jC168n6RSi");

#[program]
pub mod my_project {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        ctx.accounts.lotery_account.total_bets = 0;
        Ok(())
    }

    pub fn new_bet(ctx: Context<NewBet>, message: String, lucky_number: u64) -> ProgramResult {
        let lotery_account = &mut ctx.accounts.lotery_account;
        let user = &ctx.accounts.user;
        let bettor = BetStruct {
            message,
            lucky_number,
            address: *user.to_account_info().key
        };
        lotery_account.list_bets.push(bettor);
        lotery_account.total_bets += 1;
        Ok(())
    }

    pub fn prize_draw(ctx: Context<PrizeDraw>) -> ProgramResult {
        let magic_number = 5; // TODO: random drawn number 
        let lotery_account = &mut ctx.accounts.lotery_account;
        let lucky_bet: Vec<BetStruct> = ctx.accounts.lotery_account
        .list_bets
        .drain(..)
        .filter(|bet| bet.lucky_number == magic_number)
        .collect();

        if (lucky_bet.len() == 0) {
            msg!("No winner");
        }
        else
        {
            msg!("THE WINNER WAS CHOSEN");
            // TODO: Transfer SOL to winner account 
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 1000)]
    pub lotery_account: Account<'info, LoteryAccount>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System> // create new accounts, allocate account data, assign accounts to owning programs, transfer lamports from System Program owned accounts and pay transacation fees.
}

#[account]
pub struct LoteryAccount {
    pub total_bets: u64,
    pub list_bets: Vec<BetStruct>
}

#[derive(Accounts)]
pub struct NewBet<'info> {
    #[account(mut)]
    pub lotery_account: Account<'info, LoteryAccount>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct PrizeDraw <'info>{
    #[account(mut)]
    pub lotery_account: Account<'info, LoteryAccount>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct BetStruct {
    pub message: String,
    pub lucky_number: u64,
    pub address: Pubkey,
}

