use core::panic;
use std::{path::Path, collections::HashMap};
use skullian::{cli::CLIConfig, graph::sg::ExtendableWithTSGrammar};
use stack_graphs::graph::StackGraph;
use tree_sitter_stack_graphs::{StackGraphLanguage, Variables};

fn tree_sitter_process(config: &CLIConfig, path_str : &std::path::Path) {
    let tree: Option<tree_sitter::Tree>;
    if config.language_name.is_empty() {
        tree = skullian::graph::ts::from_file_name(path_str);
    } else {
        tree = skullian::graph::ts::from_file_name_and_language_name(
            path_str,
            &config.language_name);
    }
    if tree.is_none() {
        panic!("error while parsing file {}", path_str.display())
    } else {
        // log::info!("# --- {} --- #", path_str.display());
        // log::info!("{}", skullian::graph::ts::tree_to_sexp(&tree.unwrap()));
        // log::info!("# --- {} --- #", path_str.display());
    }
    log::info!("TreeSitterProcessor is_done_with {}", path_str.display());
}

fn job_tree_sitter(config: &CLIConfig) {
    for target in &config.targets {
        let target_path = Path::new(target);
        if !target_path.exists() {
            panic!("target {} doesn't exists", target_path.display());
        }
        for direntry in walkdir::WalkDir::new(target_path) {
            let entry = direntry.unwrap();
            let extension = entry.path().extension().unwrap_or_default().to_str().unwrap().to_string();
            if entry.file_type().is_file() {
                if
                    config.file_extension.is_empty() ||
                    (extension.len() != 0 &&
                    config.file_extension == extension)
                {
                    tree_sitter_process(config, entry.path());
                }
            }
        }
    }
}

fn stack_graph_process(
    sgl_cache: &mut HashMap<String, StackGraphLanguage>,
    stack_graph: &mut stack_graphs::graph::StackGraph,
    globals: &mut tree_sitter_stack_graphs::Variables,
    language_name: &str,
    path_str : &std::path::Path
) {
    let mut language_name = language_name;
    if language_name.is_empty() {
        language_name = skullian::language::name::from_file_name(path_str).unwrap();
    }
    if sgl_cache.get(&language_name.to_string()).is_none() {
        let tsg_path = skullian::language::tsg::from_language_name(&language_name).unwrap();
        let grammar = skullian::language::grammar::from_language_name(&language_name).expect("unable to load language_grammar");
        let ts_rules = std::fs::read_to_string(tsg_path).expect("stack graph rules not issued");
        sgl_cache.insert(language_name.to_string(), StackGraphLanguage::from_str(grammar, ts_rules.as_str()).unwrap());
        log::info!("StackGraph is_done_with loading sgl for {}", &language_name);
    }

    let sgl = sgl_cache.get(&language_name.to_string()).unwrap();
    stack_graph.extend(globals, path_str, sgl);
    log::info!("StackGraphProcessor is_done_with {}", path_str.display());
}

fn job_stack_graph(config: &CLIConfig) {
    let mut stack_graph = StackGraph::new();
    let mut globals = Variables::new();
    let mut sgl_cache = HashMap::<String, StackGraphLanguage>::new();
    for target in &config.targets {
        let target_path = Path::new(target);
        if !target_path.exists() {
            panic!("target {} doesn't exists", target_path.display());
        }
        for direntry in walkdir::WalkDir::new(target_path) {
            let entry = direntry.unwrap();
            let extension = entry.path().extension().unwrap_or_default().to_str().unwrap().to_string();
            if entry.file_type().is_file() {
                if
                    config.file_extension.is_empty() ||
                    (extension.len() != 0 &&
                    config.file_extension == extension)
                {
                stack_graph_process(
                    &mut sgl_cache,
                    &mut stack_graph,
                    &mut globals,
                    &config.language_name,
                    entry.path()
                );
                }
            }
        }
    }
}

fn job_workflow(config: &CLIConfig) {
    let mut stack_graph = StackGraph::new();
    let mut globals = Variables::new();
    let mut sgl_cache = HashMap::<String, StackGraphLanguage>::new();
    for target in &config.targets {
        let target_path = Path::new(target);
        if !target_path.exists() {
            panic!("target {} doesn't exists", target_path.display());
        }
        for direntry in walkdir::WalkDir::new(target_path) {
            let entry = direntry.unwrap();
            let extension = entry.path().extension().unwrap_or_default().to_str().unwrap().to_string();
            if entry.file_type().is_file() {
                if
                    config.file_extension.is_empty() ||
                    (extension.len() != 0 &&
                    config.file_extension == extension)
                {
                    stack_graph_process(
                        &mut sgl_cache,
                        &mut stack_graph,
                        &mut globals,
                        &config.language_name,
                        entry.path()
                    );
                }
            }
        }
    }
    skullian::graph::dg::build_dep_graph(Path::new(&config.output_file), &stack_graph);
}

fn job_debug(config: &CLIConfig) {
    log::info!("{:?}", config);
}

fn command_line() {
    let mut config = skullian::cli::CLIConfig::new_empty();
    skullian::cli::parse_args(&mut config);
    config.derive_action();
    
    match config.action {
        skullian::cli::CLIAction::TreeSitter() => job_tree_sitter(&config),
        skullian::cli::CLIAction::StackGraph() => job_stack_graph(&config),
        skullian::cli::CLIAction::Debug() => job_debug(&config),
        skullian::cli::CLIAction::Workflow() => job_workflow(&config),
        _ => ()
    }
}

fn main() {
    log4rs::init_file("assets/log4rs.yml", Default::default()).unwrap();
    command_line();
}
