import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { HealthcareStaffing } from "../target/types/healthcare_staffing";

describe("healthcare_staffing", () => {
  // Configure the client to use the local cluster.
  //anchor.setProvider(anchor.AnchorProvider.env());

  let provider = anchor.AnchorProvider.local("http://127.0.0.1:8899");
  const program = anchor.workspace
    .HealthcareStaffing as Program<HealthcareStaffing>;
  const admin_owner = anchor.web3.Keypair.generate();
  const applicant_owner = anchor.web3.Keypair.generate();
  const institution_owner_1 = anchor.web3.Keypair.generate();
  const institution_owner_2 = anchor.web3.Keypair.generate();
  const institution_owner_3 = anchor.web3.Keypair.generate();
  const institution_owner_4 = anchor.web3.Keypair.generate();

  let [application] = anchor.web3.PublicKey.findProgramAddressSync(
    [anchor.utils.bytes.utf8.encode("application")],
    program.programId
  );

  let [applicant] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("applicant"),
      applicant_owner.publicKey.toBuffer(),
    ],
    program.programId
  );

  let [institution_1] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("institution"),
      institution_owner_1.publicKey.toBuffer(),
    ],
    program.programId
  );

  let [institution_2] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("institution"),
      institution_owner_2.publicKey.toBuffer(),
    ],
    program.programId
  );

  let [institution_3] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("institution"),
      institution_owner_3.publicKey.toBuffer(),
    ],
    program.programId
  );

  let [institution_4] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("institution"),
      institution_owner_4.publicKey.toBuffer(),
    ],
    program.programId
  );

  // admin_owner
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      admin_owner.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  // applicant_owner
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      applicant_owner.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  // institution_owner 1
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      institution_owner_1.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  // institution_owner 2
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      institution_owner_2.publicKey,
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
      institution_owner_3.publicKey,
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
      institution_owner_4.publicKey,
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
        owner: admin_owner.publicKey,
        application: application,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([admin_owner])
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
        owner: applicant_owner.publicKey,
        applicant: applicant,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([applicant_owner])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.nurseApplicant.fetch(applicant);
    console.log("applicant: ", result);
  });

  it("Is add institution - EducationalInstitution", async () => {
    enum InstitutionType {
      EducationalInstitution,
      NursingRegulatoryLicensingBody,
      Commission,
      HealthcareStaffingCompany,
      None,
    }

    let initParams = {
      //institutionType: InstitutionType.EducationalInstitution,
      institutionName: "moi university",
      country: "KE",
    };

    /*     console.log(
      "initParams: " + initParams.institutionType
    ); */

    const tx = await program.methods
      .addInstitution(initParams)
      .accounts({
        owner: institution_owner_1.publicKey,
        institution: institution_1,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([institution_owner_1])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.institution.fetch(institution_1);
    console.log("institution_1: ", result);
  });

  it("Is add institution - NursingRegulatoryLicensingBody", async () => {
    enum InstitutionType {
      EducationalInstitution,
      NursingRegulatoryLicensingBody,
      Commission,
      HealthcareStaffingCompany,
      None,
    }

    let initParams = {
      //institutionType: InstitutionType.EducationalInstitution,
      institutionName: "nursing council",
      country: "KE",
    };

    /*     console.log(
      "initParams: " + initParams.institutionType
    ); */

    const tx = await program.methods
      .addInstitution(initParams)
      .accounts({
        owner: institution_owner_2.publicKey,
        institution: institution_2,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([institution_owner_2])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.institution.fetch(institution_2);
    console.log("institution_2: ", result);
  });

  it("Is add institution - Commission", async () => {
    enum InstitutionType {
      EducationalInstitution,
      NursingRegulatoryLicensingBody,
      Commission,
      HealthcareStaffingCompany,
      None,
    }

    let initParams = {
      //institutionType: InstitutionType.EducationalInstitution,
      institutionName: "cgfns",
      country: "USA",
    };

    /*     console.log(
      "initParams: " + initParams.institutionType
    ); */

    const tx = await program.methods
      .addInstitution(initParams)
      .accounts({
        owner: institution_owner_3.publicKey,
        institution: institution_3,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([institution_owner_3])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.institution.fetch(institution_3);
    console.log("institution_3: ", result);
  });

  it("Is add institution - HealthcareStaffingCompany", async () => {
    enum InstitutionType {
      EducationalInstitution,
      NursingRegulatoryLicensingBody,
      Commission,
      HealthcareStaffingCompany,
      None,
    }

    let initParams = {
      //institutionType: InstitutionType.EducationalInstitution,
      institutionName: "medpro",
      country: "USA",
    };

    /*     console.log(
      "initParams: " + initParams.institutionType
    ); */

    const tx = await program.methods
      .addInstitution(initParams)
      .accounts({
        owner: institution_owner_4.publicKey,
        institution: institution_4,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([institution_owner_4])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.institution.fetch(institution_4);
    console.log("institution_4: ", result);
  });
});
