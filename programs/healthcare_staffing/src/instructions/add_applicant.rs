//! AddApplicant instruction handler

use {
    crate::{
        //error::PerpetualsError,
        //math,
        state::applicant::NurseApplicant,
    },
    anchor_lang::prelude::*,
    //anchor_spl::token::{Token, TokenAccount},
    //solana_program::program_error::ProgramError,
};

#[derive(Accounts)]
#[instruction(params: AddApplicantParams)]
pub struct AddApplicant<'info> {
    // init means to create applicant account
    // bump to use unique address for applicant account
    #[account(
        init,
        payer = owner,
        space = 8 + NurseApplicant::INIT_SPACE,
        seeds = [b"applicant", owner.key().as_ref()],
        bump
    )]
    pub applicant: Account<'info, NurseApplicant>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct AddApplicantParams {
    national_id_no: u32, // national id no
    full_names: String,  // full names i.e first name, middlename, surname
    dob: String,         // date of birth i.e YYYY-MM-DD
    license_no: u32,     // license no
    hospital: String,    // hospital where applicant works
    country: String,     // home country of applicant
    transcript: String,  // transcript of applicant i.e url of document
    certificate: String, // certificate of applicant i.e url of document
    license: String,     // license of applicant i.e url of document
}

pub fn add_applicant(ctx: Context<AddApplicant>, params: &AddApplicantParams) -> Result<()> {
    // validate inputs
    msg!("Validate inputs");
    if params.license_no == 0 {
        //return Err(ProgramError::InvalidArgument.into());
    }

    let applicant = &mut ctx.accounts.applicant;
    // * - means dereferencing
    applicant.owner = *ctx.accounts.owner.key;
    applicant.national_id_no = params.national_id_no;
    applicant.full_names = params.full_names.to_string();
    applicant.dob = params.dob.to_string();
    applicant.license_no = params.license_no;
    applicant.hospital = params.hospital.to_string();
    applicant.country = params.country.to_string();
    applicant.transcript = params.transcript.to_string();
    applicant.certificate = params.certificate.to_string();
    applicant.license = params.license.to_string();
    applicant.active = true;

    Ok(())
}
