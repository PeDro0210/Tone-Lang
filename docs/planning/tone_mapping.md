# Tone Mapping

## Basic Details.

As far I've been looking at it, a tone is a bit of a complex structure, but will take three main "fields" for doing a mapping.

- Depending on the numbers thrown by this to fields will do a map to an instruction.
  - Hz
- Duration: Will start taking a 2 sec span for each instruction mapping.

## Overall Mapping

The basis for the language will be pretty similar to Brainfuck, having:

- A dynamic size non-circular strip with 1 byte for each cell.

- A single pointer looking at the strip.


With basic instructions as:

- Increment the pointer position.
- Decrement the pointer position.
- Increment on the cell being pointed at.
- Decrement on the cell being pointed at.
- Jumps between sections (similar on how the '\[' and '\]' operators work).
- Showing the the cell value on the stdout.


Mapping for each note:

- C4 -> Increment Pointer (~261 Hz)
- D4 -> Decrement Pointer (~294 Hz)
- E4 -> Increment Cell (~329 Hz)
- F4 -> Decrement Cell (~349 Hz)
- G4 -> Jump bracket \[ (~392 Hz)
- A4 -> Jump bracket \] (~440 Hz)
- B4 -> Showing value in ascii 256 (~493 Hz)

