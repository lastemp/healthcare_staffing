//! AddApplicant instruction handler

use {
    crate::{
        error::HealthcareStaffingError,
        state::application::{Approval, ApprovalStatus, NursingApplication},
        state::institution::Institution,
    },
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction(params: ApproveApplicantCommissionParams)]
pub struct ApproveApplicantCommission<'info> {
    #[account(mut, has_one = owner)]
    pub institution: Account<'info, Institution>,
    // mut makes it changeble (mutable)
    //#[account(mut)]
    //mut, constraint = application.nursing_regulatory_licensing_body_approval.approval_status == ApprovalStatus::Approved
    /// CHECK: application account for approval by nursing_regulatory_licensing_body
    #[account(
        mut, constraint = application.nursing_regulatory_licensing_body_approval.approval_status @ HealthcareStaffingError::InvalidApprovalStatus
    )]
    pub application: Account<'info, NursingApplication>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ApproveApplicantCommissionParams {
    //nurse_applicant: Pubkey,                        // publickey of the applicant
    //healthcare_staffing_company_approval: Approval, // healthcare_staffing_company approval
    //educational_institution_approval: Approval, // educational_institution approval
    //nursing_regulatory_licensing_body_approval: Approval, // nursing_regulatory_licensing_body approval
    commission_approval: Approval, // commission approval
                                   //active: bool,                                         // status of application
}

// Declined reason length
const DECLINED_REASON_LENGTH: usize = 20;

pub fn approve_applicant_commission(
    ctx: Context<ApproveApplicantCommission>,
    params: &ApproveApplicantCommissionParams,
) -> Result<()> {
    // validate inputs
    msg!("Validate inputs");
    if !params.commission_approval.approval_status
        && params.commission_approval.reason.as_bytes().len() == 0
    {
        return Err(HealthcareStaffingError::InvalidDeclinedReason.into());
    }
    if params.commission_approval.reason.as_bytes().len() > DECLINED_REASON_LENGTH {
        return Err(HealthcareStaffingError::ExceededDeclinedReasonMaxLength.into());
    }

    let application = &mut ctx.accounts.application;
    let institution_type = ctx.accounts.institution.institution_type;

    /* EducationalInstitution = 1,
    NursingRegulatoryLicensingBody = 2,
    Commission = 3,
    HealthcareStaffingCompany = 4, */
    if institution_type != 3 {
        panic!()
    }

    // * - means dereferencing
    application.commission = *ctx.accounts.owner.key;

    if application.nurse_applicant != *ctx.accounts.owner.key {
        application.commission_approval = params.commission_approval.to_owned();
    }

    Ok(())
}
