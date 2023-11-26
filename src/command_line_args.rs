use std::path::PathBuf;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn parse_arguments_nothing_passwed_working_dir_none() {
        // Arrange
        let args = vec!["/path/to/program".to_owned()];

        // Act
        let config = parse_arguments(args);

        // Assert
        assert_eq!(config.working_dir, None);
    }

    #[test]
    fn parse_arguments_argument_passed_working_dir_set() {
        // Arrange
        let args = vec!["/path/to/program".to_owned(), "/path/to/folder".to_owned()];

        // Act
        let config = parse_arguments(args);

        // Assert
        assert!(config.working_dir != None);
        let configured_workering_dir = config.working_dir.unwrap();
        let configured_workering_dir = configured_workering_dir.to_str().unwrap();
        assert_eq!(configured_workering_dir, "/path/to/folder");
    }
}

/// Program flags and options
pub struct ProgramArguments {
    working_dir: Option<PathBuf>,
}

/// Parses command line arguments and returns a ProgramArguments object.
pub fn parse_arguments(args: Vec<String>) -> ProgramArguments {
    let mut set_options = ProgramArguments { working_dir: None };

    // Skip the first argument as that's just the program's path
    for each_argument in &args[1..] {
        // Right now this program only supports a path being passed in.
        let path = PathBuf::from(each_argument);
        set_options.working_dir = Some(path);
    }

    set_options
}
