library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std_unsigned.all;

entity subtractor is
  port (
    A : in    std_logic_vector(7 downto 0);
    B : in    std_logic_vector(7 downto 0);
    Y : out   std_logic_vector(7 downto 0)
  );
end entity subtractor;

architecture synth of subtractor is
begin
  Y <= A - B;
end architecture synth;