library IEEE;
use IEEE.STD_LOGIC_1164.all;

entity bit_manipulation is
  port (
    c : in std_logic_vector(2 downto 0); -- 3-bit input
    d : in std_logic_vector(3 downto 0); -- 4-bit input
    y : out std_logic_vector(8 downto 0) -- 9-bit output
  );
end;

architecture synth of bit_manipulation is
begin
  -- Assemble a 9-bit output from pieces of inputs
  y <= c(2 downto 1) & d(0) & d(0) & d(0) & c(0) & "101";
  -- c(2) becomes the most significant bit ? y(8)
  -- c(1) becomes bit ? y(7)
  -- d(0) is duplicated three times in a row and occupies the places ? y(6), y(5), y(4)
  -- c(0) takes the place ? y(3)
  -- The string "101" (or three hard-coded bits '1', '0', '1')
  --- is permanently soldered to the least significant bits ? y(2), y(1), y(0)
end;