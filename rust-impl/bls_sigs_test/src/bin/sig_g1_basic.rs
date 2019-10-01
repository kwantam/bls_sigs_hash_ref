extern crate bls_sigs_ref_rs;
extern crate bls_sigs_test;
extern crate pairing;

use bls_sigs_test::{proc_testvec_file, test_sig_basic};
use pairing::bls12_381::G1;
use std::env::args;
use std::io::Result;

fn main() -> Result<()> {
    for arg in args().skip(1) {
        test_sig_basic::<G1>(proc_testvec_file(arg.as_ref())?, 48)?;
    }
    Ok(())
}
