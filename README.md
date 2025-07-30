# Chess against engine

Play chess against the device.

## Development

### Chessboard component

Please notice that the chessboard component reset its state to the last "registered" position on new message

- if there was a pending drag and drop, then a new toggle orientation message (i.e) will reset the board to the last position
- if there was a pending promotion chooser, then a new toggle orientation message will reset the board to the last position

## Credits

### Chess vectors

Chess svg vectors have been downloaded from [Wikimedia Commons](https://commons.wikimedia.org/wiki/Category:SVG_chess_pieces).

### SvgRepo

Some vectors have been download from [SvgRepo](https://www.svgrepo.com)

- [swap-vert.svg](https://www.svgrepo.com/svg/432595/swap-vert)
