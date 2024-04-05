//! AddApplicant instruction handler

use {
    crate::{
        //error::PerpetualsError,
        //math,
        state::application::{Approval, ApprovalStatus, NursingApplication},
        state::institution::Institution,
    },
    anchor_lang::prelude::*,
    //anchor_spl::token::{Token, TokenAccount},
    //solana_program::program_error::ProgramError,
};

#[derive(Accounts)]
#[instruction(params: ApproveApplicantNursingRegulatoryLicensingBodyParams)]
pub struct ApproveApplicantNursingRegulatoryLicensingBody<'info> {
    #[account(mut)]
    pub institution: Account<'info, Institution>,
    // mut makes it changeble (mutable)
    //#[account(mut)]
    /// CHECK: application account for approval by educational_institution
    #[account(
        mut, constraint = application.educational_institution_approval.approval_status == ApprovalStatus::Approved
    )]
    pub application: Account<'info, NursingApplication>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ApproveApplicantNursingRegulatoryLicensingBodyParams {
    //nurse_applicant: Pubkey,                        // publickey of the applicant
    //healthcare_staffing_company_approval: Approval, // healthcare_staffing_company approval
    //educational_institution_approval: Approval, // educational_institution approval
    nursing_regulatory_licensing_body_approval: Approval, // nursing_regulatory_licensing_body approval
                                                          //commission_approval: Approval,                        // commission approval
                                                          //active: bool,                                         // status of application
}

pub fn approve_applicant_nursing_regulatory_licensing_body(
    ctx: Context<ApproveApplicantNursingRegulatoryLicensingBody>,
    params: &ApproveApplicantNursingRegulatoryLicensingBodyParams,
) -> Result<()> {
    // validate inputs
    msg!("Validate inputs");
    //if params.license_no == 0 {
    //return Err(ProgramError::InvalidArgument.into());
    //}

    let application = &mut ctx.accounts.application;

    if application.nurse_applicant == *ctx.accounts.owner.key {
        application.nursing_regulatory_licensing_body_approval =
            params.nursing_regulatory_licensing_body_approval;
    }

    Ok(())
}
