use bsuite_core::{BsuiteCoreError, EmitFormat, ExitCode, ProcessExitEmitter, RoutingKey};
use std::io::Write;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct SharedBuf(Arc<Mutex<Vec<u8>>>);

impl SharedBuf {
    fn new() -> Self {
        Self(Arc::new(Mutex::new(Vec::new())))
    }

    fn bytes(&self) -> Vec<u8> {
        self.0.lock().expect("lock").clone()
    }

    fn is_empty(&self) -> bool {
        self.0.lock().expect("lock").is_empty()
    }

    fn as_string(&self) -> String {
        String::from_utf8(self.bytes()).expect("UTF-8")
    }
}

impl Write for SharedBuf {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.lock().expect("lock").extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn buf_emitter(format: EmitFormat) -> (ProcessExitEmitter, SharedBuf, SharedBuf) {
    let out = SharedBuf::new();
    let err = SharedBuf::new();
    let emitter =
        ProcessExitEmitter::for_streams(format, Box::new(out.clone()), Box::new(err.clone()));
    (emitter, out, err)
}

// Both CorpusSchemaMismatch directions prove the emitter's routing is direction-independent;
// two RoutingKey values in CorpusKeyMissing prove key-independence.
fn all_corpus_error_variants() -> Vec<BsuiteCoreError> {
    vec![
        BsuiteCoreError::CorpusSignatureInvalid,
        BsuiteCoreError::CorpusSchemaMismatch {
            expected: 1,
            found: 2,
        },
        BsuiteCoreError::CorpusSchemaMismatch {
            expected: 2,
            found: 1,
        },
        BsuiteCoreError::CorpusDeserializationFailed(
            "ed25519 signature mismatch: embedded corpus rejected".into(),
        ),
        BsuiteCoreError::CorpusKeyMissing(RoutingKey::BRatch),
        BsuiteCoreError::CorpusKeyMissing(RoutingKey::BGround),
    ]
}

#[test]
fn plain_mode_all_corpus_errors_route_to_internal_error() {
    for err in all_corpus_error_variants() {
        let (mut emitter, _, _) = buf_emitter(EmitFormat::Plain);
        let code = emitter.emit_directive(Err(err.clone()));
        assert_eq!(
            code,
            ExitCode::InternalError,
            "corpus error must route to InternalError for {err:?}",
        );
    }
}

#[test]
fn plain_mode_all_corpus_errors_emit_nothing_to_stdout() {
    for err in all_corpus_error_variants() {
        let (mut emitter, out, _) = buf_emitter(EmitFormat::Plain);
        let _ = emitter.emit_directive(Err(err.clone()));
        assert!(
            out.is_empty(),
            "stdout must be empty in plain mode for {err:?}; got: {:?}",
            out.as_string(),
        );
    }
}

#[test]
fn plain_mode_all_corpus_errors_emit_display_string_to_stderr() {
    for err in all_corpus_error_variants() {
        let expected = format!("{err}\n");
        let (mut emitter, _, stderr) = buf_emitter(EmitFormat::Plain);
        let _ = emitter.emit_directive(Err(err.clone()));
        assert_eq!(
            stderr.as_string(),
            expected,
            "stderr must hold the Display output for {err:?}",
        );
    }
}

#[test]
fn json_mode_all_corpus_errors_write_json_envelope_with_error_to_stdout() {
    for err in all_corpus_error_variants() {
        let (mut emitter, out, _) = buf_emitter(EmitFormat::Json);
        let _ = emitter.emit_directive(Err(err.clone()));
        let envelope: serde_json::Value = serde_json::from_str(out.as_string().trim())
            .unwrap_or_else(|_| {
                panic!(
                    "stdout must be valid JSON in JSON mode for {err:?}; got: {:?}",
                    out.as_string()
                )
            });
        assert_eq!(
            envelope["outcome"], "internal_error",
            "JSON envelope outcome must be internal_error for {err:?}",
        );
        assert!(
            envelope["error"].is_object(),
            "JSON envelope must contain an error object for {err:?}",
        );
        assert!(
            envelope.get("directive").is_none(),
            "JSON envelope must not contain a directive field on error for {err:?}",
        );
    }
}

#[test]
fn json_mode_all_corpus_errors_emit_nothing_to_stderr() {
    for err in all_corpus_error_variants() {
        let (mut emitter, _, stderr) = buf_emitter(EmitFormat::Json);
        let _ = emitter.emit_directive(Err(err.clone()));
        assert!(
            stderr.is_empty(),
            "stderr must be empty in JSON mode for {err:?}; got: {:?}",
            stderr.as_string(),
        );
    }
}
