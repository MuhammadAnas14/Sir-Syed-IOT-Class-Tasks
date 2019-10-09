# Sir-Syed-Rust-Class-Task

# CHAPTER # 3 FUNCTION

Task 1:
Write a function that can take 3 ​ integer​ numbers as
argument/parameter and ​ prints​ the sum of them 

Task 2:
Write a function that can take 3 ​ float​ numbers as
argument/parameter and ​ returns​ the multiplication result of them. In the main function

# CHAPTER # 3 CONTROL FLOW

TASK#1
Store marks in a variable and print the Grade of student.
Marks Range:
Greater than 80 - Grade A+
Between 70 and 80 - Grade A
Between 60 and 70 - Grade B
Between 50 and 60 - Grade C
Between 40 and 50 - Grade D
Below 40 - Grade F
Hint [ condition 1 &&  condition 2 ]

TASK#2
Print even numbers using for loop. (till 20)
Output: 2,4,6,8,10,12,....

TASK#3
Print odd numbers using while loop. (till 19)
Output: 1,3,5,7,9,11

TASK#4
Store an integer (any value) in a variable and ​ print the table of that number using any loop. (till  * 12)

Output:
5 * 1 = 5
5 * 2 = 10
5 * 3 = 15
5 * 4 = 20
5 * 5 = 25
5 * 6 = 30
5 * 7 = 35
5 * 8 = 40
5 * 9 = 45
5 * 10 = 50

# CHAPTER # 4 OWNERSHIP

Task#1

Assign a variable s to PAKISTAN then make a function that takes s variable as parameter/argument but doesn’t take ownership of s variable and concatenates “ZINDABAD” in variable s to makes it “PAKISTAN ZINDABAD”. Print variable s after change

Task#2

Take a string input from user and store it in a variable then pass that variable in a function which returns you the length of the string. Print the length.
Hint:
Use std::io;
let mut x = String::new();
io::stdin.read_line(&mut x).expect(“invalid input”);

# CHAPTER # 5 STRUCTS

TASK#1

Make a custom data type named Employee using struct with following fields:
	
EMPLOYEE NO
NAME
EMAIL
Gender
PHONE NO
active(boolean)

Assign all five fields the appropriate data type in struct definition.

Create 2 instances named Employee1 and Employee2 of Employee struct and print Name and phone no of  Employee1, print Employee_no and gender of Employee2 .Take  Gender and active of employee1 into employee2




TASK#2

Make a custom data type named Student using struct with following fields:
	
Name 
Father_Name
Class
Grade

Assign all four fields the appropriate data type in struct definition.

Create a function which takes all four fields of struct as
a parameters/argument and make instance named student1 of Student struct and print Name and class 
Of Student.Print all the fields of student 1 

Task#3
Make a custom data type named  Triangle using struct with following fields:
	
base
Heigth
Type of the triangle

Create an instance of that struct and pass it to t
Make a function Area_of_circle which take the struct Triangle as a parameter  and calculate the area of triangle
And return it.Create an instance of that struct and pass it to the function and print the type of the triangle and  the area of triangle in the main function.
Hint:
Area= ½ * base *heigth



