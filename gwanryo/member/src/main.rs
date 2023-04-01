use rand::random;

struct Member {
    name: String,
    email: String,
    address: String,
    active: bool,
}

impl Member {
    fn is_active(&self) -> String {
        if self.active {
            String::from("Active")
        } else {
            String::from("Not active")
        }
    }
}

fn add_new_member(members: &mut Vec<Member>) {
    // Input new member
    let mut new_member = Member {
        name: String::from(""),
        email: String::from(""),
        address: String::from(""),
        active: false,
    };

    println!("Enter name: ");
    std::io::stdin()
        .read_line(&mut new_member.name)
        .expect("Failed to read line");
    println!("Enter email: ");
    std::io::stdin()
        .read_line(&mut new_member.email)
        .expect("Failed to read line");
    println!("Enter address: ");
    std::io::stdin()
        .read_line(&mut new_member.address)
        .expect("Failed to read line");

    // Remove line ending in new_member
    new_member.name = new_member.name.trim().to_string();
    new_member.email = new_member.email.trim().to_string();
    new_member.address = new_member.address.trim().to_string();

    // Set active to random boolean
    new_member.active = random();

    members.push(new_member);
}

fn print_all_members(members: &mut Vec<Member>) {
    println!("Total members: {}", members.len());

    for member in members {
        println!("------------------");
        println!("Name: {}, Email: {}, Address: {}, Active: {}", member.name, member.email, member.address, member.is_active());
    }
}

fn main() {
    let mut members: Vec<Member> = Vec::new();

    // If user input is 1, then add new member
    // if user input is 2, then print all members
    loop {
        println!("------------------");
        println!("1: Add new member");
        println!("2: Print all members");
        println!("3: Exit");
        println!("Select an option:");

        let mut user_input = String::new();
        std::io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let user_input = user_input.trim();
        if user_input == "1" {
            add_new_member(&mut members);
        } else if user_input == "2" {
            print_all_members(&mut members);
        } else if user_input == "3" {
            break;
        } else{
            println!("Invalid input");
        }
    }
}
