//! This binary is used to test `cargo-audit`'s ability to find and report
//! affected functions in binaries. This binary is vulnerable a selection of
//! advisories through February 2026.
//!
//! The advisories this binary is vulnerable to are not exhaustive. That is,
//! there are advisories within the given timeframe that this binary is not
//! vulnerable to.
//!
//! Also, the advisories are not disjoint. For example, the binary would be
//! vulnerable to RUSTSEC-2019-0036 even if it did not explicitly call
//! `failure::Fail::__private_get_type_id__`.

fn main() {
    let _ = anstream::adapter::strip_str("");
    let _ = branca::Branca::new(&[]).unwrap().decode("", 0);
    let _ = crayon::utils::object_pool::ObjectPool::<_, ()>::default()
        .free(crayon::application::prelude::LifecycleListenerHandle::default());
    let _ = failure::Fail::__private_get_type_id__(&std::io::Error::other(""));
    let _ = gix_date::parse::TimeBuf::default().as_str();
    let _ = lzf::compress(&[]);
    let _ = mail_internals::utils::vec_insert_bytes(&mut vec![], 0, &[]);
    let _ = mp3_metadata::read_from_slice(&[]);
    let _ = ntru::types::PrivateKey::default().export(&ntru::encparams::EncParams::default());
    let _ = parse_duration::parse("");
    let _ = rtvm_interpreter::Interpreter::default().program_counter();
    let _ = ruint::algorithms::div::reciprocal_mg10(0);
    let _ = rustc_serialize::json::Json::from_str("");
    let _ = sequoia_openpgp::crypto::ecdh::aes_key_unwrap(
        sequoia_openpgp::types::SymmetricAlgorithm::default(),
        &sequoia_openpgp::crypto::mem::Protected::new(0),
        &[],
    );
    let _ = shaman::cryptoutil::read_u32v_be(&mut [], &[]);
    let _ = time::OffsetDateTime::now_local();
    let _ = wasmtime_jit_debug::perf_jitdump::JitDumpFile::new("", 0)
        .unwrap()
        .dump_code_load_record("", std::ptr::null(), 0, 0, 0, 0);
    let _ = whoami::username();
    let _ = xmp_toolkit::XmpFile::new().unwrap().close();
    let _ = unsafe {
        zlib_rs::inflate::inflate(
            &mut zlib_rs::inflate::InflateStream::from_stream_mut(std::ptr::null_mut()).unwrap(),
            zlib_rs::InflateFlush::default(),
        )
    };
}
