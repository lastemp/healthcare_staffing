use anchor_lang::prelude::*;

#[derive(Copy, Clone, PartialEq, AnchorSerialize, AnchorDeserialize, Debug, Default, InitSpace)]
pub enum InstitutionType {
    EducationalInstitution,
    NursingRegulatoryLicensingBody,
    Commission,
    HealthcareStaffingCompany,
    #[default]
    None,
}

#[account]
#[derive(Default, Debug, InitSpace)]
pub struct Institution {
    pub owner: Pubkey, // publickey of the institution
    //pub institution_type: InstitutionType, // institution type
    pub institution_type: u8, //institution type
    #[max_len(30)]
    pub institution_name: String, // institution name
    #[max_len(3)]
    pub country: String, // home country of institution
    pub active: bool,         // status of institution
}
