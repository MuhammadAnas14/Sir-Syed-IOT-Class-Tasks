# Sir-Syed-IOT-Class-Task

# CHAPTER # 3 FUNCTION

Task 1:

Write a function that can take 3 ​ integer​ numbers as
argument/parameter and ​ prints​ the sum of them 

Task 2:

Write a function that can take 3 ​ float​ numbers as
argument/parameter and ​ returns​ the multiplication result of them. In the main function

# CHAPTER # 3 CONTROL FLOW

TASK#1:

Store marks in a variable and print the Grade of student.
Marks Range:
Greater than 80 - Grade A+
Between 70 and 80 - Grade A
Between 60 and 70 - Grade B
Between 50 and 60 - Grade C
Between 40 and 50 - Grade D
Below 40 - Grade F
Hint [ condition 1 &&  condition 2 ]

TASK#2:

Print even numbers using for loop. (till 20)
Output: 2,4,6,8,10,12,....

TASK#3:

Print odd numbers using while loop. (till 19)
Output: 1,3,5,7,9,11

TASK#4:

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

Task#1:

Assign a variable s to PAKISTAN then make a function that takes s variable as parameter/argument but doesn’t take ownership of s variable and concatenates “ZINDABAD” in variable s to makes it “PAKISTAN ZINDABAD”. Print variable s after change

Task#2:

Take a string input from user and store it in a variable then pass that variable in a function which returns you the length of the string. Print the length.

Hint:

Use std::io;

let mut x = String::new();

io::stdin.read_line(&mut x).expect(“invalid input”);

# CHAPTER # 5 STRUCTS

TASK#1:

Make a custom data type named Employee using struct with following fields:
	
EMPLOYEE NO

NAME

EMAIL

Gender

PHONE NO

active(boolean)

Assign all five fields the appropriate data type in struct definition.

Create 2 instances named Employee1 and Employee2 of Employee struct and print Name and phone no of  Employee1, print Employee_no and gender of Employee2 .Take  Gender and active of employee1 into employee2


TASK#2:

Make a custom data type named Student using struct with following fields:
	
Name 

Father_Name

Class

Grade

Assign all four fields the appropriate data type in struct definition.

Create a function which takes all four fields of struct as
a parameters/argument and make instance named student1 of Student struct and print Name and class 
Of Student.Print all the fields of student 1 

Task#3:

Make a custom data type named  Triangle using struct with following fields:
	
Base

Heigth

Type of the triangle

Make a function Area_of_circle which take the struct Triangle as a parameter  and calculate the area of triangle
And return it.Create an instance of that struct and pass it to the function and print the type of the triangle and  the area of triangle in the main function.

Hint:

Area= ½ * base *heigth

# 6-Oct-2019 Class Initial Task

Task#1

Make a calculator:

Make a custom data type named Entries using struct with following subject marks fields:

Number1

Number2 

Operation

You have to make the four function And pass the struct to it:

Add

Sub

Mult

Div

In these four function you have to just return the answer of that operation
In the main function You have to create an instance of that struct and Match the operation with control flow structure(If_else Structure) and call the function according to it and save it to variable and print the result

# CHAPTER # 5 METHODS

Task#1:

Make a custom data type named Marks using struct with following subject marks fields:
	
English 

Science

Math

Pakistan_Studies,

Urdu

Assign all five fields the appropriate data type in struct definition.

Create two implementation block:

 Total Marks

 Percentage

Create an instance of the Student_marks and print the total marks and percentage of the Student.


# 13-Oct-2019 Class Initial Task

Make a custom data type named Report Card of a University student using struct with following fields:

GPA

Grade

Total_Marks

Assign all Three fields the appropriate data type in struct definition.

Make 4 instances of each year of his university graduation.

Make two Arrays:

One  will store all the  GPA and

Other will store total marks  of all four year 

Make 2 Function and pass the array as an argument without taking its ownership and return: 

One Function will calculate highest Gpa using loops

Other Total marks of all 4 year using loops

Make an implementation block which compare two year gpa and returns true/false

You have to print highest Gpa and overall marks and two comparition of two different year.

Note:    Year1>Year2


# CHAPTER # 6 ENUMS

TASK#1:

Define an enum for Vehicle where vehicles are of 3 variants i.e Cars, Trucks, Bikes.

Assign some useful data directly into it 

Create instances for all variants and also pass some data when creating instances.

let a: Option<i32> = Some(5);

Implement the above statement with enum of vehicle

TASK#2

Define an enum for Department where vehicles are of 3 variants i.e Civil, Electronic, Computer.

Assign some useful data using struct:

For example:

No of Student

No of Classes

Create instances for all variants and also pass some data when creating instances.

# 03-Nov-2019 Class Initial Task

Make a custom data type enum Items  with following fields 

Fruits

Vegetables

HousesHold Goods

Make a custom data type struct Customer_details with following fields

Name

Phone no

Address

Define 2  function :-

1)The first function  takes an instance of Items enum as a parameter and  return the price of the item using match operator.

2)The second function takes an instance of the struck and prints all the details of customer

You have to apply a infinity loop in which you have to take 4 inputs 

1)Selection of items 

2)Name 

3)Phone no 

4)Address

Now you have to check it with control flow whether the user has selected fruits/vegetable/household and make an instance 

of enum according to the selection of the user and pass it the function of price and print the price.

Moreover, the last thing is to create an instance of struct and pass it to the function.

# Chapter # 7 Crates

Task#1

Make 3 module with following name :

Teachers

Syllabus

Time table

In Every module  you  have to make a module and then you have to make 3  more function and print something in that.

All have to imported in the main.rs file
 
# Chapter # 7 Library

Task#2

Create a library of the following tree structure using lib.rs

Crate
	
	Calculator_functions
	
		Basic_functions
			Add

			Subtract
			
			Divide
			
			Multiply

		Power_functions

			Square_function

			Cube_function
			
			power_function

Import two or three function  in main.rs

Hint: You can relate following tree diagram with a diagram in Chapter 7.2.

# Chapter # 8 Vectors

TASK # 1:

Make a vector laptops and assigned some laptop companies to it like “dell”,”lenovo”.......

Print the whole vector and calculate its length and print it

Now make a new  vector and insert some prices of laptop into it 

Use the loop and extract the price  and add 5% tax to it and print it one by one

Use the get() method and print the last item of laptops by match method

# Chapter # 8 String

TASK # 2 

Create a new empty string “japanese” and push " language"  to it.

Make "السالم "علیكم to string type and assign it to another variable “arabic”.

Concatenate the above variables and store them to a third variable “jap_ara”.

Update the variable “jap_ara” with a character ‘!’ and print it.

Use for loop to print characters and bytes of the variable “arabic”.

# 01-Dec-2019 Class Initial Task

Task#1

Generate a Vector of 100 random floats between 0 and 900. Find out how many of the random numbers are

between 0 to 300, 300 to 600, and above 600.

Task#2

Generate a random integer vector between 0 to 100 and keep on adding numbers to the vector until a number greater than 90 

is generated. Once the vector is generated sort it.

Task#3

Write a program that simulates a dice roll by picking a random number from 1-6 and then picking a second random number from 

1-6. Add the two values together, and display the total.

# CHAPTER # 8 HASH MAPS

TASK#1

Create a HashMap with name Team_scores and insert three teams as keys and their runs as the values.

[Pakistan: 250, New Zealand:309, England:224]

Take two 2 input of team and score

Add the team as key as team input with its runs as value as score input it might be any team from the above given 

teams or it might be some new team of your choice. But if it's from the above given teams then the runs should not be 

updated and if it's another team other the hashmap then the runs and team should add. 

(HINT entry() , or_insert() )

Print the whole hash maps and print the hash maps using for loop one by one entries

