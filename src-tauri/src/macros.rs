#[macro_export]
macro_rules! generate_upscale_run_information {
    ($path:expr, $save_path:expr, $upscale_factor:expr, $upscale_type:expr, $advanced_options:expr) => {{
        let upscale_information = format!(
            "Upscaling image: {} with the following configuration:
            -> Save path: {}
            -> Upscale factor: {} ### NOT WORKING ATM ###
            -> Upscale type: {}
            -> Operating System: {}
            -> Advanced options: {:?}\n",
            $path,
            $save_path,
            $upscale_factor,
            $upscale_type,
            std::env::consts::OS,
            $advanced_options
        );
        println!("{}", &upscale_information);

        upscale_information
    }};
}

#[macro_export]
macro_rules! generate_command_parameters {
    () => {{
        let models_folder = match std::env::consts::OS {
            "linux" => "./resources/models",
            "windows" => r#".\resources\models"#,
            _ => {
                panic!("Unsupported operating system, currently only Windows and Linux are supported.")
            }
        };

        models_folder
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_generate_upscale_run_information() {
        let upscale_information = generate_upscale_run_information!(
            "test_path",
            "test_save_path",
            "test_upscale_factor",
            "test_upscale_type",
            "test_advanced_options"
        );

        assert_eq!(
            upscale_information,
            format!(
                "Upscaling image: test_path with the following configuration:
            -> Save path: test_save_path
            -> Upscale factor: test_upscale_factor ### NOT WORKING ATM ###
            -> Upscale type: test_upscale_type
            -> Operating System: {}
            -> Advanced options: \"test_advanced_options\"\n",
                std::env::consts::OS
            )
        );
    }
}
