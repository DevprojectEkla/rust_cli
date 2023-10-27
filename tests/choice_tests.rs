//to see println messages add this option: cargo test -- --nocapture
//consider using also cargo test -- --test-threads=1 to have input one after another

// #[test]
// fn check_pattern_test(){
// let mut choix = Choice::new();
// choix.question = "taper un nom commençant par 3 chiffres:".to_string();
// let test = choix.check_pattern_loop(r"^\d{3}");
// let input = choix.user_input;
// assert_eq!(test,input);
// }

// #[test]
// fn yes_or_no_loop_test_no() {
// let mut choix = Choice::new();

// choix.question = "Paris est la capital du Brésil ?".to_string();
// let test = choix.yes_or_no_loop();
// assert_eq!(test, false)
// }
// #[test]
// fn yes_or_no_loop_test_yes() {
// let mut choix = Choice::new();

// choix.question = "Paris est la capital de la France ?".to_string();
// let test = choix.yes_or_no_loop();
// assert_eq!(test, true)
// }
