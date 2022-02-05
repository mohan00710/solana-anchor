use anchor_lang::prelude::*;
use crate::{Initialize,Burn,TransferToken};

impl<'info> Initialize<'info> {
    pub fn process_mint(&mut self, amount: u64) -> ProgramResult {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = anchor_spl::token::MintTo {
            mint: self.mint.to_account_info(),
            to: self.mint_to.to_account_info(),
            authority: self.mint.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        anchor_spl::token::mint_to(cpi_ctx, amount)?;
        Ok(())
    }
}

impl<'info> Burn <'info>{
    pub fn process_burn(&mut self, amount: u64) -> ProgramResult{
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = anchor_spl::token::Burn{
            mint : self.mint.to_account_info(),
            to : self.source.to_account_info(),
            authority : self.payer.to_account_info()
        };
        let cpi_ctx = CpiContext::new(cpi_program,cpi_accounts);
        anchor_spl::token::burn(cpi_ctx, amount)?;
        Ok(())
    }
}

impl<'info> TransferToken <'info>{
    pub fn process_transfer(&mut self, amount:u64) -> ProgramResult{
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = anchor_spl::token::Transfer{
            from : self.source.to_account_info(),
            to : self.destination.to_account_info(),
            authority : self.payer.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program,cpi_accounts);
        anchor_spl::token::transfer(cpi_ctx, amount)?;
        Ok(())
    }
}