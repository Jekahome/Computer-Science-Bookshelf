library IEEE;
use IEEE.STD_LOGIC_1164.all;

entity tristate is
  port (
    a  : in std_logic_vector(3 downto 0);
    en : in std_logic;
    y  : out std_logic_vector(3 downto 0)
  );
end;

architecture synth of tristate is
begin
  y <= a when en else
    "ZZZZ"; -- ????????? Z ??? 4-? ????????? ???? 
end;