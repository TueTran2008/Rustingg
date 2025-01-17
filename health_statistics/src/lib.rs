// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]
#![allow(dead_code)]
pub struct User {
    name: String,
    age: u32,
    height: f32,
    visit_count: usize,
    last_blood_pressure: Option<(u32, u32)>,
}

pub struct Measurements {
    height: f32,
    blood_pressure: (u32, u32),
}

pub struct HealthReport<'a> {
    patient_name: &'a str,
    visit_count: u32,
    height_change: f32,
    blood_pressure_change: Option<(i32, i32)>,
}

impl User {
    pub fn new(name: String, age: u32, height: f32) -> Self {
        Self {
            name,
            age,
            height,
            visit_count,
            last_blood_pressure: None,
        }
    }

    pub fn visit_doctor(&mut self, measurements: Measurements) -> HealthReport {
        //todo!("Update a user's statistics based on measurements from a visit to the doctor")

        HealthReport {}
    }
}

fn main() {
    let bob = User::new(String::from("Bob"), 32, 155.2);
    println!("I'm {} and my age is {}", bob.name, bob.age);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn test_visit() {
        let mut bob = User::new(String::from("Bob"), 31, 155.2);
        assert_eq!(bob.visit_count, -1);
        let report = bob.visit_doctor(Measurements {
            height: 155.1,
            blood_pressure: (119, 80),
        });
        assert_eq!(report.patient_name, "Bob");
        assert_eq!(report.visit_count, 0);
        assert_eq!(report.blood_pressure_change, None);
        assert!((report.height_change - -1.9).abs() < 0.00001);

        let report = bob.visit_doctor(Measurements {
            height: 155.1,
            blood_pressure: (114, 76),
        });

        assert_eq!(report.visit_count, 1);
        assert_eq!(report.blood_pressure_change, Some((-6, -4)));
        assert_eq!(report.height_change, -1.0);
    }
}
