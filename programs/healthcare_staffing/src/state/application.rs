use anchor_lang::prelude::*;

#[derive(Copy, Clone, PartialEq, AnchorSerialize, AnchorDeserialize, Default, Debug, InitSpace)]
pub enum DeclinedReason {
    NotExistentStudent,
    CourseNotCompleted,
    StudentExpelled,
    #[default]
    None,
}

#[derive(Copy, Clone, PartialEq, AnchorSerialize, AnchorDeserialize, Default, Debug, InitSpace)]
pub enum ApprovalStatus {
    Approved,
    Declined,
    #[default]
    None,
}

#[derive(Clone, PartialEq, AnchorSerialize, AnchorDeserialize, Default, Debug, InitSpace)]
pub struct Approval {
    pub processed: bool,       // Indicates that the approval was completed
    pub approval_status: bool, //ApprovalStatus,
    #[max_len(20)]
    pub reason: String, //DeclinedReason,
}

#[account]
#[derive(Default, Debug, InitSpace)]
pub struct NursingApplication {
    pub nurse_applicant: Pubkey,         // publickey of the applicant
    pub educational_institution: Pubkey, // publickey of the educational_institution
    pub nursing_regulatory_licensing_body: Pubkey, // publickey of the nursing_regulatory_licensing_body
    pub healthcare_staffing_company: Pubkey,       // publickey of the healthcare_staffing_company
    pub commission: Pubkey,                        // publickey of the commission
    pub healthcare_staffing_company_approval: Approval, // healthcare_staffing_company approval
    pub educational_institution_approval: Approval, // educational_institution approval
    pub nursing_regulatory_licensing_body_approval: Approval, // nursing_regulatory_licensing_body approval
    pub commission_approval: Approval,                        // commission approval
    pub active: bool,                                         // status of application
    pub submitted: bool, // Indicates that the application was submitted by applicant
}
