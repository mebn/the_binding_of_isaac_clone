### Deadline
This work should be completed before **Friday 8th November**.

### Instructions
For instructions on how to do and submit the assignment, please see the
[assignments section of the course instructions](https://gits-15.sys.kth.se/inda-19/course-instructions#assignments).

### Homework

* **5th ed:** Study all of chapter 6, and 5.10.
* **6th ed:** Study all of chapter 8, and 6.11.

### Github Task:
This week, you shall be submitting a full game. If you have done the homework
above, you will have got most of the code from working through each section and
exercise. It is important you read the code provided and understand why it was
improved. A few requirements on your game are listed in the [Base Game](#base-game)
section. Once you have completed your own game, you must extend it by
implementing two or more of the features listed in the exercises under
[Extending World of Zuul](#extending-world-of-zuul).

Additionally, you will properly document your code with Javadoc style comments,
according to the instructions in [Documenting your code](#documenting-your-code).

### Base Game
For the base game to be accepted, the following requirements must be met:

* There must be a map of your game world in the [docs](docs) directory. You can
  use e.g. [draw.io](https://www.draw.io) for this, if you wish.
* Your game must have a _beginning_ and an _end_. That is to say, there must be
  one or more things the player can do that causes the game loop to exit.
* You must add the following information to the
  [docs/README.md](docs/README.md) file:
    - A description of the theme of your game.
    - How to win/lose.
    - Which of the features from [Extending World of Zuul](#extending-world-of-zuul)
      you implemented and where they can be found in the game.

### Extending World of Zuul
As indicated in Chapter 6 (8), Exercise 6.3 (8.3), you must customise the theme
of the game to your own design, but keep the same general format and reuse the
code provided. It is strongly advised to work through the chapter as key
lessons are presented throughout, and the code is given to you to illustrate
the lesson. The initial version of World of Zuul has been provided in your
[src](src) directory.

After you have finished improving the game, choose **two or more** features
listed below to implement.

#### Exercise 6.41 (8.41)
Add some form of time limit to your game. If a certain task is not completed in
a specified time, the player loses. A time limit can easily be implemented by
counting the number of moves or the number of entered commands. You do not need
to use real time.

#### Exercise 6.42 (8.42)
Implement a trapdoor somewhere (or some other form of door that you can only
cross one way).

#### Exercise 6.43 (8.43)
Add a beamer to the game. A beamer is a device that can be charged and fired.
When you charge the beamer, it memorizes the current room. When you fire the
beamer, it transports you immediately back to the room it was charged in. The
beamer could either be standard equipment or an item that the player can find.
Of course, you need commands to charge and fire the beamer.

#### Exercise 6.44 (8.44)
Add locked doors to your game. The player needs to find (or otherwise obtain) a
key to open a door.

#### Exercise 6.45 (8.45)
Add a transporter room. Whenever the player enters this room, he/she is
randomly transported into one of the other rooms. Note: Coming up with a good
design for this task is not trivial. It might be interesting to discuss design
alternatives for this with other students. (We discuss design alternatives for
this task at the end of Chapter 9. The adventurous or advanced reader may want
to skip ahead and have a look.)

#### Exercise 6.46 (8.46)
In the processCommand method in Game, there is a switch statement (or a
sequence of if statements) to dispatch commands when a command word is
recognized. This is not a very nice design, because every time we add a
command, we have to add a case here. Can you improve this design? Design the
classes so that handling of commands is more modular and new commands can be
added more easily. Implement it. Test it.

#### Exercise 6.47 (8.47)
Add characters to the game. Characters are similar to items, but they can talk.
They speak some text when you first meet them, and they may give you some help
if you give them the right item.

#### Exercise 6.48 (8.48)
Add moving characters. These are like other characters, but every time the
player types a command, these characters can move into an adjoining room.

### Documenting your code

#### Exercise 6.XX
Using Javadoc, write the class documentation for __all__  of your classes. First,
briefly review the **Format of a Doc Comment** and **Example of Doc Comments**
sections from the [official documentation](http://www.oracle.com/technetwork/java/javase/documentation/index-137868.html)
on Javadoc from Oracle and read 5.10 (6.11) in the book. Then go through your
classes and add Javadoc according to the requirements below.

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
Each week we will communicate grading criteria through the [issue tracker](../../issues/). Grading criteria set the basic standards for a pass, komp or fail, so it is essential you review them each week. These will change over time as your skills develop, so make sure you read the grading criteria issue carefully and tick off all the requirements.
