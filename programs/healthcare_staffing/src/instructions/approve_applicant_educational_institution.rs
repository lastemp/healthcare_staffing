//! AddApplicant instruction handler

use {
    crate::{
        //error::PerpetualsError,
        //math,
        state::application::{Approval, NursingApplication},
        state::institution::Institution,
    },
    anchor_lang::prelude::*,
    //anchor_spl::token::{Token, TokenAccount},
    //solana_program::program_error::ProgramError,
};

#[derive(Accounts)]
#[instruction(params: ApproveApplicantEducationalInstitutionParams)]
pub struct ApproveApplicantEducationalInstitution<'info> {
    #[account(mut)]
    pub institution: Account<'info, Institution>,
    // mut makes it changeble (mutable)
    //#[account(mut)]
    /// CHECK: application account for submitted status
    #[account(
        mut, constraint = application.submitted == true
    )]
    pub application: Account<'info, NursingApplication>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ApproveApplicantEducationalInstitutionParams {
    //nurse_applicant: Pubkey,                        // publickey of the applicant
    //healthcare_staffing_company_approval: Approval, // healthcare_staffing_company approval
    educational_institution_approval: Approval, // educational_institution approval
                                                //nursing_regulatory_licensing_body_approval: Approval, // nursing_regulatory_licensing_body approval
                                                //commission_approval: Approval,                        // commission approval
                                                //active: bool,                                         // status of application
}

pub fn approve_applicant_educational_institution(
    ctx: Context<ApproveApplicantEducationalInstitution>,
    params: &ApproveApplicantEducationalInstitutionParams,
) -> Result<()> {
    // validate inputs
    msg!("Validate inputs");
    //if params.license_no == 0 {
    //return Err(ProgramError::InvalidArgument.into());
    //}

    let application = &mut ctx.accounts.application;

    if application.nurse_applicant == *ctx.accounts.owner.key {
        application.educational_institution_approval = params.educational_institution_approval;
    }

    Ok(())
}
