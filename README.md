### Deadline
This work should be completed before **Friday 11th November**.

### Instructions
To pass the assignment, you must do all of the tasks. Small errors are acceptable, but the most important thing is that you attempt all the tasks. If you get stuck, then help is available in the labs.

Please note that this is individual work. You may discuss the work with other students, but it is absolutely forbidden to submit copies of other student's work as your own. Please read and consider the [Code of Honour](https://www.kth.se/csc/utbildning/hederskodex) carefully.

### Submission
- All required work must be committed to your KTH Github Repository.
- A week-8 repository will be created for you automatically and it can be found [here](https://gits-15.sys.kth.se/inda-16).
- Please refer to the Kurswiki for help. Contact your teaching assistant or course leader if you get stuck.

### Homework
Study all sections from chapter 6 from the course textbook and be prepared to answer any of the exercises.

### Github Task:
This week, you shall be submitting a full game. If you have done the homework above, you will have got most of the code from working through each section and exercise. It is important you read the code provided and understand why it was improved. Once you have completed your own game, you must extend it by implementing one or more of the features suggested in Exercise 6.41 - 6.48. Please indicate in a README.md file the theme of your game and which extensions you made.

### Extending World of Zuul
As indicated in Chapter 6, Exercise 6.3, you must customise the theme of the game to your own design, but keep the same general format and reuse the code provided.  It is strongly advised to work through the chapter as key lessons are presented throughout, and the code is given to you to illustrate the lesson.  The initial version of World of Zuul has been provided in your /src directory.

After you have finished improving the game, choose **one or more** features listed below to implement. **Please indicate via a README.md in your project which tasks you have attempted implemented**.

#### Exercise 6.41
Add some form of time limit to your game. If a certain task is not completed in a specified time, the player loses. A time limit can easily be implemented by counting the number of moves or the number of entered commands. You do not need to use real time.

#### Exercise 6.42
Implement a trapdoor somewhere (or some other form of door that you can only cross one way).

#### Exercise 6.43
Add a beamer to the game. A beamer is a device that can be charged and fired. When you charge the beamer, it memorizes the current room. When you fire the beamer, it transports you immediately back to the room it was charged in. The beamer could either be standard equipment or an item that the player can find. Of course, you need commands to charge and fire the beamer.

#### Exercise 6.44
Add locked doors to your game. The player needs to find (or otherwise obtain) a key to open a door.

#### Exercise 6.45
Add a transporter room. Whenever the player enters this room, he/she is randomly transported into one of the other rooms. Note: Coming up with a good design for this task is not trivial. It might be interesting to discuss design alternatives for this with other students. (We discuss design alternatives for this task at the end of Chapter 9. The adventurous or advanced reader may want to skip ahead and have a look.)

#### Exercise 6.46 - Challenge exercise
In the processCommand method in Game, there is a switch statement (or a sequence of if statements) to dispatch commands when a command word is recognized. This is not a very nice design, because every time we add a command, we have to add a case here. Can you improve this design? Design the classes so that handling of commands is more modular and new commands can be added more easily. Implement it. Test it.

#### Exercise 6.47
Add characters to the game. Characters are similar to items, but they can talk. They speak some text when you first meet them, and they may give you some help if you give them the right item.

#### Exercise 6.48
Add moving characters. These are like other characters, but every time the player types a command, these characters can move into an adjoining room.
