//! AddApplicant instruction handler

use {
    crate::{
        //error::PerpetualsError,
        //math,
        state::institution::{Institution, InstitutionType},
    },
    anchor_lang::prelude::*,
    //anchor_spl::token::{Token, TokenAccount},
    //solana_program::program_error::ProgramError,
};

#[derive(Accounts)]
#[instruction(params: AddInstitutionParams)]
pub struct AddInstitution<'info> {
    // init means to create institution account
    // bump to use unique address for institution account
    #[account(
        init,
        payer = owner,
        space = 8 + Institution::INIT_SPACE,
        seeds = [b"institution", owner.key().as_ref()],
        bump
    )]
    pub institution: Account<'info, Institution>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct AddInstitutionParams {
    institution_type: u8,     //institution type
    institution_name: String, // institution name
    country: String,          // home country of institution
                              //active: bool,                      // status of institution
}

pub fn add_institution(ctx: Context<AddInstitution>, params: &AddInstitutionParams) -> Result<()> {
    // validate inputs
    msg!("Validate inputs");
    //if params.license_no == 0 {
    //return Err(ProgramError::InvalidArgument.into());
    //}

    let institution = &mut ctx.accounts.institution;
    // * - means dereferencing
    institution.owner = *ctx.accounts.owner.key;
    institution.institution_type = params.institution_type;
    institution.institution_name = params.institution_name.to_string();
    institution.country = params.country.to_string();
    institution.active = true;

    Ok(())
}
