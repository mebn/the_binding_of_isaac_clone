### Deadline
This work should be completed before **Friday 6th November**.

### Instructions
For instructions on how to do and submit the assignment, please see the
[assignments section of the course instructions](https://gits-15.sys.kth.se/inda-20/course-instructions#assignments).

### Preparation
You must read and answer the questions in the OLI material:

- Read [Module 8: Class Design](https://kth.oli.cmu.edu/jcourse/webui/syllabus/module.do?context=3f7e37a2ac1f08887dc6095c0ce2fb55)
- If you have not done so, goto https://kth.oli.cmu.edu/, signup and register for the course key `dd1337-ht20`

You may wish to also read from _Objects First with Java_:
- **5th edition:** Chapter 6
- **6th edition:** Chapter 8

### GitHub Task

### Basic Input / Output

To warm up, we will use Java to read data from a file. File IO and Exceptions go
well together as there are normally a lot of problems. Instead of just our own
programs being a source of error, the format and contents of the data we read
from file can be problematic. Keep that in mind when you get errors - it may be
in the code, or it may be in the data - so check both.

#### Exercise IO.1 (use - `highscore`)

The `highscore` project contains a program that will read in a file of player
names and high scores and print them to the terminal. Read the source code and
create a valid data file called `scores.txt` that the program can process. Pay
attention to how each line is split in order to work out the correct format. The
data file should contain 5 entries or more.

#### Exercise IO.2 (use - `highscore`)

Modify the program so that it also can read the country of the player. Modify
your data file to include this extra field. Test that it still prints out all
the entries correctly. The output should now be as follows:

```
Player Alice from Sweden scored 10 points
```

#### Exercise IO.3 (use - `highscore`)

Often when we read from files, we want to build objects from the data. Modify
your program to make use of the `Player` class. You should create an empty
`ArrayList<Player>`, read the player data from file, and create Player objects
for each line of data. Add each Player object to the ArrayList.

> Assistants Note: One challenge we face is that numbers in our data file are
> read as String objects. However Player expects the score to be of type `int`.
> If we need to convert the String "10" to the int 10, we can use the following:
> Integer.parseInt("10")

After the collection of players has been created, iterate over this collection
using a for each loop and print out the high scores. Note a `toString()` method
for player exists so you can take advantage of that to create the output.

### Adventure Time

This week, you shall be submitting a full game. This will be made somewhat
easier as we shall be reusing the World of Zuul source code. It is important you
read and execute the code provided in `src/game` and understand how it
works. Then, you will create your own game scenario, design a world model, read
this model from an input file, and extend your game with some suggested
features. Finally you should ensure that your code is well documented using
JavaDoc.

We have added a main method to the `Game` class so you can run it outside of
BlueJ. To run the game, compile the all the classes and execute the Game class:

```
$ javac *.java
$ java Game
```

Or use your favorite IDE :-)

#### Exercise 6.3

Design your own game scenario. Do this away from the computer. Do not think
about implementation, classes, or even programming in general. Just think about
inventing an interesting game. The game can be anything that has as its base
structure a player moving through different locations. Here are some examples:

- You are a white blood cell traveling through the body in search of viruses to attack...
- You are lost in a shopping mall and must find the exit...
- You are a mole in its burrow and you cannot remember where you stored your food reserves before winter...
- You are an adventurer who searches through a dungeon full of monsters and other characters...
- You are from the bomb squad and must find and defuse a bomb before it goes off...

Make sure that your game has a goal (so that it has an end and the player can
"win"). Try to think of many things to make the game interesting (trap doors,
magic items, characters that help you only if you feed them, time limits...
whatever you like). Let your imagination run wild.

### Base Game
For the base game to be accepted, the following requirements must be met:

* The game theme must be different from World of Zuul
* Your game must have a _beginning_ and an _end_. That is to say, there must be
 one or more things the player can do that causes the game loop to exit.
* The game world should have a minumum of 5 rooms
* There must be a map of your game world in the [docs](docs) directory. You can
 use e.g. [draw.io](https://www.draw.io) for this, if you wish.
* You must add the following information to the
  [docs/README.md](docs/README.md) file:
    - A description of the theme of your game.
    - How to win/lose.
    - Which feature(s) from [Extending World of Zuul](#extending-world-of-zuul)
      you implemented and where they can be found in the game.

### Loading a World Model from File

#### Exercise IO.4

Having completed the earlier IO exercises, you should be able to see the
potential for storing world models for your game as a text file and creating
Room objects from these descriptions. Add a new method
`createRoomsFromFile(String filename)` that achieves this.

One complication we must deal with is the order of creating rooms and then
linking their exits. Whilst it is possible to include the room information and
exits on a single line, it creates a parsing problem, where we create a Room
object, but the corresponsing exit Rooms have not yet been created. The original
`createRooms()` can act as a guide for a simple solution, where we create the rooms
first, then assign the exits. To solve this we can prefix lines in our file and
test with a conditional as we read them. For example:

```
Room,Name1,Description1
Room,Name2,Description2
Exit,Name1,East,Name2
Exit,Name2,West,Name1
```

As long as we list all the rooms first, then all the exits, we can build and
link the world model of the game. A rough guide for the method is provided
below. Perhaps you can also spot an opportunity to optimise the number of lines
required in the file?

```java
void createRoomsFromFile(String filename) {
  // create a HashMap<String, Room> worldModel to store the game world as it is read from file

  // while there are more lines in the file, read line
  // if line starts with "Room" then extract the Name and Description and
  // create a Room instance and
  // add it to the HashMap, using Name as the key

  // else if the line starts with "Exit" assume the necessary rooms exist in the HashMap and
  // use the setExit() method to link Rooms together
  // hint: this requires us to acces the HashMap twice like:
  // worldModel.get("Name1").setExit("East", worldmodel.get("Name2"));

  // remember to set the currentRoom to the starting room as the final step
  // you can chose to hardcode this room for convenience or
  // modify the logic above to assume the first room is the starting point
}
```

Remember to include a sample world model file in the repo.

> Assistants Note: One curious thing to think about is how if we declare the
> worldModel as a local variable in this method, we do not expect it to persist
> for the rest of the game, as it cannot be accessed outside of this scope. So
> if it gets garbage collected, how do our rooms still exist? The same thing
> applied to the original version as all the Room variables were local to the
> createRooms method...

### Extending World of Zuul
After you have finished improving your game, choose **one or more** features
listed below to implement.

#### Option 1 - Exercise 6.41 (8.41)
Add some form of time limit to your game. If a certain task is not completed in
a specified time, the player loses. A time limit can easily be implemented by
counting the number of moves or the number of entered commands. You do not need
to use real time.

#### Option 2 - Exercise 6.42 (8.42)
Implement a trapdoor somewhere (or some other form of door that you can only
cross one way).

#### Option 3 - Exercise 6.43 (8.43)
Add a beamer to the game. A beamer is a device that can be charged and fired.
When you charge the beamer, it memorizes the current room. When you fire the
beamer, it transports you immediately back to the room it was charged in. The
beamer could either be standard equipment or an item that the player can find.
Of course, you need commands to charge and fire the beamer.

#### Option 4 - Exercise 6.44 (8.44)
Add locked doors to your game. The player needs to find (or otherwise obtain) a
key to open a door.

#### Option 5 - Exercise 6.45 (8.45)
Add a transporter room. Whenever the player enters this room, he/she is
randomly transported into one of the other rooms. Note: Coming up with a good
design for this task is not trivial. It might be interesting to discuss design
alternatives for this with other students. (We discuss design alternatives for
this task at the end of Chapter 9. The adventurous or advanced reader may want
to skip ahead and have a look.)

#### Option 6 - Exercise 6.46 (8.46)
In the processCommand method in Game, there is a switch statement (or a
sequence of if statements) to dispatch commands when a command word is
recognized. This is not a very nice design, because every time we add a
command, we have to add a case here. Can you improve this design? Design the
classes so that handling of commands is more modular and new commands can be
added more easily. Implement it. Test it.

#### Option 7 - Exercise 6.47 (8.47)
Add characters to the game. Characters are similar to items, but they can talk.
They speak some text when you first meet them, and they may give you some help
if you give them the right item.

#### Option 8 - Exercise 6.48 (8.48)
Add moving characters. These are like other characters, but every time the
player types a command, these characters can move into an adjoining room.

### Documenting your code

#### Exercise 6.XX

Using Javadoc, write the class documentation for __all__  of your classes.
First, briefly review the **Format of a Doc Comment** and **Example of Doc
Comments** sections from the [official
documentation](http://www.oracle.com/technetwork/java/javase/documentation/index-137868.html)
on Javadoc from Oracle. Then go through your classes and add Javadoc according
to the requirements below.

**The documentation of a class should at least include:**
* a comment describing the overall purpose and characteristics of the class
* a version number (use `@version`)
* the author’s name (or authors’ names) (use `@author`)
* documentation for each `public` constructor and method. Methods/constructors
  with other visibility (`protected`, `private`, `package private`) are in
  general only Javadoced if they are complex and require an explanation, or
  part of some larger machinery that is not obvious.

**The documentation for each constructor and method should include:**
* a description of the purpose and function of the method
* name and description of each parameter (use `@param`)
* a description of the value returned (use `@return`). Note that this is not
  applicable to constructors and `void` methods.
* getters and setters are in general trivial, but the field they correspond to
  may not be. it is reasonable to describe the purpose of the field, rather
  than what the method does (because it should in most cases be magnificently
  obvious).
* **Note:** The types of parameters and return values should **not** be written
  in the Javadoc, as these are already in the method/constructor header!

**For examples of good Javadoc, see the files you have been provided in
the [src](src) directory**

Good Javadoc will become a **minimum requirement** in documentation of future
assignments where you have created your own class, so absolute care must be
taken to understand correct style of documentation.  Otherwise, you may be
asked to **resubmit work if the documentation is of a poor standard**.

### Grading Criteria

Each week we will communicate grading criteria through the [issue
tracker](../../issues/). Grading criteria set the basic standards for a pass,
komp or fail, so it is essential you review them each week. These will change
over time as your skills develop, so make sure you read the grading criteria
issue carefully and tick off all the requirements.
