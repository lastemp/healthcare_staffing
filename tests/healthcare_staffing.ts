import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { HealthcareStaffing } from "../target/types/healthcare_staffing";

describe("healthcare_staffing", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.local("http://127.0.0.1:8899");
  const program = anchor.workspace
    .HealthcareStaffing as Program<HealthcareStaffing>;
  const adminOwner = anchor.web3.Keypair.generate();
  const applicantOwner = anchor.web3.Keypair.generate();
  const institutionOwner1 = anchor.web3.Keypair.generate();
  const institutionOwner2 = anchor.web3.Keypair.generate();
  const institutionOwner3 = anchor.web3.Keypair.generate();
  const institutionOwner4 = anchor.web3.Keypair.generate();

  let [application] = anchor.web3.PublicKey.findProgramAddressSync(
    [anchor.utils.bytes.utf8.encode("application")],
    program.programId
  );

  let [applicant] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("applicant"),
      applicantOwner.publicKey.toBuffer(),
    ],
    program.programId
  );

  let [institution1] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("institution"),
      institutionOwner1.publicKey.toBuffer(),
    ],
    program.programId
  );

  let [institution2] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("institution"),
      institutionOwner2.publicKey.toBuffer(),
    ],
    program.programId
  );

  let [institution3] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("institution"),
      institutionOwner3.publicKey.toBuffer(),
    ],
    program.programId
  );

  let [institution4] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("institution"),
      institutionOwner4.publicKey.toBuffer(),
    ],
    program.programId
  );

  // admin owner
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      adminOwner.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  // applicant owner
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      applicantOwner.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  // institution owner 1
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      institutionOwner1.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  // institution owner 2
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      institutionOwner2.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  // institution_owner 3
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      institutionOwner3.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  // institution_owner 4
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      institutionOwner4.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  it("Is initialized!", async () => {
    let initParams = { active: true };

    const tx = await program.methods
      .init(initParams)
      .accounts({
        owner: adminOwner.publicKey,
        application: application,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([adminOwner])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.nursingApplication.fetch(application);
    console.log("application: ", result);
  });

  it("Is add applicant!", async () => {
    let initParams = {
      nationalIdNo: 1234,
      fullNames: "paul john",
      dob: "2010-07-13",
      licenseNo: 6163,
      hospital: "Maryhill Wellcare",
      country: "KE",
      transcript: "https://me/transcript/aysb",
      certificate: "https://me/certificate/lahs",
      license: "https://me/license/nabd",
    };

    const tx = await program.methods
      .addApplicant(initParams)
      .accounts({
        owner: applicantOwner.publicKey,
        applicant: applicant,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([applicantOwner])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.nurseApplicant.fetch(applicant);
    console.log("applicant: ", result);
  });

  it("Is add institution - EducationalInstitution", async () => {
    /* EducationalInstitution = 1,
    NursingRegulatoryLicensingBody = 2,
    Commission = 3,
    HealthcareStaffingCompany = 4, */

    let initParams = {
      //EducationalInstitution
      institutionType: 1,
      institutionName: "moi university",
      country: "KE",
    };

    const tx = await program.methods
      .addInstitution(initParams)
      .accounts({
        owner: institutionOwner1.publicKey,
        institution: institution1,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([institutionOwner1])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.institution.fetch(institution1);
    console.log("institution: ", result);
  });

  it("Is add institution - NursingRegulatoryLicensingBody", async () => {
    /* EducationalInstitution = 1,
    NursingRegulatoryLicensingBody = 2,
    Commission = 3,
    HealthcareStaffingCompany = 4, */

    let initParams = {
      //NursingRegulatoryLicensingBody
      institutionType: 2,
      institutionName: "nursing council",
      country: "KE",
    };

    const tx = await program.methods
      .addInstitution(initParams)
      .accounts({
        owner: institutionOwner2.publicKey,
        institution: institution2,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([institutionOwner2])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.institution.fetch(institution2);
    console.log("institution: ", result);
  });

  it("Is add institution - Commission", async () => {
    /* EducationalInstitution = 1,
    NursingRegulatoryLicensingBody = 2,
    Commission = 3,
    HealthcareStaffingCompany = 4, */

    let initParams = {
      //Commission
      institutionType: 3,
      institutionName: "cgfns",
      country: "USA",
    };

    const tx = await program.methods
      .addInstitution(initParams)
      .accounts({
        owner: institutionOwner3.publicKey,
        institution: institution3,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([institutionOwner3])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.institution.fetch(institution3);
    console.log("institution: ", result);
  });

  it("Is add institution - HealthcareStaffingCompany", async () => {
    /* EducationalInstitution = 1,
    NursingRegulatoryLicensingBody = 2,
    Commission = 3,
    HealthcareStaffingCompany = 4, */

    let initParams = {
      //HealthcareStaffingCompany
      institutionType: 4,
      institutionName: "medpro",
      country: "USA",
    };

    const tx = await program.methods
      .addInstitution(initParams)
      .accounts({
        owner: institutionOwner4.publicKey,
        institution: institution4,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([institutionOwner4])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.institution.fetch(institution4);
    console.log("institution: ", result);
  });

  it("Is submit application!", async () => {
    const tx = await program.methods
      .submitApplication()
      .accounts({
        owner: applicantOwner.publicKey,
        applicant: applicant,
        application: application,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([applicantOwner])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.nursingApplication.fetch(application);
    console.log("application: ", result);
  });

  it("Is approve applicant - EducationalInstitution", async () => {
    let approval = {
      processed: true, // Indicates that the approval process was completed
      approvalStatus: true,
      reason: "",
    };

    let initParams = {
      educationalInstitutionApproval: approval,
    };

    const tx = await program.methods
      .approveApplicantEducationalInstitution(initParams)
      .accounts({
        owner: institutionOwner1.publicKey,
        institution: institution1,
        application: application,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([institutionOwner1])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.nursingApplication.fetch(application);
    console.log("application: ", result);
  });

  it("Is approve applicant - NursingRegulatoryLicensingBody", async () => {
    let approval = {
      processed: true, // Indicates that the approval process was completed
      approvalStatus: true,
      reason: "",
    };

    let initParams = {
      nursingRegulatoryLicensingBodyApproval: approval,
    };

    const tx = await program.methods
      .approveApplicantNursingRegulatoryLicensingBody(initParams)
      .accounts({
        owner: institutionOwner2.publicKey,
        institution: institution2,
        application: application,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([institutionOwner2])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.nursingApplication.fetch(application);
    console.log("application: ", result);
  });

  it("Is approve applicant - Commission", async () => {
    let approval = {
      processed: true, // Indicates that the approval process was completed
      approvalStatus: true,
      reason: "",
    };

    let initParams = {
      commissionApproval: approval,
    };

    const tx = await program.methods
      .approveApplicantCommission(initParams)
      .accounts({
        owner: institutionOwner3.publicKey,
        institution: institution3,
        application: application,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([institutionOwner3])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.nursingApplication.fetch(application);
    console.log("application: ", result);
  });

  it("Is approve applicant - HealthcareStaffingCompany", async () => {
    let approval = {
      processed: true, // Indicates that the approval process was completed
      approvalStatus: true,
      reason: "",
    };

    let initParams = {
      healthcareStaffingCompanyApproval: approval,
    };

    const tx = await program.methods
      .approveApplicantHealthcareStaffingCompany(initParams)
      .accounts({
        owner: institutionOwner4.publicKey,
        institution: institution4,
        application: application,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([institutionOwner4])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.nursingApplication.fetch(application);
    console.log("application: ", result);
  });
});
