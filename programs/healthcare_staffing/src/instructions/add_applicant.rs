//! AddApplicant instruction handler

use {
    crate::{error::HealthcareStaffingError, state::applicant::NurseApplicant},
    anchor_lang::prelude::*,
    //anchor_spl::token::{Token, TokenAccount},
    //solana_program::program_error::ProgramError,
};

#[derive(Accounts)]
#[instruction(params: AddApplicantParams)]
pub struct AddApplicant<'info> {
    // init means to create applicant account
    // bump to use unique address for applicant account
    #[account(
        init,
        payer = owner,
        space = 8 + NurseApplicant::INIT_SPACE,
        seeds = [b"applicant", owner.key().as_ref()],
        bump
    )]
    pub applicant: Account<'info, NurseApplicant>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct AddApplicantParams {
    national_id_no: u32, // national id no
    full_names: String,  // full names i.e first name, middlename, surname
    dob: String,         // date of birth i.e YYYY-MM-DD
    license_no: u32,     // license no
    hospital: String,    // hospital where applicant works
    country: String,     // home country of applicant
    transcript: String,  // transcript of applicant i.e url of document
    certificate: String, // certificate of applicant i.e url of document
    license: String,     // license of applicant i.e url of document
}

// full names length
const FULL_NAMES_LENGTH: usize = 50;
// date of birth length
const DATE_OF_BIRTH_LENGTH: usize = 10;
// hospital length
const HOSPITAL_LENGTH: usize = 30;
// country length
const COUNTRY_LENGTH: usize = 3;
const COUNTRY_LENGTH_2: usize = 2;
// transcript length
const TRANSCRIPT_LENGTH: usize = 100;
// certificate length
const CERTIFICATE_LENGTH: usize = 100;
// license length
const LICENSE_LENGTH: usize = 100;

pub fn add_applicant(ctx: Context<AddApplicant>, params: &AddApplicantParams) -> Result<()> {
    // validate inputs
    msg!("Validate inputs");
    if params.national_id_no == 0 {
        return Err(HealthcareStaffingError::InvalidNationalIdNo.into());
    }
    if params.full_names.as_bytes().len() > FULL_NAMES_LENGTH {
        return Err(HealthcareStaffingError::ExceededFullNamesMaxLength.into());
    } else if params.full_names.as_bytes().len() == 0 {
        return Err(HealthcareStaffingError::InvalidLength.into());
    }

    if params.dob.as_bytes().len() != DATE_OF_BIRTH_LENGTH {
        return Err(HealthcareStaffingError::ExceededDateOfBirthMaxLength.into());
    } else if params.dob.as_bytes().len() == 0 {
        return Err(HealthcareStaffingError::InvalidLength.into());
    }

    if params.license_no == 0 {
        return Err(HealthcareStaffingError::InvalidLicenseNo.into());
    }
    if params.hospital.as_bytes().len() > HOSPITAL_LENGTH {
        return Err(HealthcareStaffingError::ExceededHospitalMaxLength.into());
    } else if params.hospital.as_bytes().len() == 0 {
        return Err(HealthcareStaffingError::InvalidLength.into());
    }

    if params.country.as_bytes().len() != COUNTRY_LENGTH
        && params.country.as_bytes().len() != COUNTRY_LENGTH_2
    {
        return Err(HealthcareStaffingError::InvalidCountryLength.into());
    } else if params.country.as_bytes().len() == 0 {
        return Err(HealthcareStaffingError::InvalidLength.into());
    }

    if params.transcript.as_bytes().len() > TRANSCRIPT_LENGTH {
        return Err(HealthcareStaffingError::ExceededTranscriptMaxLength.into());
    } else if params.transcript.as_bytes().len() == 0 {
        return Err(HealthcareStaffingError::InvalidLength.into());
    }

    if params.certificate.as_bytes().len() > CERTIFICATE_LENGTH {
        return Err(HealthcareStaffingError::ExceededCertificateMaxLength.into());
    } else if params.certificate.as_bytes().len() == 0 {
        return Err(HealthcareStaffingError::InvalidLength.into());
    }

    if params.license.as_bytes().len() > LICENSE_LENGTH {
        return Err(HealthcareStaffingError::ExceededLicenseMaxLength.into());
    } else if params.license.as_bytes().len() == 0 {
        return Err(HealthcareStaffingError::InvalidLength.into());
    }

    let applicant = &mut ctx.accounts.applicant;
    // * - means dereferencing
    applicant.owner = *ctx.accounts.owner.key;
    applicant.national_id_no = params.national_id_no;
    applicant.full_names = params.full_names.to_string();
    applicant.dob = params.dob.to_string();
    applicant.license_no = params.license_no;
    applicant.hospital = params.hospital.to_string();
    applicant.country = params.country.to_string();
    applicant.transcript = params.transcript.to_string();
    applicant.certificate = params.certificate.to_string();
    applicant.license = params.license.to_string();
    applicant.active = true;

    Ok(())
}
