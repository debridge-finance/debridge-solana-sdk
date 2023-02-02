use std::{
    env, fs, io,
    path::Path,
    process,
    process::Stdio,
    str::FromStr,
    time::{Duration, Instant},
};

use base58::FromBase58;
use borsh::de::BorshDeserialize;
use debridge_sdk::debridge_accounts::ChainSupportInfo;
use rstest::*;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::{signature::Signer, signer::keypair::Keypair};

pub const USER: &str =
    "52KnSH4Eu8AuL7SNdVohakJCij1WCaoNgbURKTHH2b7FCiBiRGCCCeyeXCX7beQSY9w6wdXoLomGmizFjV1FMwkP";
pub const LOCAL_CLUSTER: &str = "http://127.0.0.1";
pub const DEFAUTL_PORT: &str = "8899";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to get test validator folder")]
    FailedToGetTestValidatorFolder,
    #[error("Failed to get target folder")]
    FailedToGetTargetFolder,
    #[error("Failed to get binaries folder")]
    FailedToGetBinariesFolder,
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Logger(#[from] log::SetLoggerError),
    #[error("TestValidatorNotRun: {stdout}")]
    TestValidatorNotRun { stdout: String },
}

pub struct TestValidator {
    process: process::Child,
}

impl TestValidator {
    pub async fn new(project_dir: &Path, mint_to: Pubkey) -> Result<Self, Error> {
        println!("Test1");
        simple_logger::SimpleLogger::new()
            .with_utc_timestamps()
            .with_level(log::LevelFilter::Debug)
            .init()
            .unwrap();
        println!("Test");
        log::debug!("Creating new test validator");
        let target = project_dir.join("target");

        log::debug!("Use target dir: {:?}", &target);
        let ledger_dir = target.join("test-validator");
        log::debug!("Use ledger dir: {:?}", &ledger_dir);

        if !ledger_dir.exists() {
            log::debug!("Create ledger dir for test validator: {:?}", &ledger_dir);
            fs::create_dir(ledger_dir.clone())?;
        }

        let mut command = process::Command::new("solana-test-validator");
        let command = command
            .stdout(Stdio::null())
            .arg("--ledger")
            .arg(ledger_dir)
            .arg("--mint")
            .arg(mint_to.to_string())
            .arg("--bpf-program")
            .arg(debridge_solana_sdk_example::ID.to_string())
            .arg(target.join("deploy/debridge_solana_sdk_example.so"))
            .arg("--account-dir")
            .arg("./debridge-snapshot/");

        log::debug!("Start {:?}", command);
        let mut process = command.spawn()?;

        let url = format!("{}:{}", LOCAL_CLUSTER, DEFAUTL_PORT);
        log::debug!("Connect to {}", url);
        let client = RpcClient::new(url);

        const TIMEOUT: Duration = Duration::from_secs(60 * 5);

        let instant = Instant::now();
        while client.get_latest_blockhash().await.is_err() {
            if TIMEOUT < instant.elapsed() {
                panic!("Can't start test validator");
            }
            if process.stderr.is_some() {
                return Err(Error::TestValidatorNotRun {
                    stdout: format!("{:?}", process.stderr.take()),
                });
            }
        }

        log::info!("Validator online");

        Ok(Self { process })
    }
}

#[rstest]
#[tokio::test]
async fn send_via_debridge_test() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("failed to get manifest directory");
    let work_dir = Path::new(&manifest_dir);
    let user = Keypair::from_bytes(USER.from_base58().expect("Unreachable").as_slice())
        .expect("Unreachable");

    println!("Work dir: {:?}", work_dir);

    let _validator = TestValidator::new(work_dir, user.pubkey()).await;

    loop {}

    assert!(false);
}
