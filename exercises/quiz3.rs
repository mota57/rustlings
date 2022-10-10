// quiz3.rs
// This quiz tests:
// - Generics
// - Traits
// An imaginary magical school has a new report card generation system written in Rust!
// Currently the system only supports creating report cards where the student's grade
// is represented numerically (e.g. 1.0 -> 5.5).
// However, the school also issues alphabetical grades (A+ -> F-) and needs
// to be able to print both types of report card!

// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to "A+"
// to show that your changes allow alphabetical grades.

// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

// grade range (e.g. 1.0 -> 5.5).
// grade range (e.g. A+ -> F-). 
// need to print both


pub struct ReportCard {
    pub grade: f32,
    pub student_name: String,
    pub student_age: u8,
}

impl ReportCard {
    pub fn print(&self) -> String {

        let mut grades = vec!["A+","A-","B+","B-","C+","C-","F+","F-"];
        let mut index = (self.grade * 2.0_f32 -2.0_f32).floor() as usize;
        let letter = grades.get(index).unwrap();


        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, letter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let mut report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of B+"
        );
        report_card.grade = 1.6;
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of A-"
        );

        report_card.grade = 3.4;
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of C+"
        );

        report_card.grade = 3.7;
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of C-"
        );

        report_card.grade = 4.49;
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of F+"
        );

        report_card.grade = 4.5;
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of F-"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of B+"
        );
    }
}
