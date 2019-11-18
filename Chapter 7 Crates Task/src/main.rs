mod teacher;
mod Syllabus;
mod Timetable;

fn main() {
    
    teacher::teachers_list::math_teacher();
    teacher::teachers_list::physics_teacher();
    teacher::teachers_list::Science_teacher();

    Syllabus::Syllabus_length::math_Syllabus();
    Syllabus::Syllabus_length::physics_Syllabus();
    Syllabus::Syllabus_length::Science_Syllabus();

    Timetable::classes_time::math_classes();
    Timetable::classes_time::physics_classes();
    Timetable::classes_time::Science_classes();
}
