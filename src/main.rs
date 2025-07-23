#[forbid(missing_docs)]
use matrix_sdk_search::{
    Event, EventId, IndexError, RoomIndex, UserId,
};

fn main() -> Result<(), IndexError> {
    let mut index = RoomIndex::new_in_ram().expect("big oops");

    // 2x week, (the)m the & (The)re the, 1x cuter

    index.add_event(Event::new(
        EventId::from("$event_id_1"),
        "There is a meeting next week about the whales.",
        UserId::from("@user_id_1"),
        123456701,
    ))?;

    index.add_event(Event::new(
        EventId::from("$event_id_2"),
        "Dolphins are so much cuter.",
        UserId::from("@user_id_1"),
        123456702,
    ))?;

    index.add_event(Event::new(
        EventId::from("$event_id_3"),
        "We can go and see them the week after that.",
        UserId::from("@user_id_2"),
        12345673,
    ))?;

    index.force_commit()?;

    let res = index.search("week", 10)?;
    for i in res.iter() {
        print!("{i:?}, ");
        println!("")
    }

    let res = index.search("the", 10)?;
    for i in res.iter() {
        print!("{i:?}, ");
        println!("")
    }

    let res = index.search("cuter", 10)?;
    for i in res.iter() {
        print!("{i:?}, ");
        println!("")
    }

    Ok(())
}
