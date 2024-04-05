use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug, InitSpace)]
pub struct NurseApplicant {
    pub owner: Pubkey,       // publickey of the applicant
    pub national_id_no: u32, // national id no
    #[max_len(50)]
    pub full_names: String, // full names i.e first name, middlename, surname
    #[max_len(10)]
    pub dob: String, // date of birth i.e YYYY-MM-DD
    pub license_no: u32,     // license no
    #[max_len(30)]
    pub hospital: String, // hospital where applicant works
    #[max_len(30)]
    pub country: String, // home country of applicant
    #[max_len(100)]
    pub transcript: String, // transcript of applicant i.e url of document
    #[max_len(100)]
    pub certificate: String, // certificate of applicant i.e url of document
    #[max_len(100)]
    pub license: String, // license of applicant i.e url of document
    pub active: bool,        // status of applicant
}
