#![feature(rustc_private)]
#![deny(rustc::internal)]

#[macro_use]
extern crate tracing;

extern crate rustc_ast_pretty;
extern crate rustc_driver;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_session;
// extern crate rustc_span;

mod diag_sink;
use rustc_ast_pretty::pprust;
use rustc_interface::interface;
use rustc_session::config as se_config;
// use rustc_span::source_map;

fn main() -> interface::Result<()> {
    use diag_sink::DiagnosticSink;
    use std::{ops::Index, path, process, str, sync};

    // manage env
    // std::env::set_var("RUSTC_LOG", "info");
    /* does not work for debug with a kernel
    which is built with "max_level_info" feature
    */
    rustc_driver::init_rustc_env_logger();

    let args: Vec<_> = std::env::args().collect();
    debug!("Passed args {:?}", args);
    // TODO parse arguments
    let src_file = &args.index(1)[..];

    let out = process::Command::new("rustc")
        .arg("--print=sysroot")
        .current_dir(".")
        .output()
        .unwrap();
    let sysroot = str::from_utf8(&out.stdout).unwrap().trim();
    debug!("Compiler sysroot:{}", sysroot);
    /***
     * If we do not provide sysroot for the rustc_driver it would try to get
     * it or use the default one. See filesearch::get_or_default_sysroot().
     * It does not seem a big deal
     */

    // Session options
    let opts = se_config::Options {
        maybe_sysroot: Some(path::PathBuf::from(sysroot)),
        // Configure the compiler to emit diagnostics in compact JSON format.
        error_format: se_config::ErrorOutputType::Json {
            pretty: false,
            json_rendered: rustc_errors::emitter::HumanReadableErrorType::Default(
                rustc_errors::emitter::ColorConfig::Never,
            ),
        },
        ..se_config::Options::default()
    };

    // Crate source code
    // let input = se_config::Input::Str {
    //     /// A string that is shown in place of a filename.
    //     name: source_map::FileName::Custom("main.rs".to_string()),
    //     input: "fn main() { let x: &str = 1; }".to_string(),
    // };
    // Or maybe just from a file
    let input = se_config::Input::File(path::PathBuf::from(src_file));

    // Diagnostic messages Buffer
    let buffer = sync::Arc::new(sync::Mutex::new(Vec::<u8>::new()));

    // Rustc interface config
    let config = rustc_interface::Config {
        opts,
        input,
        // Redirect the diagnostic output of the compiler to a buffer.
        diagnostic_output: rustc_session::DiagnosticOutput::Raw(Box::from(DiagnosticSink::new(
            buffer.clone(),
        ))),
        crate_cfg: rustc_hash::FxHashSet::default(),
        input_path: None,
        output_dir: None,
        output_file: None,
        file_loader: None,
        stderr: None,

        lint_caps: rustc_hash::FxHashMap::default(),
        parse_sess_created: None,
        register_lints: None,
        override_queries: None,
        registry: rustc_errors::registry::Registry::new(&rustc_error_codes::DIAGNOSTICS),
        make_codegen_backend: None,
    };
    rustc_interface::run_compiler(config, |compiler| {
        compiler.enter(|queries| {
            // TODO: add this to -Z unpretty
            let ast_krate = queries.parse().unwrap().take();
            for item in ast_krate.items {
                println!("{}", pprust::item_to_string(&item));
            }

            // Analyze the crate and inspect the types under the cursor.
            queries.global_ctxt().unwrap().take().enter(|tcx| {
                // Every compilation contains a single crate.
                let hir_krate = tcx.hir();
                // Iterate over the top-level items in the crate, looking for the main function.
                for item in hir_krate.items() {
                    // Use pattern-matching to find a specific node inside the main function.
                    if let rustc_hir::ItemKind::Fn(_, _, body_id) = item.kind {
                        let expr = &tcx.hir().body(body_id).value;
                        if let rustc_hir::ExprKind::Block(block, _) = expr.kind {
                            if let rustc_hir::StmtKind::Local(local) = block.stmts[0].kind {
                                if let Some(expr) = local.init {
                                    let hir_id = expr.hir_id; // hir_id identifies the string "Hello, world!"
                                    let def_id = tcx.hir().local_def_id(item.hir_id()); // def_id identifies the main function
                                    let ty = tcx.typeck(def_id).node_type(hir_id);
                                    println!("{:?}: {:?}", expr, ty); // prints expr(HirId { owner: DefIndex(3), local_id: 4 }: "Hello, world!"): &'static str
                                }
                            }
                        }
                    }
                }
            })
        });
    });

    // Read buffered diagnostics.
    // let diagnostics = String::from_utf8(buffer.lock().unwrap().clone()).unwrap();
    // info!("{}", diagnostics);

    Ok(())
}
