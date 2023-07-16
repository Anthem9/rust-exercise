use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json;

// 学生结构体
#[derive(Clone, Serialize, Deserialize)]
pub struct Student {
    id: u32,
    name: String,
    class_id: u32,
}

// 社团结构体
#[derive(Clone, Serialize, Deserialize)]
pub struct Club {
    id: u32,
    name: String,
    member_ids: Vec<u32>,
}

// 班级结构体
#[derive(Clone, Serialize, Deserialize)]
pub struct Class {
    id: u32,
    name: String,
    student_ids: Vec<u32>,
}

// 课程结构体
#[derive(Clone, Serialize, Deserialize)]
pub struct Course {
    id: u32,
    name: String,
    class_id: u32,
}

// 学生管理系统
#[derive(Serialize, Deserialize)]
pub struct StudentManagementSystem {
    students: HashMap<u32, Student>,
    clubs: HashMap<u32, Club>,
    classes: HashMap<u32, Class>,
    courses: HashMap<u32, Course>,
}

impl StudentManagementSystem {
    // 创建一个新的学生管理系统
    pub fn new() -> Self {
        Self {
            students: HashMap::new(),
            clubs: HashMap::new(),
            classes: HashMap::new(),
            courses: HashMap::new(),
        }
    }

    // 学生的 CRUD 操作
    pub fn add_student(&mut self, student: Student) {
        self.students.insert(student.id, student);
    }
    pub fn delete_student(&mut self, student_id: u32) {
        self.students.remove(&student_id);
    }
    pub fn update_student(&mut self, student: Student) {
        self.students.insert(student.id, student);
    }
    pub fn get_student(&self, student_id: u32) -> Option<&Student> {
        self.students.get(&student_id)
    }

    // 社团的 CRUD 操作
pub fn add_club(&mut self, club: Club) {
    self.clubs.insert(club.id, club);
}
pub fn delete_club(&mut self, club_id: u32) {
    self.clubs.remove(&club_id);
}
pub fn update_club(&mut self, club: Club) {
    self.clubs.insert(club.id, club);
}
pub fn get_club(&self, club_id: u32) -> Option<&Club> {
    self.clubs.get(&club_id)
}

// 班级的 CRUD 操作
pub fn add_class(&mut self, class: Class) {
    self.classes.insert(class.id, class);
}
pub fn delete_class(&mut self, class_id: u32) {
    self.classes.remove(&class_id);
}
pub fn update_class(&mut self, class: Class) {
    self.classes.insert(class.id, class);
}
pub fn get_class(&self, class_id: u32) -> Option<&Class> {
    self.classes.get(&class_id)
}

// 课程的 CRUD 操作
pub fn add_course(&mut self, course: Course) {
    self.courses.insert(course.id, course);
}
pub fn delete_course(&mut self, course_id: u32) {
    self.courses.remove(&course_id);
}
pub fn update_course(&mut self, course: Course) {
    self.courses.insert(course.id, course);
}
pub fn get_course(&self, course_id: u32) -> Option<&Course> {
    self.courses.get(&course_id)
}

    // 保存数据到文件
    pub fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let serialized = serde_json::to_string(self)?;
        let mut file = File::create(filename)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    // 从文件加载数据
    pub fn load_from_file(filename: &str) -> std::io::Result<Self> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let system: StudentManagementSystem = serde_json::from_str(&contents)?;
        Ok(system)
    }
}

fn main() {
    // 创建一个新的学生管理系统
    let mut sms = StudentManagementSystem::new();

    // 添加一些学生
    sms.add_student(Student {
        id: 1,
        name: "Alice".to_string(),
        class_id: 101,
    });
    sms.add_student(Student {
        id: 2,
        name: "Bob".to_string(),
        class_id: 101,
    });

    // 将数据保存到文件
    let result = sms.save_to_file("data.json");
    if let Err(e) = result {
        println!("Error saving data: {}", e);
    }

    // 加载数据
    let loaded_sms = StudentManagementSystem::load_from_file("data.json");
    match loaded_sms {
        Ok(sms) => {
            // 打印学生列表
            for student in sms.students.values() {
                println!("Student ID: {}, Name: {}", student.id, student.name);
            }
        }
        Err(e) => {
            println!("Error loading data: {}", e);
        }
    }
}