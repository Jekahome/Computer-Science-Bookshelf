library IEEE;
use IEEE.STD_LOGIC_1164.all;

entity and8 is
  port (
    a : in std_logic_vector(7 downto 0);
    y : out std_logic);
end;
architecture synth of and8 is
begin
  y <= and a;
  
   --   y <= a(7) and a(6) and a(5) and a(4) and a(3) and a(2) and a(1) and a(0);
 
end;