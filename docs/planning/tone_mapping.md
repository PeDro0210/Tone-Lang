# Tone Mapping


## Basic Details.

As far i've been looking at it, a tone is a bit of a complex structure, but will take three main "fields" for doing a mapping.

- Depending on the numbers thrown by this to fields will do a map to an instruction.
  - Hz
  - dB

- Duration: Will start taking a 2 sec span for each instruction mapping.

## Overall Mapping

The basis for the language will be pretty similar to Brainfuck, having:

- A dynamic size non-circular strip with 1 byte for each cell.

- A single pointer looking at the strip.


with basic instructions as:

- Increment the pointer position.
- Decrement the pointer position.
- Increment on the cell being pointed at.
- Decrement on the cell being pointed at.
- Jumps between sections (similar on how the '\[' and '\]' operators work).
- Showing the the cell value on the stdout.


