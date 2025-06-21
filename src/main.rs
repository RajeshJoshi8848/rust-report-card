use std::fs::File;
use std::io;
use std::path::Path;

use printpdf::*;


struct Student {
    name: String,
    average: f32,
    grade: char,
}

fn calculate_average(total_marks: f32, subjects: u32) -> f32 {
    total_marks / subjects as f32
}

fn assign_grade(avg: f32) -> char {
    match avg {
        x if x >= 90.0 => 'A',
        x if x >= 75.0 => 'B',
        x if x >= 60.0 => 'C',
        _ => 'D',
    }
}

fn main() {
    let mut students: Vec<Student> = Vec::new();

    loop {
        let mut name = String::new();
        println!("Enter Student Name (or type 'exit' to finish):");
        io::stdin().read_line(&mut name).unwrap();
        let name = name.trim();
        if name.eq_ignore_ascii_case("exit") {
            break;
        }

        let mut total_marks = String::new();
        let mut num_subjects = String::new();

        println!("Enter Total Marks:");
        io::stdin().read_line(&mut total_marks).unwrap();

        println!("Enter Number of Subjects:");
        io::stdin().read_line(&mut num_subjects).unwrap();

        let total_marks: f32 = total_marks.trim().parse().unwrap_or(0.0);
        let num_subjects: u32 = num_subjects.trim().parse().unwrap_or(1);

        let avg = calculate_average(total_marks, num_subjects);
        let grade = assign_grade(avg);

        println!("\n📄 Report Card:");
        println!("Student: {}", name);
        println!("Average: {:.2}", avg);
        println!("Grade: {}\n", grade);

        students.push(Student {
            name: name.to_string(),
            average: avg,
            grade,
        });
    }

    if !students.is_empty() {
        generate_pdf(&students, "report_card.pdf");
        println!("✅ PDF report generated as 'report_card.pdf'");
    } else {
        println!("⚠️ No student data entered.");
    }
}

fn generate_pdf(students: &Vec<Student>, file_name: &str) {
    let (doc, page1, layer1) = PdfDocument::new("Student Report Card", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();
    let mut y_position = Mm(280.0);

    // Title
    current_layer.use_text("📄 Student Report Card", 16.0, Mm(20.0), y_position, &font);
    y_position -= Mm(15.0);

    for (i, student) in students.iter().enumerate() {
        let text = format!(
            "{}. Name: {:20} | Avg: {:6.2} | Grade: {}",
            i + 1,
            student.name,
            student.average,
            student.grade
        );

        current_layer.use_text(text, 12.0, Mm(20.0), y_position, &font);
        y_position -= Mm(10.0);
    }

    // ✅ Fix: Wrap the File in BufWriter
    let file = File::create(Path::new(file_name)).unwrap();
    let mut writer = std::io::BufWriter::new(file);
    doc.save(&mut writer).unwrap();
}
