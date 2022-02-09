#![feature(rustc_private)]
#![deny(rustc::internal)]

#[macro_use]
extern crate tracing;

// extern crate rustc_ast_pretty;

extern crate rustc_ast;
extern crate rustc_driver;
extern crate rustc_span;
// extern crate rustc_errors;
extern crate rustc_interface;
// extern crate rustc_lint;
// extern crate rustc_session;

// use rustc_ast_pretty::pprust::state::Comments;
use rustc_ast::util::comments::gather_comments;
use rustc_driver::Compilation;
use rustc_interface::{
    interface::{self, Compiler},
    Queries,
};

// use rustc_span::source_map::SourceFile;

struct VfCallbacks;
impl rustc_driver::Callbacks for VfCallbacks {
    fn config(&mut self, _config: &mut interface::Config) {
        // _config.opts.debuginfo = config::DebugInfo::Full;
    }
    //
    fn after_parsing<'tcx>(
        &mut self,
        _compiler: &Compiler,
        _queries: &'tcx Queries<'tcx>,
    ) -> Compilation {
        let sess = _compiler.session();
        let src_map = sess.source_map();
        let files = &**(src_map.files());

        for f in files {
            info!("File {:?}", f);
        }

        let input = _compiler.input();
        // let krate = _queries.parse();
        // The following part is coppied from rustc_driver::pretty::get_source
        // begin
        let src_name = input.source_name();
        let src = String::clone(
            sess.source_map()
                .get_source_file(&src_name)
                .expect("get_source_file")
                .src
                .as_ref()
                .expect("src"),
        );
        // end
        info!("gathering comments of crate {:?}", src_name);
        let comments = gather_comments(src_map, src_name, src);
        for comment in comments {
            info!("{:?}", comment.lines);
        }
        Compilation::Stop
        // let printer = || -> interface::Result<()> {
        //     let krate_name = _queries.crate_name()?.peek();
        //     info!("Crate name:{}", krate_name);
        //     let krate = _queries.parse()?.peek();
        //     pretty::print_after_parsing(
        //         _compiler.session(),
        //         _compiler.input(),
        //         &krate,
        //         PpMode::Source(PpSourceMode::ExpandedHygiene),
        //         _compiler.output_file().as_ref().map(|p| &**p),
        //     );
        //     Ok(())
        // };
        // let _ = printer();
        // Compilation::Stop
    }
}

fn main() -> interface::Result<()> {
    // manage env
    std::env::set_var("RUSTC_LOG", "info");
    /* does not work for debug with a kernel
    which is built with "max_level_info" feature
    */

    let args: Vec<_> = std::env::args().collect();
    debug!("Passed args {:?}", args);

    rustc_driver::init_rustc_env_logger();
    let mut cbs = VfCallbacks {};
    let r_comp = rustc_driver::RunCompiler::new(&args, &mut cbs);
    let res = r_comp.run();
    info!("Compiler compiles!");
    return res;
}
