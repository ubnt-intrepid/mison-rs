#[cfg(feature = "avx-accel")]
mod imp {
    use std::env;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use misosoup::index_builder::backend::AvxBackend;
    use misosoup::index_builder::IndexBuilder;
    use misosoup::query::QueryTree;
    use misosoup::query_parser::{QueryParser, QueryParserMode};

    pub fn main() {
        let mut tree = QueryTree::default();
        tree.add_path("$._id.$oid").unwrap();
        tree.add_path("$.partners").unwrap();
        tree.add_path("$.twitter_username").unwrap();
        tree.add_path("$.total_money_raised").unwrap();

        let index_builder = IndexBuilder::new(AvxBackend::default(), tree.max_level());
        let parser = QueryParser::new(index_builder, tree);

        let path = env::args().nth(1).unwrap();
        let f = BufReader::new(File::open(path).unwrap());
        for input in f.lines().filter_map(Result::ok) {
            let _ = parser.parse(&input, QueryParserMode::Basic).unwrap();
        }
        println!("{:#?}", parser);
    }
}

#[cfg(not(feature = "avx-accel"))]
mod imp {
    pub fn main() {}
}

fn main() {
    imp::main()
}
