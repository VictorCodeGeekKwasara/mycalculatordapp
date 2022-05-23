use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod mycalculator {
    use super::*;

        pub fn create(ctx:Context<Create>, init_message: String) -> Result<()> {
            let calculator = &mut ctx.accounts.calculator ;
            calculator.greeting = init_message ;

            Ok(())

    }

    pub fn add(ctx: Context<Addition>, num1: i64, num2: i64) -> Result<()> {
        
            Ok(())
    }

    pub fn multiply(ctx: Context<Multiplication>, num1: i64, num2: i64) -> Result<()> {
        
            Ok(())
    }

    pub fn subtract(ctx: Context<Subtraction>, num1: i64, num2: i64) -> Result<()> {
        
            Ok(())
    }

    pub fn divide(ctx: Context<Division>, num1: i64, num2: i64) -> Result<()> {
        
            Ok(())
    }
}

#[derive(Accounts)]
pub struct Create {
    #[account(mut)]
   pub calculator: Accounts<T, Calculator>,
}
