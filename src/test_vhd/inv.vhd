library IEEE;
use IEEE.STD_LOGIC_1164.all;

entity inv is
  port (
    a : in std_logic_vector(3 downto 0);
    y : out std_logic_vector(3 downto 0));
end;

architecture synth of inv is
begin
  y <= not a;
end;