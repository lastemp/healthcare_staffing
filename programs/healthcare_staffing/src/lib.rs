//! healthcare_staffing program entrypoint

pub mod error;
pub mod instructions;
pub mod state;

use {anchor_lang::prelude::*, instructions::*};

declare_id!("HoYSBxAs2puTobgpTFTGf6ieVDBf2oXYCN4tUn5KNEHs");

#[program]
pub mod healthcare_staffing {
    use super::*;

    // admin instructions
    pub fn init(ctx: Context<Init>, params: InitParams) -> Result<()> {
        instructions::init(ctx, &params)
    }

    // public instructions
    pub fn add_applicant(ctx: Context<AddApplicant>, params: AddApplicantParams) -> Result<()> {
        instructions::add_applicant(ctx, &params)
    }

    pub fn add_institution(
        ctx: Context<AddInstitution>,
        params: AddInstitutionParams,
    ) -> Result<()> {
        instructions::add_institution(ctx, &params)
    }

    pub fn submit_application(
        ctx: Context<SubmitApplication>,
        //params: SubmitApplicationParams,
    ) -> Result<()> {
        instructions::submit_application(ctx)
    }

    pub fn approve_applicant_healthcare_staffing_company(
        ctx: Context<ApproveApplicantHealthCareStaffingCompany>,
        params: ApproveApplicantHealthCareStaffingCompanyParams,
    ) -> Result<()> {
        instructions::approve_applicant_healthcare_staffing_company(ctx, &params)
    }

    pub fn approve_applicant_educational_institution(
        ctx: Context<ApproveApplicantEducationalInstitution>,
        params: ApproveApplicantEducationalInstitutionParams,
    ) -> Result<()> {
        instructions::approve_applicant_educational_institution(ctx, &params)
    }

    pub fn approve_applicant_nursing_regulatory_licensing_body(
        ctx: Context<ApproveApplicantNursingRegulatoryLicensingBody>,
        params: ApproveApplicantNursingRegulatoryLicensingBodyParams,
    ) -> Result<()> {
        instructions::approve_applicant_nursing_regulatory_licensing_body(ctx, &params)
    }

    pub fn approve_applicant_commission(
        ctx: Context<ApproveApplicantCommission>,
        params: ApproveApplicantCommissionParams,
    ) -> Result<()> {
        instructions::approve_applicant_commission(ctx, &params)
    }
}
