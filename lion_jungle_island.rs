//=== FILE: WorkplaceWellnessProgram.rs ===//

// Imports
extern crate rand;
use std::time::SystemTime;

// Define a enum
enum WellnessProgramType {
    PhysicalActivity,
    MentalActivity,
    Nutrition,
    StressManagement,
    SleepManagement
}

// Define a struct
struct WorkplaceWellnessProgram {
    program_type: WellnessProgramType,
    start_time: SystemTime,
    end_time: SystemTime,
    participants: Vec<String>,
    activity_description: String,
}

// Define a function
fn generate_workplace_wellness_program(program_type: WellnessProgramType, start_time: SystemTime, end_time: SystemTime, participants: Vec<String>) -> WorkplaceWellnessProgram {
    let activity_description: String;
    match program_type {
        WellnessProgramType::PhysicalActivity => {
            activity_description = String::from("We will have a 10 minute walk around the block near the office.");
        }
        WellnessProgramType::MentalActivity => {
            activity_description = String::from("We will have a 10 minute guided meditation session in the office lounge.");
        }
        WellnessProgramType::Nutrition => {
            activity_description = String::from("We will have a 10 minute nutrition talk and healthy snack options available in the break room.");
        }
        WellnessProgramType::StressManagement => {
            activity_description = String::from("We will have a 10 minute stress management session in the office conference room.");
        }
        WellnessProgramType::SleepManagement => {
            activity_description = String::from("We will have a 10 minute sleep hygiene session in the office library.");
        }
    }
    return WorkplaceWellnessProgram {
        program_type,
        start_time,
        end_time,
        participants,
        activity_description
    };
}

// Define a function
fn randomize_workplace_wellness_program(program_type: WellnessProgramType, start_time: SystemTime, end_time: SystemTime, participants: Vec<String>) -> WorkplaceWellnessProgram {
    let mut rng = rand::thread_rng();
    let rnd_activity: u32 = rng.gen_range(0, 5);
    let activity_description: String;
    match rnd_activity {
        0 => {
            activity_description = String::from("We will have a 10 minute walk around the block near the office.");
        }
        1 => {
            activity_description = String::from("We will have a 10 minute guided meditation session in the office lounge.");
        }
        2 => {
            activity_description = String::from("We will have a 10 minute nutrition talk and healthy snack options available in the break room.");
        }
        3 => {
            activity_description = String::from("We will have a 10 minute stress management session in the office conference room.");
        }
        _ => {
            activity_description = String::from("We will have a 10 minute sleep hygiene session in the office library.");
        }
    }
    return WorkplaceWellnessProgram {
        program_type,
        start_time,
        end_time,
        participants,
        activity_description
    };
}

// Define a function
fn broadcast_workplace_wellness_program(program: &WorkplaceWellnessProgram) {
    // Broadcast the program to the participants
    for participant in &program.participants {
        println!("{}: Please join us for a {} session from {} to {}. Details: {}", participant, program.program_type, program.start_time, program.end_time, program.activity_description);
    }
}

// Define a function
fn main() {
    let start_time = SystemTime::now();
    let end_time = start_time + (10 * 60); // 10 minutes

    // Create a list of participants
    let participants: Vec<String> = vec![String::from("John"),
                                        String::from("Jane"),
                                        String::from("Bob"),
                                        String::from("Alice")];

    let program = generate_workplace_wellness_program(WellnessProgramType::PhysicalActivity, start_time, end_time, participants);
    broadcast_workplace_wellness_program(&program);

    let rnd_program = randomize_workplace_wellness_program(WellnessProgramType::PhysicalActivity, start_time, end_time, participants);
    broadcast_workplace_wellness_program(&rnd_program);
}