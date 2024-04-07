//! AddApplicant instruction handler

use {
    crate::{
        state::applicant::NurseApplicant,
        //error::PerpetualsError,
        //math,
        state::application::{Approval, NursingApplication},
    },
    anchor_lang::prelude::*,
    //anchor_spl::token::{Token, TokenAccount},
    //solana_program::program_error::ProgramError,
};

#[derive(Accounts)]
//#[instruction(params: SubmitApplicationParams)]
pub struct SubmitApplication<'info> {
    // mut makes it changeble (mutable)
    //#[account(mut)]
    /// CHECK: applicant account for ownership
    #[account(
        mut, has_one = owner
    )]
    pub applicant: Account<'info, NurseApplicant>,

    /// CHECK: application account for ownership
    #[account(
        mut, constraint = application.active
    )]
    pub application: Account<'info, NursingApplication>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/* #[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SubmitApplicationParams {
    nurse_applicant: Pubkey,                        // publickey of the applicant
    healthcare_staffing_company_approval: Approval, // healthcare_staffing_company approval
    educational_institution_approval: Approval,     // educational_institution approval
    nursing_regulatory_licensing_body_approval: Approval, // nursing_regulatory_licensing_body approval
    commission_approval: Approval,                        // commission approval
                                                          //active: bool,                                         // status of application
} */

pub fn submit_application(
    ctx: Context<SubmitApplication>,
    //params: &SubmitApplicationParams,
) -> Result<()> {
    let application = &mut ctx.accounts.application;
    // * - means dereferencing
    application.nurse_applicant = *ctx.accounts.owner.key;
    application.submitted = true; // Indicates that the application was submitted by applicant

    Ok(())
}
