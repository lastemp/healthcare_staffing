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
#[instruction(params: ApproveApplicantHealthCareStaffingCompanyParams)]
pub struct ApproveApplicantHealthCareStaffingCompany<'info> {
    #[account(mut, has_one = owner)]
    pub institution: Account<'info, Institution>,
    // mut makes it changeble (mutable)
    //#[account(mut)]
    /// CHECK: application account for approval by commission
    #[account(
        mut, constraint = application.commission_approval.approval_status
    )]
    pub application: Account<'info, NursingApplication>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ApproveApplicantHealthCareStaffingCompanyParams {
    //nurse_applicant: Pubkey,                        // publickey of the applicant
    healthcare_staffing_company_approval: Approval, // healthcare_staffing_company approval
                                                    //educational_institution_approval: Approval,     // educational_institution approval
                                                    //nursing_regulatory_licensing_body_approval: Approval, // nursing_regulatory_licensing_body approval
                                                    //commission_approval: Approval,                        // commission approval
                                                    //active: bool,                                         // status of application
}

pub fn approve_applicant_healthcare_staffing_company(
    ctx: Context<ApproveApplicantHealthCareStaffingCompany>,
    params: &ApproveApplicantHealthCareStaffingCompanyParams,
) -> Result<()> {
    // validate inputs
    msg!("Validate inputs");
    //if params.license_no == 0 {
    //return Err(ProgramError::InvalidArgument.into());
    //}

    let application = &mut ctx.accounts.application;
    let institution_type = ctx.accounts.institution.institution_type;

    /* EducationalInstitution = 1,
    NursingRegulatoryLicensingBody = 2,
    Commission = 3,
    HealthcareStaffingCompany = 4, */
    if institution_type != 4 {
        panic!()
    }

    // * - means dereferencing
    application.healthcare_staffing_company = *ctx.accounts.owner.key;

    if application.nurse_applicant != *ctx.accounts.owner.key {
        application.healthcare_staffing_company_approval =
            params.healthcare_staffing_company_approval.to_owned();
    }

    Ok(())
}
