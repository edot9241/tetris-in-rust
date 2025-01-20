## misc
- create a window with bevy
## game logic
### game grid
- x, y boundaries for the playable area 
- keeping track of filled and empty squares
### game loop
- spawn tetromino
- check for active tetromino 
- check for colision
- check for input (rotation, x axis, y axis advance speed)
- advance y value of the tetromino by -1
- fill grid squares when the tetromino becomes inactive 
- clear row when full
- tally up the score
### display
- display game grid
- display next tetromino
- display score
- display high score
- display level
- display possible tetrominos
