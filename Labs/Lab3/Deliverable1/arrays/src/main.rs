fn main() {
    let mut groups = [[""; 4]; 6];
    groups[0] = ["Bob", "Carol", "Eric", "Matt"];
    groups[1] = ["Jim", "Lucy", "Terry", "Brenda"];
    groups[2] = ["Susan", "Brad", "Jim", "Matt"];
    groups[3] = ["Sue", "Wendy", "Sam", "Brad"];
    groups[4] = ["Kate", "Jack", "James", "Sydney"];
    groups[5] = ["Mary", "John", "Ricky", "Wendy"];

    let report = searchMember(groups, "Jim");
    if report.exists {
        println!("Member found in group {}", report.group_index.unwrap() + 1);
        if report.group_leader {
            println!("This member is the group leader.");
        } else {
            println!("This member is not the group leader.");
        }
    } else {
        println!("Member not found in any group.");
    }
}

struct MemberReport {
    exists: bool,
    group_index: Option<usize>,
    group_leader: bool, // first member in the group
}

fn searchMember(groups: [[&str; 4]; 6], name: &str) -> MemberReport {
    for (i, group) in groups.iter().enumerate() {
        for (j, member) in group.iter().enumerate() {
            if *member == name {
                return MemberReport {
                    exists: true,
                    group_index: Some(i),
                    group_leader: j == 0,
                };
            }
        }
    }

    MemberReport {
        exists: false,
        group_index: None,
        group_leader: false,
    }
}
