
fn parse_options(argv: Vec<String>) -> Vec<String> {
    let mut options: Vec<String> = Vec::new();
    for arg in argv {
        if arg.starts_with("-") {
            if is_definition_option(&arg[..]) {
                let definition_option_vec = get_definition_parts(&arg[..]);
                let definition = &definition_option_vec[0];
                options.push(definition.to_string());
            } else {
                options.push(arg);
            }
        }
    }

    options
}

fn is_definition_option(needle: &str) -> bool {
    if needle.contains("=") {
        true
    } else {
        false
    }
}

fn get_definition_parts(needle: &str) -> Vec<String> {
    let opt_def: Vec<_> = needle.split('=').collect();
    vec![String::from(opt_def[0]), String::from(opt_def[1])]
}


// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn function_parse_options() {
        let test_vec = vec![
            String::from("tester"),
            String::from("subcommand"),
            String::from("-o"),
            String::from("spacedefinition"),
            String::from("--longoption"),
            String::from("--defoption=equaldefinition"),
            String::from("lastpos")
            ];

        let expected_vec = vec![
            String::from("-o"),
            String::from("--longoption"),
            String::from("--defoption")
        ];
        
        assert!(parse_options(test_vec) == expected_vec);
    }

    #[test]
    fn function_is_definition_option_true() {
        let true_defintion = "--option=definition";
        assert!(is_definition_option(true_defintion) == true);
    }

    #[test]
    fn function_is_definition_option_false() {
        let false_definition = "--option";
        assert!(is_definition_option(false_definition) == false);
    }

    #[test]
    fn function_get_definition_parts() {
        let definition_string = "--option=definition";
        let expected: Vec<String> = vec![String::from("--option"), String::from("definition")];
        assert!(get_definition_parts(definition_string) == expected);
    }
}