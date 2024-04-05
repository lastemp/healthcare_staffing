// admin instructions

// test instructions

// public instructions
pub mod add_applicant;
pub mod add_institution;
pub mod approve_applicant_commission;
pub mod approve_applicant_educational_institution;
pub mod approve_applicant_healthcare_staffing_company;
pub mod approve_applicant_nursing_regulatory_licensing_body;
pub mod init;
pub mod submit_application;

// bring everything in scope
pub use {
    add_applicant::*, add_institution::*, approve_applicant_commission::*,
    approve_applicant_educational_institution::*, approve_applicant_healthcare_staffing_company::*,
    approve_applicant_nursing_regulatory_licensing_body::*, init::*, submit_application::*,
};
