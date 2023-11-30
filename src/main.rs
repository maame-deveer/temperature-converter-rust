use std::io;

fn main() {
    println!("Temperature Converter\n1.To Celsius\n2.To Farenheit\n\nEnter 1-2: ");

    loop{
        println!("please input your choice: ");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if choice == 1 || choice == 2
        {
            if choice == 1
            {
                println!("Converting To Celsius:");
                println!("Enter The Temperature In Farenheit: ");

                convert(choice);
                break;
            }
            else{
                println!("Converting To Farenheit:");
                println!("Enter The Temperature In Celsius: ");

                convert(choice);
                break;
            }
        }
        else{
            println!("please enter a valid number");
        }
    }
}

fn convert(choice: u32){
    let mut temp= String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let temp: u32 = temp.trim().parse().expect("Please type a number!");
    
    if choice == 1
    {
        let temp = to_celsius(temp);
        println!("Result: {temp}°C");
    }
    else if choice == 2
    {
        let temp = to_farenheit(temp);
        println!("Result: {temp}°F");
    }
    else {
        println!("Error, the calculation does not exist");
    }
}

fn to_celsius(temp: u32) -> u32{
    let temp = (temp - 32) * 5/9;

    return temp;
}

fn to_farenheit(temp: u32) -> u32{
    let temp = temp * (9/5) + 32;

    return temp;
}
