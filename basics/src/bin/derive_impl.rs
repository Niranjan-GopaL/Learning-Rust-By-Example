//! This file is to demonstrate THE USAGE of #[derive()]




/// This enum has Debug , Clone , Copy impls  
/// Automatically ADDS 3 impls cuz of derive macro
#[derive(Debug, Clone,Copy)]
enum Position {
    Manager,
    Supervisor,
    Worker
}


/// This struct (and ALL ITS FIELDS) has Debug , Clone , Copy impls  
/// Automatically ADDS 3 impls cuz of derive macro
/// Be carefull ALL THE FIELDS (if a field is another struct or enum) MUST HAVE THOSE impls as well
#[derive(Debug, Clone,Copy)]
struct Employee{
    position: Position,
    work_hours: usize
}



fn main(){
    let employee_1: Employee = Employee{
        position: Position::Worker,
        work_hours: 40,
    };
    // THIS IS INSANE, PRINTING OUT A STRUCT
    println!("{:?}", employee_1);
    println!("Employee_1 has work_hours: {:?} and position: {:?}",
    employee_1.work_hours,
    //You can PRINT WHICH VARIENT OF ENUM IT IS!!! (NORMALLY WE MATCHED AND PRINTED THE VARIENTS)
    employee_1.position);



/*                  OLD WAYYYYYYYYYYYY LOLLL :- 

    match employee_1.position {
        Position::Manager => println!("Manager"),
        Position::Worker => println!("Worker"),
        Position::Supervisor => println!("Supervisor"),
    }
    

                    New way:- 

    #[derive(Debug)]
    enum Position{
        ...
    }


    YAY!!
    println!("{:?}", Position);
*/
2



    // Now you can "clone" structs too (cuz of the Clone functionality)
    let employee_1_temp: Employee = employee_1.clone();

    
    println!("{:?}", employee_1_temp);
    println!("Employee_1_temp has work_hours: {:?} and position: {:?}", 
    employee_1_temp.work_hours,
    employee_1_temp.position);

}