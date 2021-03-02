A few notes:

  1. The FBP algorithm drops any states that cannot reach an accepting state.

  2. The FBP algorithm will drop transitions. It does not produce "complete"
     DFAs. I think the convention here is that any input that doesn't have a
     matching transition immediately fails. This is roughly equivalent to having a
     "bottom" state, which loops to itself for all input symbols.
