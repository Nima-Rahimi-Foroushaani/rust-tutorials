#![feature(rustc_private)]
#![deny(rustc::internal)]

#[macro_use]
extern crate tracing;

extern crate rustc_ast;
extern crate rustc_driver;
extern crate rustc_interface;
// extern crate rustc_error_codes;
// extern crate rustc_errors;
// extern crate rustc_hash;
// extern crate rustc_session;
// extern crate rustc_span;
// extern crate rustc_ast_pretty;
// extern crate rustc_lint;

use rustc_ast::util::comments::{gather_comments, Comment};
use rustc_driver::Compilation;
use rustc_interface::{
    interface::{self, Compiler},
    Queries,
};
// use rustc_ast_pretty::pprust::state::Comments;

struct VfCallbacks {
    crate_comments: std::vec::Vec<Comment>,
}

impl rustc_driver::Callbacks for VfCallbacks {
    fn config(&mut self, _config: &mut interface::Config) {
        // _config.opts.debuginfo = config::DebugInfo::Full;
    }
    fn after_parsing<'tcx>(
        &mut self,
        _compiler: &Compiler,
        _queries: &'tcx Queries<'tcx>,
    ) -> Compilation {
        let sess = _compiler.session();
        let src_map = sess.source_map();
        
        // Build a list of files used to compile the output and
        // write Makefile-compatible dependency rules
        // let mut files: Vec<String> = sess
        //     .source_map()
        //     .files()
        //     .iter()
        //     .filter(|fmap| fmap.is_real_file())
        //     .filter(|fmap| !fmap.is_imported())
        //     .map(|fmap| escape_dep_filename(&fmap.name.prefer_local().to_string()))
        //     .collect();

        let input = _compiler.input();
        // The following part is coppied from rustc_driver::pretty::get_source
        // begin
        let src_name = input.source_name();
        let src = String::clone(
            src_map
                .get_source_file(&src_name)
                .expect("get_source_file")
                .src
                .as_ref()
                .expect("src"),
        );
        // end
        info!("Gathering comments of {:?}", src_name);
        self.crate_comments = gather_comments(src_map, src_name, src);
        Compilation::Stop
    }
}

fn main() -> interface::Result<()> {
    // manage env
    std::env::set_var("RUSTC_LOG", "info");
    /* does not work for debug with a kernel
    which is built with "max_level_info" feature
    */
    rustc_driver::init_rustc_env_logger();

    let args: Vec<_> = std::env::args().collect();
    debug!("Passed args {:?}", args);
    let mut cbs = VfCallbacks {
        crate_comments: std::vec::Vec::new(),
    };
    let r_comp = rustc_driver::RunCompiler::new(&args, &mut cbs);
    let res = r_comp.run();
    info!("Gathered comments:");
    for comment in cbs.crate_comments {
        info!("{:?}", comment.lines);
    }

    return res;
}
