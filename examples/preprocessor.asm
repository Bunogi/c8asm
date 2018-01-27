;;; Preprocessor test file.
  #define player_x v0
  #define player_y v1
  #define reminder ;;

  ld $player_x, 0
  ld $player_y, 0

  $reminder This line should be commented out.
