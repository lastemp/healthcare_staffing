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
#[instruction(params: ApproveApplicantNursingRegulatoryLicensingBodyParams)]
pub struct ApproveApplicantNursingRegulatoryLicensingBody<'info> {
    #[account(mut, has_one = owner)]
    pub institution: Account<'info, Institution>,
    // mut makes it changeble (mutable)
    //#[account(mut)]
    /// CHECK: application account for approval by educational_institution
    #[account(
        mut, constraint = application.educational_institution_approval.approval_status @ HealthcareStaffingError::InvalidApprovalStatus
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

// Declined reason length
const DECLINED_REASON_LENGTH: usize = 20;

pub fn approve_applicant_nursing_regulatory_licensing_body(
    ctx: Context<ApproveApplicantNursingRegulatoryLicensingBody>,
    params: &ApproveApplicantNursingRegulatoryLicensingBodyParams,
) -> Result<()> {
    // validate inputs
    msg!("Validate inputs");
    if !params
        .nursing_regulatory_licensing_body_approval
        .approval_status
        && params
            .nursing_regulatory_licensing_body_approval
            .reason
            .as_bytes()
            .len()
            == 0
    {
        return Err(HealthcareStaffingError::InvalidDeclinedReason.into());
    }
    if params
        .nursing_regulatory_licensing_body_approval
        .reason
        .as_bytes()
        .len()
        > DECLINED_REASON_LENGTH
    {
        return Err(HealthcareStaffingError::ExceededDeclinedReasonMaxLength.into());
    }

    let application = &mut ctx.accounts.application;
    let institution_type = ctx.accounts.institution.institution_type;

    /* EducationalInstitution = 1,
    NursingRegulatoryLicensingBody = 2,
    Commission = 3,
    HealthcareStaffingCompany = 4, */
    if institution_type != 2 {
        return Err(HealthcareStaffingError::MismatchedInstitutionType.into());
    }

    // * - means dereferencing
    application.nursing_regulatory_licensing_body = *ctx.accounts.owner.key;

    if application.nurse_applicant != *ctx.accounts.owner.key {
        application.nursing_regulatory_licensing_body_approval =
            params.nursing_regulatory_licensing_body_approval.to_owned();
    }

    Ok(())
}
