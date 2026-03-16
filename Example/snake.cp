.(CP Snake Full Version)

# ---- INITIALIZE ----

A              # Head X =1
R A            # Head Y =1
R R A A A      # Food X =3
R A A          # Food Y =2
R A            # Direction = Right
R              # Score
R A A          # Length =2
R A            # GameFlag =1

# Store initial body
,[2,0]
,[3,1]

{[7]B}

:(4)          # Take direction input

# ---- MOVE ENGINE ----

# Shift body backwards
,[2,6]
,[3,6]

# Direction Logic
IC == 1_J(10)   # Right
A

IC == 2_J(11)   # Left
S

IC == 3_J(12)   # Up
R S L

IC == 4_J(13)   # Down
R A L

# ---- COLLISION CHECK ----

# Wall (0 or 15)
IC == 0_J(20)
IC == 15_J(20)

# ---- FOOD CHECK ----

,[1,0]
,[1,2]
IC == 0_J(30)

# Eat Food
R R R R R A     # Score++
R A              # Length++
.(FoodRespawn)

; (5)

{E}

.(Game Over)