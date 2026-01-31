enum Status {
    SUCCESS,
    FAILURE,
    PENDING,
}

struct Task {
    name: String,
    description: String,
    status: Status,
}

fn main(){
    let mut task = Task{
        name: String::from("Implement Feature X"),
        description: String::from("Develop and test feature X for the project."),
        status: Status::SUCCESS,
    };

    // Update status
    task.status= Status::FAILURE;

    // Check status with match
    match task.status{
        Status::SUCCESS=> println!("Task '{}' completed successfully.", task.name),
        Status::FAILURE=> println!("Task '{}' failed to complete.", task.name),
        Status::PENDING=> println!("Task '{}' is still pending.", task.name),
    
    }
}