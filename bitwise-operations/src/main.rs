fn main() {
    // this demonstrates how to minimize storage overhead when storing bools - a bool can only be 0 or 1, storing it in a u8 wastes 7 remaining bits
    // abool is  either 00000000 or 00000001 - assuming we have to store 8 different bools, we could pack all 8 into a single u8.
    let mut first_num = 0; // or 0b00000000; 
    // this is 00000000
    // can_jump should_jump can_run should_run can_eat should_eat can_talk should_talk
    //      0        0          0        0        0        0         0          0

    // let's give  them power to talk

    let can_talk = first_num | (1 << 1); // let can_talk = 0b00000010;

    first_num = first_num | can_talk; //

    // let's make them talk
    let should_talk = 0b00000001;

    first_num = first_num | should_talk;
    println!("Hello, world!, {:08b}", &first_num);

    // let's check if they can talk
    let can_they_talk = can_talk & first_num;

    println!("{:08b}", can_they_talk);

    if can_they_talk == 0b00000010 {
        println!("Hello, They can talk");
    } else {
        println!("They can't talk right now");
    }
}
