#[cfg(test)]
mod operation_tests {
    use crate::parser::CliApiArgs;


    pub fn initialize() -> CliApiArgs {
        CliApiArgs {
            port: 8080,
            verbose: 0,
            debug: 0,
            trace: 0,
        }
    }

    #[test]
    fn test_parse_client_to_address() {
        let args = initialize();
        let client_args: CliApiArgs = CliApiArgs {
            port: args.port.clone(),
            verbose: args.verbose.clone(),
            debug: args.debug.clone(),
            trace: args.trace.clone(),
        };

        assert_eq!(client_args.port, 8080);
    }
    
}