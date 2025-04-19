# Definition of the First Assignment

The first assignment consists of developing a **question-and-answer game** based on the content covered so far in the **Computer Networks** course. The project must follow a **client/server architecture** and use **sockets** for communication.

Each group must define the game rules, such as:

* setting the number of participants;
* awarding points as players answer correctly;
* including different levels of difficulty;
* following a path or progression;
* and other custom features.

At the end of the game, the **network connection must be closed**, allowing the user to return to the main menu and play again.

The game must be implemented using **UDP**, and if a response is not received, the server should **resend** the message. To test this functionality, you must **simulate packet loss** using *netem* on Linux or *clumsy* on Windows.

Each group must define the game rules in a **document to be submitted on the presentation day**. This document must also describe **how the client-server communication works** (including the types of messages used). Additionally, it should explain **how the game was implemented**.

Be creative with your game and make sure it works correctly, **without bugs**! Also, ensure **good user experience**, providing feedback and instructions whenever necessary.

## Results and Submission

**Groups**: up to 3 members.

**Deadline and Presentations**: May 18

**Final Submission**:

* A **written document (minimum of 4 pages)** describing:
  * the rules and flow of the game;
  * how the game was implemented;
  * data structures used;
  * threads and main classes;
  * synchronization mechanisms;
  * the message format used in communication, etc.
* **Commented source code**

**Presentation**:

* The presentation will be conducted via **Zoom**
* **All group members** must be present and ready to answer questions

**IMPORTANT:**
Late submissions **will not be accepted**.
Projects that **do not compile or run** will **not be evaluated**.
All submissions will be **reviewed and compared**.
If **plagiarism** is detected, **all involved projects will receive a grade of ZERO.**
