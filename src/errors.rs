#![allow(missing_docs)]

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    errors {
        InvalidQuery {
            description("invalid query")
            display("invalid query")
        }

        InvalidRecord {
            description("invalid record")
            display("invalid record")
        }
    }
}