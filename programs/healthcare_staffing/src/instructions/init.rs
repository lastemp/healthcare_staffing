//! Init instruction handler

use {
    crate::{
        //error::PerpetualsError,
        //math,
        state::application::NursingApplication,
    },
    anchor_lang::prelude::*,
    //anchor_spl::token::{Token, TokenAccount},
    //solana_program::program_error::ProgramError,
};

#[derive(Accounts)]
#[instruction(params: InitParams)]
pub struct Init<'info> {
    // init means to create application account
    // bump to use unique address for institution account
    #[account(
        init,
        payer = owner,
        space = 8 + NursingApplication::INIT_SPACE,
        seeds = [b"application"],
        bump
    )]
    pub application: Account<'info, NursingApplication>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitParams {
    /* institution_type: InstitutionType, // institution type
    institution_name: String,          // institution name
    country: String,                   // home country of institution */
    active: bool, // status of application
}

pub fn init(ctx: Context<Init>, params: &InitParams) -> Result<()> {
    // validate inputs
    msg!("Validate inputs");
    //if params.license_no == 0 {
    //return Err(ProgramError::InvalidArgument.into());
    //}

    let application = &mut ctx.accounts.application;
    // * - means dereferencing
    /* institution.owner = *ctx.accounts.owner.key;
    institution.institution_type = params.institution_type;
    institution.institution_name = params.institution_name.to_string();
    institution.country = params.country.to_string(); */
    application.active = params.active;

    Ok(())
}
